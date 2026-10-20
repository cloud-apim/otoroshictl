use futures_util::Future;
use http::Response;
use hyper::Request;
use hyper::body::Incoming;
use hyper::service::Service;
use hyper_util::client::legacy::ResponseFuture;
use rustls::server::WebPkiClientVerifier;
use tokio_rustls::TlsAcceptor;

use super::cache::{OtoroshiCertificate, OtoroshiChallengePlugin, SidecarCache};
use super::config::OtoroshiSidecarConfig;

use crate::http_utils::client::{HttpClient, http_client};
use crate::http_utils::server::{bind, serve};
use crate::otoroshi::protocol::OtoroshiProtocol;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct InboundProxyClient {
    config: OtoroshiSidecarConfig,
    http_client: HttpClient<Incoming>,
}

impl InboundProxyClient {
    fn new(config: OtoroshiSidecarConfig) -> InboundProxyClient {
        let http_client = http_client();
        InboundProxyClient {
            config,
            http_client,
        }
    }

    fn request(&self, mut req: Request<Incoming>) -> ResponseFuture {
        let uri_string = format!(
            "{}://{}:{}{}",
            "http",
            "127.0.0.1",
            self.config.spec.inbound.target_port.unwrap_or(8080),
            req.uri()
                .path_and_query()
                .map(|x| x.as_str())
                .unwrap_or("/")
        );
        let uri = uri_string.parse().unwrap();
        *req.uri_mut() = uri;
        let version = match self.config.clone().spec.inbound.target_version {
            Some(v) if v == "h2" => http::version::Version::HTTP_2,
            _ => http::version::Version::HTTP_11,
        };
        *req.version_mut() = version;
        self.http_client.request(req)
    }
}

pub struct InboundProxy {}

impl InboundProxy {
    pub fn start_http(
        config: OtoroshiSidecarConfig,
        cache: Arc<SidecarCache>,
    ) -> impl Future<Output = ()> {
        let in_addr: SocketAddr = SocketAddr::new(
            "0.0.0.0".parse().unwrap(),
            config.spec.inbound.port.unwrap_or(15000),
        );
        let client = InboundProxyClient::new(config.clone());
        let svc = Svc {
            client,
            config,
            cache,
        };
        async move {
            let listener = bind(in_addr).await;
            info!(target: "inbound_proxy", "listening on http://{}", in_addr);
            serve(listener, None, svc, std::future::pending()).await
        }
    }

    pub fn start_https(
        config: OtoroshiSidecarConfig,
        cache: Arc<SidecarCache>,
    ) -> impl Future<Output = ()> {
        let in_addr: SocketAddr = SocketAddr::new(
            "0.0.0.0".parse().unwrap(),
            config.spec.inbound.port.unwrap_or(15000),
        );
        let client = InboundProxyClient::new(config.clone());
        let mtls = config
            .clone()
            .spec
            .inbound
            .mtls
            .map(|i| i.enabled)
            .unwrap_or(false);
        let cert_id: String = config
            .clone()
            .spec
            .inbound
            .tls
            .map(|i| i.cert_id)
            .unwrap_or("none".to_string());
        let certificate = cache.wait_and_get_cert_by_id(cert_id.clone(), 30);
        let svc = Svc {
            client,
            config: config.clone(),
            cache: cache.clone(),
        };
        let client_ca = if mtls {
            let ca_id = config
                .clone()
                .spec
                .inbound
                .mtls
                .map(|i| i.ca_cert_id)
                .unwrap_or("none".to_string());
            Some(cache.wait_and_get_cert_by_id(ca_id, 30))
        } else {
            None
        };
        let server_config = tls_config(&certificate, client_ca.as_ref());
        let acceptor = TlsAcceptor::from(Arc::new(server_config));
        async move {
            let listener = bind(in_addr).await;
            if mtls {
                info!(target: "inbound_proxy", "listening on https://{} with mTLS", in_addr);
            } else {
                info!(target: "inbound_proxy", "listening on https://{}", in_addr);
            }
            serve(listener, Some(acceptor), svc, std::future::pending()).await
        }
    }
}

/// TLS settings of the inbound proxy. When `client_ca` is set, clients must
/// present a certificate signed by it (mTLS).
fn tls_config(
    certificate: &OtoroshiCertificate,
    client_ca: Option<&OtoroshiCertificate>,
) -> rustls::ServerConfig {
    let builder = rustls::ServerConfig::builder();
    let builder = match client_ca {
        Some(ca) => {
            let mut roots = rustls::RootCertStore::empty();
            roots.add_parsable_certificates(ca.certs());
            let client_auth = WebPkiClientVerifier::builder(Arc::new(roots))
                .build()
                .unwrap_or_else(|e| panic!("invalid mTLS CA certificate '{}': {}", ca.id, e));
            builder.with_client_cert_verifier(client_auth)
        }
        None => builder.with_no_client_auth(),
    };
    let mut server_config = builder
        .with_single_cert(certificate.certs(), certificate.key())
        .unwrap();
    server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    server_config
}

