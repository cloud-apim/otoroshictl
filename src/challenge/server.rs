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

use crate::challenge::config::{ProtocolVersion, ProxyConfig, clever_cloud_health_check_paths};
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
            // Presence of the state header is checked independently of its value: a header
            // that is present but not valid UTF-8 must be rejected, never bypassed.
            let state_header_present = req.headers().contains_key(&config.state_header);

            // Extract the state header value
            let state_value = req
                .headers()
                .get(&config.state_header)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            // Excluded paths (health checks) are forwarded without challenge verification when
            // no state header is present (external probe). A request carrying a state header is
            // processed normally so Otoroshi itself can still call these paths.
            let bypass_challenge =
                !state_header_present && config.is_excluded(req.method(), req.uri().path());
            if bypass_challenge {
                debug!(
                    "Bypassing Otoroshi challenge for excluded path {} {} (no state header)",
                    req.method(),
                    req.uri().path()
                );
            }

            // Process based on protocol version
            let response_value = if bypass_challenge {
                None
            } else {
                match config.version {
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
                        // Determine the verification key (public key for asymmetric, secret for HMAC)
                        let verify_key = if config.algorithm.is_asymmetric() {
                            match &config.public_key {
                                Some(pk) => pk.as_slice(),
                                None => {
                                    return Ok(json_error_response(
                                        http::StatusCode::INTERNAL_SERVER_ERROR,
                                        "Public key is required for asymmetric V2 protocol verification",
                                        None,
                                    ));
                                }
                            }
                        } else {
                            match &config.secret {
                                Some(s) => s.as_slice(),
                                None => {
                                    return Ok(json_error_response(
                                        http::StatusCode::INTERNAL_SERVER_ERROR,
                                        "Secret is required for V2 protocol",
                                        None,
                                    ));
                                }
                            }
                        };

                        // Determine the signing key (private key for asymmetric, secret for HMAC)
                        // Use response_secret if provided, otherwise fall back to secret
                        let sign_key = if config.response_algorithm.is_asymmetric() {
                            match config.response_secret.as_ref().or(config.secret.as_ref()) {
                                Some(sk) => sk.as_slice(),
                                None => {
                                    return Ok(json_error_response(
                                        http::StatusCode::INTERNAL_SERVER_ERROR,
                                        "Private key (--response-secret or --secret) is required for asymmetric response signing",
                                        None,
                                    ));
                                }
                            }
                        } else {
                            match config.response_secret.as_ref().or(config.secret.as_ref()) {
                                Some(s) => s.as_slice(),
                                None => {
                                    return Ok(json_error_response(
                                        http::StatusCode::INTERNAL_SERVER_ERROR,
                                        "Secret (--response-secret or --secret) is required for response signing",
                                        None,
                                    ));
                                }
                            }
                        };

                        match state_value {
                            Some(token) => {
                                let protocol = OtoroshiProtocol::new_asymmetric_with_ttl(
                                    verify_key,
                                    config.algorithm,
                                    sign_key,
                                    config.response_algorithm,
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
                }
            };

            // Process Consumer Info JWT if configured (skipped for excluded paths)
            let consumer_info_decoded: Option<String> = if bypass_challenge {
                None
            } else if let Some(ci_config) = &config.consumer_info {
                let token_opt = req
                    .headers()
                    .get(&ci_config.in_header)
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string());

                match token_opt {
                    None => {
                        if ci_config.strict {
                            return Ok(json_error_response(
                                http::StatusCode::UNAUTHORIZED,
                                "Missing Consumer Info header",
                                None,
                            ));
                        }
                        None
                    }
                    Some(token) => match ci_config.verifier.verify_and_decode(&token) {
                        Ok(claims) => match serde_json::to_string(&claims) {
                            Ok(json) => Some(json),
                            Err(e) => {
                                warn!("Failed to serialize Consumer Info claims: {}", e);
                                None
                            }
                        },
                        Err(e) => {
                            if ci_config.strict {
                                return Ok(json_error_response(
                                    http::StatusCode::UNAUTHORIZED,
                                    "Invalid Consumer Info token",
                                    None,
                                ));
                            }
                            warn!("Consumer Info token verification failed: {}", e);
                            None
                        }
                    },
                }
            } else {
                None
            };

            // Build the backend request, filtering hop-by-hop headers
            let (parts, body) = req.into_parts();
            let mut backend_req_builder = Request::builder().method(parts.method).uri(parts.uri);

            for (name, value) in parts.headers.iter() {
                if is_hop_by_hop_header(name) {
                    continue;
                }
                // Strip the Otoroshi state-challenge header if requested.
                if config.strip_otoroshi_headers && name == config.state_header {
                    continue;
                }
                // Consumer info header handling.
                if let Some(ci_config) = &config.consumer_info {
                    // The decoded output header is produced by the proxy only: never forward a
                    // client-supplied one (it would be an unverified identity claim).
                    if ci_config.out_header != ci_config.in_header && name == ci_config.out_header {
                        continue;
                    }
                    let skip = if config.strip_otoroshi_headers {
                        // Strip mode: always remove the raw JWT — the decoded JSON
                        // is added separately after the loop if available.
                        name == ci_config.in_header
                    } else {
                        // Normal mode: only remove the header when it is being
                        // replaced in-place by the decoded JSON (same header name).
                        ci_config.in_header == ci_config.out_header
                            && consumer_info_decoded.is_some()
                            && name == ci_config.in_header
                    };
                    if skip {
                        continue;
                    }
                }
                backend_req_builder = backend_req_builder.header(name, value);
            }

            // Add the decoded Consumer Info JSON as a header if available
            if let Some(ci_config) = &config.consumer_info {
                if let Some(ref json) = consumer_info_decoded {
                    match json.parse::<http::header::HeaderValue>() {
                        Ok(header_value) => {
                            backend_req_builder = backend_req_builder
                                .header(ci_config.out_header.clone(), header_value);
                        }
                        Err(e) => {
                            warn!("Failed to set Consumer Info output header: {}", e);
                        }
                    }
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

                    // Add the response header. On bypass, make sure no state response header
                    // coming from the backend leaks through: the proxy is the only source of it.
                    match response_value {
                        Some(value) => {
                            if let Ok(header_value) = value.parse() {
                                resp_parts
                                    .headers
                                    .insert(config.state_resp_header.clone(), header_value);
                            }
                        }
                        None => {
                            resp_parts.headers.remove(&config.state_resp_header);
                        }
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
    public_key: Option<String>,
    response_secret: Option<String>,
    response_secret_base64: bool,
    response_alg: Option<String>,
    use_v1: bool,
    consumer_info_enabled: bool,
    consumer_info_header: String,
    consumer_info_out_header: Option<String>,
    consumer_info_alg: String,
    consumer_info_secret: Option<String>,
    consumer_info_secret_base64: bool,
    consumer_info_public_key: Option<String>,
    consumer_info_strict: bool,
    strip_otoroshi_headers: bool,
    exclude_paths: Vec<String>,
) {
    // Validate that secret or public_key is provided for V2
    if !use_v1 && secret.is_none() && public_key.is_none() {
        cli_stderr_printline!(
            "Error: --secret or --public-key is required for V2 protocol (or use --v1 for simple echo mode)"
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
        public_key,
        response_secret,
        response_secret_base64,
        response_alg,
        use_v1,
        consumer_info_enabled,
        consumer_info_header,
        consumer_info_out_header,
        consumer_info_alg,
        consumer_info_secret,
        consumer_info_secret_base64,
        consumer_info_public_key,
        consumer_info_strict,
        strip_otoroshi_headers,
    ) {
        Ok(config) => config,
        Err(e) => {
            cli_stderr_printline!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };

    // Excluded paths: explicit --exclude-path values plus Clever Cloud health check paths
    let cc_paths = clever_cloud_health_check_paths();
    let config = Arc::new(config.with_excluded_paths(exclude_paths.iter().chain(cc_paths.iter())));

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
    if !config.excluded_paths.is_empty() {
        let origin = if cc_paths.is_empty() {
            ""
        } else if exclude_paths.is_empty() {
            " (from Clever Cloud health check variables)"
        } else {
            " (from --exclude-path and Clever Cloud health check variables)"
        };
        cli_stdout_printline!(
            "Challenge bypassed for GET/HEAD on: {}{}",
            config.excluded_paths.join(", "),
            origin
        );
    }

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
    // -------------------------------------------------------------------------
    // Excluded paths (health checks) behaviour with a real backend
    // -------------------------------------------------------------------------

    use hyper::service::{make_service_fn, service_fn};
    use std::convert::Infallible;

    /// Spawn a tiny backend that answers 200 with the request path as body.
    /// It also sets a forged `Otoroshi-State-Resp` header: the proxy must never let it through
    /// on bypassed requests, and must overwrite it when it answers a challenge.
    async fn spawn_backend() -> u16 {
        let make = make_service_fn(|_| async {
            Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
                let body = format!("backend:{}:{}", req.method(), req.uri().path());
                let resp = Response::builder()
                    .header("Otoroshi-State-Resp", "forged-by-backend")
                    .body(Body::from(body))
                    .unwrap();
                Ok::<_, Infallible>(resp)
            }))
        });
        let server = Server::bind(&SocketAddr::from(([127, 0, 0, 1], 0))).serve(make);
        let port = server.local_addr().port();
        tokio::spawn(server);
        port
    }

    struct TestConfigOpts {
        use_v1: bool,
        state_header: &'static str,
        consumer_info: bool,
        consumer_info_out_header: Option<&'static str>,
    }

    impl Default for TestConfigOpts {
        fn default() -> Self {
            TestConfigOpts {
                use_v1: false,
                state_header: "Otoroshi-State",
                consumer_info: false,
                consumer_info_out_header: None,
            }
        }
    }

    fn test_config_with(
        backend_port: u16,
        excluded: &[&str],
        opts: TestConfigOpts,
    ) -> Arc<ProxyConfig> {
        let config = ProxyConfig::new(
            8080,
            "127.0.0.1".to_string(),
            backend_port,
            Some("test-secret".to_string()),
            false,
            opts.state_header.to_string(),
            "Otoroshi-State-Resp".to_string(),
            5,
            30,
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            opts.use_v1,
            opts.consumer_info,
            "Otoroshi-Claims".to_string(),
            opts.consumer_info_out_header.map(|s| s.to_string()),
            "HS512".to_string(),
            Some("ci-secret".to_string()),
            false,
            None,
            true, // consumer info strict
            true,
        )
        .unwrap()
        .with_excluded_paths(excluded.iter().copied());
        Arc::new(config)
    }

    fn test_config(backend_port: u16, excluded: &[&str]) -> Arc<ProxyConfig> {
        test_config_with(backend_port, excluded, TestConfigOpts::default())
    }

    fn svc(config: Arc<ProxyConfig>) -> ProxySvc {
        ProxySvc {
            client: ProxyClient::new(config.clone()),
            config,
        }
    }

    fn request(method: &str, uri: &str) -> Request<Body> {
        Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .unwrap()
    }

    #[tokio::test]
    async fn test_excluded_path_bypasses_challenge() {
        let backend_port = spawn_backend().await;
        let config = test_config(backend_port, &["/health"]);
        let mut service = svc(config);

        let response = service.call(request("GET", "/health")).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
        assert!(
            response.headers().get("otoroshi-state-resp").is_none(),
            "no state response header must be present on bypassed requests, even from the backend"
        );
        assert_eq!(body_to_string(response).await, "backend:GET:/health");
    }

    #[tokio::test]
    async fn test_v1_excluded_path_bypass_and_echo() {
        let backend_port = spawn_backend().await;
        let opts = TestConfigOpts {
            use_v1: true,
            ..Default::default()
        };
        let mut service = svc(test_config_with(backend_port, &["/health"], opts));

        // No header: bypass
        let response = service.call(request("GET", "/health")).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
        assert!(response.headers().get("otoroshi-state-resp").is_none());

        // Header present: V1 echo still applies on the excluded path
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .header("Otoroshi-State", "abc")
            .body(Body::empty())
            .unwrap();
        let response = service.call(req).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(
            response.headers().get("otoroshi-state-resp").unwrap(),
            "abc"
        );

        // Non excluded path without header: 401
        let response = service.call(request("GET", "/api")).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_custom_state_header_name_drives_the_bypass() {
        let backend_port = spawn_backend().await;
        let opts = TestConfigOpts {
            state_header: "X-My-State",
            ..Default::default()
        };
        let mut service = svc(test_config_with(backend_port, &["/health"], opts));

        // The default header name is irrelevant: still no custom header, so bypass
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .header("Otoroshi-State", "ignored")
            .body(Body::empty())
            .unwrap();
        let response = service.call(req).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);

        // The custom header is present: normal processing, invalid token rejected
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .header("X-My-State", "not-a-jwt")
            .body(Body::empty())
            .unwrap();
        let response = service.call(req).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_consumer_info_strict_is_skipped_on_bypass_only() {
        let backend_port = spawn_backend().await;
        let opts = TestConfigOpts {
            consumer_info: true,
            ..Default::default()
        };
        let mut service = svc(test_config_with(backend_port, &["/health"], opts));

        // Bypassed request: strict consumer info does not apply
        let response = service.call(request("GET", "/health")).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);

        // Non excluded path: missing state header is still the first rejection
        let response = service.call(request("GET", "/api")).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
        let json: serde_json::Value =
            serde_json::from_str(&body_to_string(response).await).unwrap();
        assert_eq!(json["error"], "Missing Otoroshi-State header");
    }

    #[tokio::test]
    async fn test_consumer_info_strict_applies_on_excluded_path_with_state_header() {
        let backend_port = spawn_backend().await;
        let opts = TestConfigOpts {
            use_v1: true,
            consumer_info: true,
            ..Default::default()
        };
        let mut service = svc(test_config_with(backend_port, &["/health"], opts));

        // V1 state header present on the excluded path: no bypass, so the strict Consumer Info
        // check is the one rejecting the request.
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .header("Otoroshi-State", "abc")
            .body(Body::empty())
            .unwrap();
        let response = service.call(req).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
        let json: serde_json::Value =
            serde_json::from_str(&body_to_string(response).await).unwrap();
        assert_eq!(json["error"], "Missing Consumer Info header");
    }

    /// Backend that reports every `x-consumer-info` header value it receives.
    async fn spawn_consumer_info_backend() -> u16 {
        let make = make_service_fn(|_| async {
            Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
                let values: Vec<String> = req
                    .headers()
                    .get_all("x-consumer-info")
                    .iter()
                    .map(|v| v.to_str().unwrap_or("?").to_string())
                    .collect();
                let body = if values.is_empty() {
                    "x-consumer-info:absent".to_string()
                } else {
                    format!("x-consumer-info:{}", values.join("|"))
                };
                Ok::<_, Infallible>(Response::new(Body::from(body)))
            }))
        });
        let server = Server::bind(&SocketAddr::from(([127, 0, 0, 1], 0))).serve(make);
        let port = server.local_addr().port();
        tokio::spawn(server);
        port
    }

    #[tokio::test]
    async fn test_valid_consumer_info_is_forwarded_once_in_distinct_out_header() {
        use jsonwebtoken::{EncodingKey, Header, encode};

        let backend_port = spawn_consumer_info_backend().await;
        let opts = TestConfigOpts {
            use_v1: true,
            consumer_info: true,
            consumer_info_out_header: Some("X-Consumer-Info"),
            ..Default::default()
        };
        let mut service = svc(test_config_with(backend_port, &[], opts));

        let claims = serde_json::json!({ "apikey": { "clientId": "abc" }, "exp": chrono::Utc::now().timestamp() + 60 });
        let ci_token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(b"ci-secret"),
        )
        .unwrap();

        // A forged output header travels with the valid input token: only the decoded JSON
        // produced by the proxy must reach the backend, exactly once.
        let req = Request::builder()
            .method("GET")
            .uri("/api")
            .header("Otoroshi-State", "abc")
            .header("Otoroshi-Claims", ci_token)
            .header("X-Consumer-Info", "{\"forged\":true}")
            .body(Body::empty())
            .unwrap();
        let response = service.call(req).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
        let body = body_to_string(response).await;
        assert!(!body.contains("forged"), "forged header leaked: {body}");
        assert!(
            !body.contains('|'),
            "output header must be sent once: {body}"
        );
        let json_part = body.trim_start_matches("x-consumer-info:");
        let decoded: serde_json::Value = serde_json::from_str(json_part).unwrap();
        assert_eq!(decoded["apikey"]["clientId"], "abc");
    }

    #[tokio::test]
    async fn test_client_supplied_consumer_info_out_header_is_never_forwarded() {
        // Backend echoes the presence of the output header in its body
        let make = make_service_fn(|_| async {
            Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
                let body = format!(
                    "x-consumer-info:{}",
                    req.headers()
                        .get("x-consumer-info")
                        .map(|v| v.to_str().unwrap_or("?"))
                        .unwrap_or("absent")
                );
                Ok::<_, Infallible>(Response::new(Body::from(body)))
            }))
        });
        let server = Server::bind(&SocketAddr::from(([127, 0, 0, 1], 0))).serve(make);
        let backend_port = server.local_addr().port();
        tokio::spawn(server);

        let opts = TestConfigOpts {
            consumer_info: true,
            consumer_info_out_header: Some("X-Consumer-Info"),
            ..Default::default()
        };
        let mut service = svc(test_config_with(backend_port, &["/health"], opts));

        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .header("X-Consumer-Info", "{\"forged\":true}")
            .body(Body::empty())
            .unwrap();
        let response = service.call(req).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(body_to_string(response).await, "x-consumer-info:absent");
    }

    #[tokio::test]
    async fn test_excluded_path_with_non_utf8_state_header_is_rejected() {
        let backend_port = spawn_backend().await;
        let config = test_config(backend_port, &["/health"]);
        let mut service = svc(config);

        // Header present but not valid UTF-8: must count as present, hence no bypass
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .header(
                "Otoroshi-State",
                http::header::HeaderValue::from_bytes(&[0xff, 0xfe]).unwrap(),
            )
            .body(Body::empty())
            .unwrap();
        let response = service.call(req).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_excluded_path_with_query_and_head_bypasses_challenge() {
        let backend_port = spawn_backend().await;
        let config = test_config(backend_port, &["/health"]);
        let mut service = svc(config);

        let response = service
            .call(request("HEAD", "/health?probe=1"))
            .await
            .unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_non_excluded_path_still_requires_challenge() {
        let backend_port = spawn_backend().await;
        let config = test_config(backend_port, &["/health"]);
        let mut service = svc(config);

        let response = service.call(request("GET", "/api")).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
        let json: serde_json::Value =
            serde_json::from_str(&body_to_string(response).await).unwrap();
        assert_eq!(json["error"], "Missing Otoroshi-State header");
    }

    #[tokio::test]
    async fn test_excluded_path_with_post_still_requires_challenge() {
        let backend_port = spawn_backend().await;
        let config = test_config(backend_port, &["/health"]);
        let mut service = svc(config);

        let response = service.call(request("POST", "/health")).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_no_excluded_paths_requires_challenge_everywhere() {
        let backend_port = spawn_backend().await;
        let config = test_config(backend_port, &[]);
        let mut service = svc(config);

        let response = service.call(request("GET", "/health")).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_excluded_path_with_invalid_token_is_rejected() {
        let backend_port = spawn_backend().await;
        let config = test_config(backend_port, &["/health"]);
        let mut service = svc(config);

        // A state header on an excluded path triggers the normal verification
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .header("Otoroshi-State", "not-a-jwt")
            .body(Body::empty())
            .unwrap();
        let response = service.call(req).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
        let json: serde_json::Value =
            serde_json::from_str(&body_to_string(response).await).unwrap();
        assert_eq!(json["error"], "Invalid Otoroshi challenge");
    }

    #[tokio::test]
    async fn test_excluded_path_with_valid_token_answers_the_challenge() {
        use jsonwebtoken::{EncodingKey, Header, encode};
        use serde::Serialize;

        let backend_port = spawn_backend().await;
        let config = test_config(backend_port, &["/health"]);
        let mut service = svc(config);

        #[derive(Serialize)]
        struct Challenge {
            state: String,
            iss: String,
            iat: i64,
            exp: i64,
        }
        let now = chrono::Utc::now().timestamp();
        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &Challenge {
                state: "state-123".to_string(),
                iss: "Otoroshi".to_string(),
                iat: now,
                exp: now + 60,
            },
            &EncodingKey::from_secret(b"test-secret"),
        )
        .unwrap();

        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .header("Otoroshi-State", token)
            .body(Body::empty())
            .unwrap();
        let response = service.call(req).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
        let state_resp = response
            .headers()
            .get("otoroshi-state-resp")
            .expect("a valid challenge on an excluded path must still be answered");
        assert_ne!(
            state_resp, "forged-by-backend",
            "the proxy must overwrite a state response header coming from the backend"
        );
        assert_eq!(body_to_string(response).await, "backend:GET:/health");
    }
}
