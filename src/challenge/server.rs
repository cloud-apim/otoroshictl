//! HTTP proxy server for the Otoroshi Challenge Proxy.

use futures_util::Future;
use http::Response;
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::service::Service;
use hyper::{Body, Client, Request, Server};
use serde::Serialize;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use crate::challenge::config::{ProtocolVersion, ProxyConfig};
use crate::cli_stderr_printline;
use crate::cli_stdout_printline;
use crate::http_utils::is_hop_by_hop_header;
use crate::otoroshi::protocol::OtoroshiProtocol;

/// JSON structure for error responses.
#[derive(Serialize)]
struct ErrorResponse<'a> {
    error: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<&'a str>,
}

/// Build a JSON error response.
fn json_error_response(
    status: http::StatusCode,
    error: &str,
    details: Option<&str>,
) -> Response<Body> {
    let body = ErrorResponse { error, details };
    let json = serde_json::to_string(&body).unwrap_or_else(|e| {
        error!("Failed to serialize error response: {}", e);
        r#"{"error":"Internal error"}"#.to_string()
    });

    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .expect("Failed to build error response")
}

/// HTTP client for forwarding requests to the backend.
#[derive(Clone, Debug)]
struct ProxyClient {
    config: Arc<ProxyConfig>,
    http_client: Client<HttpConnector>,
}

impl ProxyClient {
    fn new(config: Arc<ProxyConfig>) -> Self {
        let http_client = Client::new();
        ProxyClient {
            config,
            http_client,
        }
    }

    fn forward_request(
        &self,
        mut req: Request<Body>,
    ) -> Result<ResponseFuture, http::uri::InvalidUri> {
        let uri_string = format!(
            "{}{}",
            self.config.backend_url,
            req.uri()
                .path_and_query()
                .map(|pq| pq.as_str())
                .unwrap_or("/")
        );
        let uri = uri_string.parse()?;
        *req.uri_mut() = uri;
        Ok(self.http_client.request(req))
    }
}

/// Service handling individual requests.
struct ProxySvc {
    client: ProxyClient,
    config: Arc<ProxyConfig>,
}

impl Service<Request<Body>> for ProxySvc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let config = self.config.clone();
        let client = self.client.clone();

        Box::pin(async move {
            // Extract the state header value
            let state_value = req
                .headers()
                .get(&config.state_header)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            // Process based on protocol version
            let response_value = match config.version {
                ProtocolVersion::V1 => {
                    // V1: Simple echo of the state value
                    match state_value {
                        Some(value) => Some(value),
                        None => {
                            return Ok(json_error_response(
                                http::StatusCode::UNAUTHORIZED,
                                "Missing Otoroshi-State header",
                                None,
                            ));
                        }
                    }
                }
                ProtocolVersion::V2 => {
                    // V2: JWT challenge/response
                    let secret = match &config.secret {
                        Some(s) => s,
                        None => {
                            return Ok(json_error_response(
                                http::StatusCode::INTERNAL_SERVER_ERROR,
                                "Secret is required for V2 protocol",
                                None,
                            ));
                        }
                    };

                    match state_value {
                        Some(token) => {
                            let protocol = OtoroshiProtocol::new_with_ttl(
                                secret,
                                config.algorithm,
                                config.token_ttl,
                            );
                            match protocol.process_v2(&token) {
                                Ok(resp_token) => Some(resp_token),
                                Err(e) => {
                                    return Ok(json_error_response(
                                        http::StatusCode::UNAUTHORIZED,
                                        "Invalid Otoroshi challenge",
                                        Some(&e.to_string()),
                                    ));
                                }
                            }
                        }
                        None => {
                            return Ok(json_error_response(
                                http::StatusCode::UNAUTHORIZED,
                                "Missing Otoroshi-State header",
                                None,
                            ));
                        }
                    }
                }
            };

            // Build the backend request, filtering hop-by-hop headers
            let (parts, body) = req.into_parts();
            let mut backend_req_builder = Request::builder().method(parts.method).uri(parts.uri);

            for (name, value) in parts.headers.iter() {
                if !is_hop_by_hop_header(name) {
                    backend_req_builder = backend_req_builder.header(name, value);
                }
            }

            let backend_req = match backend_req_builder.body(body) {
                Ok(req) => req,
                Err(e) => {
                    return Ok(json_error_response(
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to build request",
                        Some(&e.to_string()),
                    ));
                }
            };

            // Forward the request with timeout
            let backend_future = match client.forward_request(backend_req) {
                Ok(future) => future,
                Err(e) => {
                    return Ok(json_error_response(
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Invalid backend URI",
                        Some(&e.to_string()),
                    ));
                }
            };
            let result = tokio::time::timeout(config.request_timeout, backend_future).await;

            match result {
                Ok(Ok(backend_resp)) => {
                    let (mut resp_parts, resp_body) = backend_resp.into_parts();

                    // Add the response header
                    if let Some(value) = response_value
                        && let Ok(header_value) = value.parse()
                    {
                        resp_parts
                            .headers
                            .insert(config.state_resp_header.clone(), header_value);
                    }

                    Ok(Response::from_parts(resp_parts, resp_body))
                }
                Ok(Err(e)) => Ok(json_error_response(
                    http::StatusCode::BAD_GATEWAY,
                    "Backend unavailable",
                    Some(&e.to_string()),
                )),
                Err(_) => Ok(json_error_response(
                    http::StatusCode::GATEWAY_TIMEOUT,
                    "Backend request timed out",
                    None,
                )),
            }
        })
    }
}