#[derive(Clone)]
struct Svc {
    client: InboundProxyClient,
    config: OtoroshiSidecarConfig,
    cache: Arc<SidecarCache>,
}

impl Svc {
    /// Process Otoroshi protocol challenge using the shared module.
    fn process_otoroshi_protocol(
        version: &str,
        state_value: &str,
        secret_in: &str,
        algo_in: &str,
        secret_out: &str,
        algo_out: &str,
    ) -> Option<String> {
        if version == "V2" {
            let protocol = OtoroshiProtocol::new_asymmetric(
                secret_in.as_bytes(),
                algo_in.parse().unwrap_or_default(),
                secret_out.as_bytes(),
                algo_out.parse().unwrap_or_default(),
            );
            match protocol.process_v2(state_value) {
                Ok(response_token) => Some(response_token),
                Err(e) => {
                    error!("Otoroshi protocol error: {}", e);
                    None
                }
            }
        } else {
            // V1: Simple echo
            Some(state_value.to_string())
        }
    }

    fn get_additional_headers(
        &self,
        in_headers: http::HeaderMap,
    ) -> HashMap<http::HeaderName, http::HeaderValue> {
        let mut headers = HashMap::new();
        if self
            .config
            .clone()
            .spec
            .inbound
            .otoroshi_protocol
            .map(|i| i.enabled)
            .unwrap_or(false)
        {
            let proto = self.config.clone().spec.inbound.otoroshi_protocol.unwrap();
            match proto.route_id {
                None => {
                    let version = proto.version.unwrap_or("V2".to_string());
                    let secret_in = proto.secret_in.unwrap_or("secret".to_string());
                    let algo_in = proto.algo_in.unwrap_or("HS512".to_string());
                    let secret_out = proto.secret_out.unwrap_or("secret".to_string());
                    let algo_out = proto.algo_out.unwrap_or("HS512".to_string());
                    let header_name_in = proto
                        .header_in_name
                        .map(|i| i.to_ascii_lowercase())
                        .unwrap_or("otoroshi-state".to_string());
                    let header_name_out = proto
                        .header_out_name
                        .map(|i| i.to_ascii_lowercase())
                        .unwrap_or("otoroshi-state-resp".to_string());

                    if let Some(state) = in_headers.get(&header_name_in)
                        && let Ok(state_str) = state.to_str()
                        && let Some(response) = Self::process_otoroshi_protocol(
                            &version,
                            state_str,
                            &secret_in,
                            &algo_in,
                            &secret_out,
                            &algo_out,
                        )
                        && let (Ok(name), Ok(value)) = (
                            http::HeaderName::from_str(&header_name_out),
                            http::HeaderValue::from_str(&response),
                        )
                    {
                        headers.insert(name, value);
                    }
                }
                Some(route_id) => match self.cache.get_route_by_id(route_id.clone()) {
                    None => error!("route with id '{}' does not exist", route_id.clone()),
                    Some(route) => {
                        match route.plugins.into_iter().find(|plugin| {
                            plugin.plugin == "cp:otoroshi.next.plugins.OtoroshiChallenge"
                        }) {
                            None => error!(
                                "the specified route with id '{}' does not have the OtoroshiChallenge plugin",
                                route_id
                            ),
                            Some(plugin) => {
                                if let Ok(config) =
                                    serde_json::from_value::<OtoroshiChallengePlugin>(plugin.config)
                                {
                                    let version = config.version;
                                    let secret_in = config.algo_to_backend.secret;
                                    let algo_in = format!("HS{}", config.algo_to_backend.size);
                                    let secret_out = config.algo_from_backend.secret;
                                    let algo_out = format!("HS{}", config.algo_from_backend.size);
                                    let header_name_in = config
                                        .request_header_name
                                        .unwrap_or("otoroshi-state".to_string());
                                    let header_name_out = config
                                        .response_header_name
                                        .unwrap_or("otoroshi-state-resp".to_string());

                                    if let Some(state) = in_headers.get(&header_name_in)
                                        && let Ok(state_str) = state.to_str()
                                        && let Some(response) = Self::process_otoroshi_protocol(
                                            &version,
                                            state_str,
                                            &secret_in,
                                            &algo_in,
                                            &secret_out,
                                            &algo_out,
                                        )
                                        && let (Ok(name), Ok(value)) = (
                                            http::HeaderName::from_str(&header_name_out),
                                            http::HeaderValue::from_str(&response),
                                        )
                                    {
                                        headers.insert(name, value);
                                    }
                                }
                            }
                        }
                    }
                },
            };
        }
        headers
    }
}

impl Service<Request<Incoming>> for Svc {
    type Response = Response<Incoming>;
    type Error = hyper_util::client::legacy::Error;
    type Future = ResponseFuture;

    fn call(&self, mut req: Request<Incoming>) -> Self::Future {
        let mut headers = req.headers().clone();
        let default_value = http::HeaderValue::from_str("localhost").unwrap();
        let authority_value = http::HeaderValue::from_str(
            req.uri()
                .authority()
                .map(|a| a.host())
                .unwrap_or("localhost"),
        )
        .unwrap();
        let orig_host = headers
            .get("Host")
            .or(headers.get(":authority"))
            .or(Some(&authority_value))
            .unwrap_or(&default_value);
        let host = match self.config.clone().spec.inbound.target_hostname {
            None => orig_host.to_owned(),
            Some(hostname) => http::HeaderValue::from_str(hostname.clone().as_str()).unwrap(),
        };
        let host_str = host.to_str().unwrap().to_string();
        let h_map = http::HeaderMap::from_iter(self.get_additional_headers(headers.clone()));
        info!(target: "inbound_proxy", "{} https://{}{}, otoroshi_protocol: {}", req.method(), host_str, req.uri().path(), if h_map.is_empty() { "no" } else { "yes" });
        headers.insert("Host", host.clone());
        headers.extend(h_map);
        *req.headers_mut() = headers;
        self.client.request(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_utils::client::{HttpsClient, RequestBody, error_chain};
    use http_body_util::Full;
    use hyper::body::Bytes;
    use hyper::service::service_fn;
    use hyper_util::client::legacy::Client;
    use hyper_util::rt::TokioExecutor;
    use rcgen::{BasicConstraints, CertificateParams, CertifiedIssuer, IsCa, KeyPair};
    use rustls::pki_types::ServerName;
    use std::convert::Infallible;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;
    use tokio_rustls::TlsConnector;

    /// A CA and a leaf certificate for `localhost` signed by it.
    struct TestPki {
        ca: OtoroshiCertificate,
        leaf: OtoroshiCertificate,
    }

    fn otoroshi_certificate(id: &str, chain: String, key: &KeyPair) -> OtoroshiCertificate {
        OtoroshiCertificate {
            id: id.to_string(),
            name: id.to_string(),
            chain,
            privateKey: key.serialize_pem(),
            subject: id.to_string(),
        }
    }

    fn test_pki() -> TestPki {
        let ca_key = KeyPair::generate().unwrap();
        let mut ca_params = CertificateParams::new(Vec::<String>::new()).unwrap();
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        let ca_cert_pem = ca_params.self_signed(&ca_key).unwrap().pem();
        let ca = otoroshi_certificate("test-ca", ca_cert_pem, &ca_key);
        let issuer = CertifiedIssuer::self_signed(ca_params, ca_key).unwrap();

        let leaf_key = KeyPair::generate().unwrap();
        let leaf_params = CertificateParams::new(vec!["localhost".to_string()]).unwrap();
        let leaf_cert = leaf_params.signed_by(&leaf_key, &issuer).unwrap();
        let leaf = otoroshi_certificate("test-leaf", leaf_cert.pem(), &leaf_key);
        TestPki { ca, leaf }
    }

    /// Serve `tls_config(server, client_ca)` on a random port, answering "ok" to everything.
    async fn spawn_tls_server(
        server: &OtoroshiCertificate,
        client_ca: Option<&OtoroshiCertificate>,
    ) -> u16 {
        let acceptor = TlsAcceptor::from(Arc::new(tls_config(server, client_ca)));
        let listener = bind(SocketAddr::from(([127, 0, 0, 1], 0))).await;
        let port = listener.local_addr().unwrap().port();
        let svc = service_fn(|_req: Request<Incoming>| async {
            Ok::<_, Infallible>(Response::new(Full::new(Bytes::from_static(b"ok"))))
        });
        tokio::spawn(serve(listener, Some(acceptor), svc, std::future::pending()));
        port
    }

    fn client_config(
        ca: &OtoroshiCertificate,
        client_cert: Option<&OtoroshiCertificate>,
    ) -> rustls::ClientConfig {
        let mut roots = rustls::RootCertStore::empty();
        roots.add_parsable_certificates(ca.certs());
        let builder = rustls::ClientConfig::builder().with_root_certificates(roots);
        match client_cert {
            Some(cert) => builder
                .with_client_auth_cert(cert.certs(), cert.key())
                .unwrap(),
            None => builder.with_no_client_auth(),
        }
    }

    fn https_client(config: rustls::ClientConfig) -> HttpsClient {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(config)
            .https_only()
            .enable_http1()
            .build();
        Client::builder(TokioExecutor::new()).build(https)
    }

    async fn get(client: &HttpsClient, port: u16) -> Result<u16, String> {
        let req = Request::get(format!("https://localhost:{}/", port))
            .body(RequestBody::default())
            .unwrap();
        let resp = client.request(req).await.map_err(|e| error_chain(&e))?;
        let status = resp.status().as_u16();
        crate::http_utils::client::read_body(resp)
            .await
            .map_err(|e| e.to_string())?;
        Ok(status)
    }

    #[test]
    fn test_certificate_pem_parsing() {
        let pki = test_pki();
        assert_eq!(pki.leaf.certs().len(), 1);
        let chain = OtoroshiCertificate {
            chain: format!("{}\n\n{}", pki.leaf.chain, pki.ca.chain),
            ..pki.leaf.clone()
        };
        assert_eq!(chain.certs().len(), 2);
        assert!(matches!(
            chain.key(),
            rustls::pki_types::PrivateKeyDer::Pkcs8(_)
        ));
    }

    #[tokio::test]
    async fn test_tls_server_accepts_trusting_client() {
        let pki = test_pki();
        let port = spawn_tls_server(&pki.leaf, None).await;
        let client = https_client(client_config(&pki.ca, None));
        assert_eq!(get(&client, port).await, Ok(200));
    }

    #[tokio::test]
    async fn test_tls_server_negotiates_h2_through_alpn() {
        let pki = test_pki();
        let port = spawn_tls_server(&pki.leaf, None).await;
        let mut config = client_config(&pki.ca, None);
        config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        let stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        let tls = TlsConnector::from(Arc::new(config))
            .connect(ServerName::try_from("localhost").unwrap(), stream)
            .await
            .unwrap();
        assert_eq!(tls.get_ref().1.alpn_protocol(), Some(&b"h2"[..]));
    }

    #[tokio::test]
    async fn test_untrusted_server_error_reports_the_tls_cause() {
        let pki = test_pki();
        let other = test_pki();
        let port = spawn_tls_server(&pki.leaf, None).await;
        let client = https_client(client_config(&other.ca, None));
        let err = get(&client, port).await.unwrap_err();
        assert!(err.contains("invalid peer certificate"), "{err}");
    }

    #[tokio::test]
    async fn test_mtls_server_rejects_client_without_certificate() {
        let pki = test_pki();
        let port = spawn_tls_server(&pki.leaf, Some(&pki.ca)).await;
        let client = https_client(client_config(&pki.ca, None));
        assert!(get(&client, port).await.is_err());
    }

    #[tokio::test]
    async fn test_mtls_server_rejects_certificate_from_another_ca() {
        let pki = test_pki();
        let other = test_pki();
        let port = spawn_tls_server(&pki.leaf, Some(&pki.ca)).await;
        let client = https_client(client_config(&pki.ca, Some(&other.leaf)));
        assert!(get(&client, port).await.is_err());
    }

    #[tokio::test]
    async fn test_mtls_server_accepts_certificate_signed_by_its_ca() {
        let pki = test_pki();
        let port = spawn_tls_server(&pki.leaf, Some(&pki.ca)).await;
        let client = https_client(client_config(&pki.ca, Some(&pki.leaf)));
        assert_eq!(get(&client, port).await, Ok(200));
    }

    #[tokio::test]
    async fn test_failed_handshake_does_not_stop_the_server() {
        let pki = test_pki();
        let port = spawn_tls_server(&pki.leaf, None).await;
        let mut garbage = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        garbage
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .await
            .unwrap();
        drop(garbage);
        let client = https_client(client_config(&pki.ca, None));
        assert_eq!(get(&client, port).await, Ok(200));
    }

    #[test]
    #[should_panic(expected = "invalid mTLS CA certificate 'empty-ca'")]
    fn test_mtls_without_valid_ca_fails_fast() {
        let pki = test_pki();
        let empty_ca = OtoroshiCertificate {
            id: "empty-ca".to_string(),
            chain: String::new(),
            ..pki.ca.clone()
        };
        tls_config(&pki.leaf, Some(&empty_ca));
    }
}