/// Service factory for creating ProxySvc instances.
struct MakeSvc {
    client: ProxyClient,
    config: Arc<ProxyConfig>,
}

impl<T> Service<T> for MakeSvc {
    type Response = ProxySvc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let client = self.client.clone();
        let config = self.config.clone();
        Box::pin(async move { Ok(ProxySvc { client, config }) })
    }
}

/// Run the proxy server.
#[allow(clippy::too_many_arguments)]
pub async fn run(
    port: u16,
    backend_host: String,
    backend_port: u16,
    secret: Option<String>,
    secret_base64: bool,
    state_header: String,
    state_resp_header: String,
    timeout: u64,
    token_ttl: i64,
    alg: String,
    use_v1: bool,
) {
    // Validate that secret is provided for V2
    if !use_v1 && secret.is_none() {
        cli_stderr_printline!(
            "Error: --secret is required for V2 protocol (or use --v1 for simple echo mode)"
        );
        std::process::exit(1);
    }

    let config = match ProxyConfig::new(
        port,
        backend_host,
        backend_port,
        secret,
        secret_base64,
        state_header,
        state_resp_header,
        timeout,
        token_ttl,
        alg,
        use_v1,
    ) {
        Ok(config) => Arc::new(config),
        Err(e) => {
            cli_stderr_printline!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };

    let addr: SocketAddr = config.listen_addr;
    let client = ProxyClient::new(config.clone());
    let make_svc = MakeSvc {
        client,
        config: config.clone(),
    };

    let version_str = if use_v1 { "V1 (echo)" } else { "V2 (JWT)" };
    cli_stdout_printline!(
        "Otoroshi {} Challenge Proxy listening on http://{}",
        version_str,
        addr
    );
    cli_stdout_printline!("Forwarding requests to {}", config.backend_url);

    let server = Server::bind(&addr).serve(make_svc);

    // Handle graceful shutdown
    let graceful = server.with_graceful_shutdown(async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        cli_stdout_printline!("Shutdown signal received, stopping server...");
    });

    if let Err(e) = graceful.await {
        cli_stderr_printline!("Server error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::body::to_bytes;

    /// Helper to extract body as string from response
    async fn body_to_string(response: Response<Body>) -> String {
        let bytes = to_bytes(response.into_body()).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn test_json_error_response_without_details() {
        let response = json_error_response(http::StatusCode::BAD_REQUEST, "Test error", None);
        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

        let body = body_to_string(response).await;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(json["error"], "Test error");
        assert!(json.get("details").is_none());
    }

    #[tokio::test]
    async fn test_json_error_response_with_details() {
        let response = json_error_response(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "Error message",
            Some("Detailed info"),
        );
        assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

        let body = body_to_string(response).await;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(json["error"], "Error message");
        assert_eq!(json["details"], "Detailed info");
    }

    #[test]
    fn test_json_error_response_content_type() {
        let response = json_error_response(http::StatusCode::OK, "test", None);
        let content_type = response.headers().get("content-type").unwrap();
        assert_eq!(content_type, "application/json");
    }

    #[tokio::test]
    async fn test_json_error_response_is_valid_json() {
        let response = json_error_response(
            http::StatusCode::UNAUTHORIZED,
            "Unauthorized",
            Some("Token expired"),
        );

        let body = body_to_string(response).await;
        let result: Result<serde_json::Value, _> = serde_json::from_str(&body);
        assert!(result.is_ok(), "Response body should be valid JSON");
    }

    #[tokio::test]
    async fn test_json_error_response_special_characters() {
        let response = json_error_response(
            http::StatusCode::BAD_REQUEST,
            "Error with \"quotes\" and \\backslash",
            Some("Details with <html> & special chars"),
        );

        let body = body_to_string(response).await;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(json["error"], "Error with \"quotes\" and \\backslash");
        assert_eq!(json["details"], "Details with <html> & special chars");
    }

    #[tokio::test]
    async fn test_json_error_response_gateway_timeout() {
        let response = json_error_response(
            http::StatusCode::GATEWAY_TIMEOUT,
            "Backend request timed out",
            None,
        );

        assert_eq!(response.status(), http::StatusCode::GATEWAY_TIMEOUT);

        let body = body_to_string(response).await;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(json["error"], "Backend request timed out");
        assert!(json.get("details").is_none());
    }

    #[tokio::test]
    async fn test_json_error_response_bad_gateway() {
        let response = json_error_response(
            http::StatusCode::BAD_GATEWAY,
            "Backend unavailable",
            Some("Connection refused"),
        );

        assert_eq!(response.status(), http::StatusCode::BAD_GATEWAY);

        let body = body_to_string(response).await;
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        assert_eq!(json["error"], "Backend unavailable");
        assert_eq!(json["details"], "Connection refused");
    }
}
