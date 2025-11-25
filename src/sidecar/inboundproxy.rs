use futures_util::Future;
use hmac::Hmac;
use http::Response;
use hyper::client::{ResponseFuture, HttpConnector};
use hyper::server::conn::AddrIncoming;
use hyper::{Client, Server, Request, Body};
use hyper::service::Service;
use hyper_rustls::TlsAcceptor;
use sha2::{Sha256, Sha384, Sha512};

use super::config::OtoroshiSidecarConfig;
use super::cache::{SidecarCache, OtoroshiChallengePlugin};

use std::collections::{HashMap, BTreeMap};
use std::net::SocketAddr;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use hmac::digest::KeyInit;
use jwt::{VerifyWithKey, SignWithKey, AlgorithmType, Header, Token};

#[derive(Clone, Debug)]
struct InboundProxyClient {
    config: OtoroshiSidecarConfig, 
    http_client: Client<HttpConnector>,
}

impl InboundProxyClient {

    fn new(config: OtoroshiSidecarConfig) -> InboundProxyClient {
        let http_client = Client::new();
        InboundProxyClient {
            config,
            http_client,
        }
    }

    fn request(&self, mut req: Request<Body>) -> ResponseFuture {
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

    pub fn start_http(config: OtoroshiSidecarConfig, cache: Arc<SidecarCache>) -> impl Future<Output = std::result::Result<(), hyper::Error>> {
        let in_addr: SocketAddr = SocketAddr::new("0.0.0.0".parse().unwrap(), config.spec.inbound.port.unwrap_or(15000));
        let client = InboundProxyClient::new(config.clone());
        let make_svc = MakeSvc { 
            client, 
            config,
            cache,
        };
        let server = Server::bind(&in_addr).serve(make_svc);
        info!(target: "inbound_proxy", "listening on http://{}", in_addr);
        server
    }

    pub fn start_https(config: OtoroshiSidecarConfig, cache: Arc<SidecarCache>) -> impl Future<Output = std::result::Result<(), hyper::Error>> {
        let in_addr: SocketAddr = SocketAddr::new("0.0.0.0".parse().unwrap(), config.spec.inbound.port.unwrap_or(15000));
        let client = InboundProxyClient::new(config.clone());
        let mtls = config.clone().spec.inbound.mtls.map(|i| i.enabled).unwrap_or(false);
        let cert_id: String = config.clone().spec.inbound.tls.map(|i| i.cert_id).unwrap_or("none".to_string());
        let certificate = cache.wait_and_get_cert_by_id(cert_id.clone(), 30);
        let make_svc = MakeSvc { 
            client, 
            config: config.clone(),
            cache: cache.clone(),
        };
        if mtls {
            let incoming = AddrIncoming::bind(&in_addr).unwrap();
            let mut root = rustls::RootCertStore::empty(); 
            let ca_id = config.clone().spec.inbound.mtls.map(|i| i.ca_cert_id).unwrap_or("none".to_string());
            let ca = cache.wait_and_get_cert_by_id(ca_id.clone(), 30);
            for cert in ca.certs().into_iter() {
                let _ = root.add(&cert);
            }
            let client_auth = Arc::new(rustls::server::AllowAnyAuthenticatedClient::new(root));
            let server_config = rustls::ServerConfig::builder()
                .with_safe_default_cipher_suites()
                .with_safe_default_kx_groups()
                .with_protocol_versions(&[&rustls::version::TLS12, &rustls::version::TLS13])
                .unwrap()
                .with_client_cert_verifier(client_auth)
                .with_single_cert(certificate.certs(), certificate.key())
                .unwrap();
            let acceptor = TlsAcceptor::builder()
                .with_tls_config(server_config)
                .with_alpn_protocols(vec![b"h2".to_vec(), b"http/1.1".to_vec()])
                .with_incoming(incoming);
            let server = Server::builder(acceptor).serve(make_svc);
            info!(target: "inbound_proxy", "listening on https://{} with mTLS", in_addr);
            server
        } else {
            let incoming = AddrIncoming::bind(&in_addr).unwrap();
            let server_config = rustls::ServerConfig::builder()
                .with_safe_default_cipher_suites()
                .with_safe_default_kx_groups()
                .with_protocol_versions(&[&rustls::version::TLS12, &rustls::version::TLS13])
                .unwrap()
                .with_no_client_auth()
                .with_single_cert(certificate.certs(), certificate.key())
                .unwrap();
            let acceptor = TlsAcceptor::builder()
                .with_tls_config(server_config)
                .with_alpn_protocols(vec![b"h2".to_vec(), b"http/1.1".to_vec()])
                .with_incoming(incoming);
            let server = Server::builder(acceptor).serve(make_svc);
            info!(target: "inbound_proxy", "listening on https://{}", in_addr);
            server
        }
    }
}


struct Svc {
    client: InboundProxyClient,
    config: OtoroshiSidecarConfig,
    cache: Arc<SidecarCache>,
} 

impl Svc {
    fn get_additional_headers(&self, in_headers: http::HeaderMap) -> HashMap<http::HeaderName, http::HeaderValue> {
        let mut headers = HashMap::new();
        if self.config.clone().spec.inbound.otoroshi_protocol.map(|i| i.enabled).unwrap_or(false) {
            let proto = self.config.clone().spec.inbound.otoroshi_protocol.unwrap();
            match proto.route_id {
                None => {
                    let version = proto.version.unwrap_or("V2".to_string());
                    let secret_in = proto.secret_in.unwrap_or("secret".to_string());
                    let algo_in = proto.algo_in.unwrap_or("HS512".to_string());
                    let secret_out = proto.secret_out.unwrap_or("secret".to_string());
                    let algo_out = proto.algo_out.unwrap_or("HS512".to_string());
                    let header_name_in = proto.header_in_name.map(|i| i.to_ascii_lowercase()).unwrap_or("otoroshi-state".to_string());
                    let header_name_out = proto.header_out_name.map(|i| i.to_ascii_lowercase()).unwrap_or("otoroshi-state-resp".to_string());
                    match in_headers.get(header_name_in) {
                        None => (),
                        Some(state) => {
                            if version == "V2" {
                                let token = state.to_str().unwrap();
                                let res: Result<BTreeMap<String, String>, jwt::Error> = {
                                    match algo_in.as_str() {
                                        "HS256" => {
                                            let key: Hmac<Sha256> = hmac::Hmac::new_from_slice(secret_in.as_bytes()).unwrap();
                                            token.verify_with_key(&key)
                                        },
                                        "HS384" => {
                                            let key: Hmac<Sha384> = hmac::Hmac::new_from_slice(secret_in.as_bytes()).unwrap();
                                            token.verify_with_key(&key)
                                        },
                                        "HS512" => {
                                            let key: Hmac<Sha512> = hmac::Hmac::new_from_slice(secret_in.as_bytes()).unwrap();
                                            token.verify_with_key(&key)
                                        },
                                        _ => {
                                            let key: Hmac<Sha512> = hmac::Hmac::new_from_slice(secret_in.as_bytes()).unwrap();
                                            token.verify_with_key(&key)
                                        }
                                    }
                                };
                                match res {
                                    Err(e) => error!("bad input token: {}", e),
                                    Ok(claims) => { 
                                        match claims.get("state") {
                                            None => error!("no state in token"),
                                            Some(state) => {
                                                let alg: AlgorithmType = match algo_out.as_str() {
                                                    "HS256" => AlgorithmType::Hs256,
                                                    "HS384" => AlgorithmType::Hs384,
                                                    _ => AlgorithmType::Hs512,
                                                };
                                                let header = Header {
                                                    algorithm: alg,
                                                    ..Default::default()
                                                };
                                                let mut claims: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
                                                let date: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                                                claims.insert("state-resp", serde_json::Value::String(state.to_string()));
                                                claims.insert("exp", serde_json::Value::Number((date + 10).into()));
                                                claims.insert("iat", serde_json::Value::Number(date.into()));
                                                claims.insert("nbf", serde_json::Value::Number(date.into()));
                                                let token = match algo_out.as_str() {
                                                    "HS256" => {
                                                        let key: Hmac<Sha256> = hmac::Hmac::new_from_slice(secret_out.as_bytes()).unwrap();
                                                        Token::new(header, claims).sign_with_key(&key).unwrap()
                                                    },
                                                    "HS384" => {
                                                        let key: Hmac<Sha384> = hmac::Hmac::new_from_slice(secret_out.as_bytes()).unwrap();
                                                        Token::new(header, claims).sign_with_key(&key).unwrap()
                                                    },
                                                    "HS512" => {
                                                        let key: Hmac<Sha512> = hmac::Hmac::new_from_slice(secret_out.as_bytes()).unwrap();
                                                        Token::new(header, claims).sign_with_key(&key).unwrap()
                                                    },
                                                    _ => {
                                                        let key: Hmac<Sha512> = hmac::Hmac::new_from_slice(secret_out.as_bytes()).unwrap();
                                                        Token::new(header, claims).sign_with_key(&key).unwrap()
                                                    }
                                                };
                                                headers.insert(
                                                    http::HeaderName::from_str(header_name_out.as_str()).unwrap(),
                                                    http::HeaderValue::from_str(token.as_str()).unwrap(),
                                                );
                                            }
                                        };
                                    }
                                };
                            } else {
                                headers.insert(http::HeaderName::from_str(header_name_out.as_str()).unwrap(), state.to_owned());
                            }
                        }
                    };
                },
                Some(route_id) => {
                    match self.cache.get_route_by_id(route_id.clone()) {
                        None => error!("rooute with id '{}' does not exists", route_id.clone()),
                        Some(route) => {
                            match route.plugins.into_iter().find(|plugin| plugin.plugin == "cp:otoroshi.next.plugins.OtoroshiChallenge") {
                                None => error!("the specified route with id '{}' does not have the OtoroshiChallenge plugin", route_id),
                                Some(plugin) => {
                                    let config = serde_json::from_value::<OtoroshiChallengePlugin>(plugin.config).unwrap();
                                    let version = config.version;
                                    let secret_in = config.algo_to_backend.secret;
                                    let algo_in = format!("HS{}", config.algo_to_backend.size);
                                    let secret_out = config.algo_from_backend.secret;
                                    let algo_out = format!("HS{}", config.algo_from_backend.size);
                                    let header_name_in = config.request_header_name.unwrap_or("otoroshi-state".to_string());
                                    let header_name_out = config.response_header_name.unwrap_or("otoroshi-state-resp".to_string());
                                    match in_headers.get(header_name_in) {
                                        None => (),
                                        Some(state) => {
                                            if version == "V2" {
                                                let token = state.to_str().unwrap();
                                                let res: Result<BTreeMap<String, String>, jwt::Error> = {
                                                    match algo_in.as_str() {
                                                        "HS256" => {
                                                            let key: Hmac<Sha256> = hmac::Hmac::new_from_slice(secret_in.as_bytes()).unwrap();
                                                            token.verify_with_key(&key)
                                                        },
                                                        "HS384" => {
                                                            let key: Hmac<Sha384> = hmac::Hmac::new_from_slice(secret_in.as_bytes()).unwrap();
                                                            token.verify_with_key(&key)
                                                        },
                                                        "HS512" => {
                                                            let key: Hmac<Sha512> = hmac::Hmac::new_from_slice(secret_in.as_bytes()).unwrap();
                                                            token.verify_with_key(&key)
                                                        },
                                                        _ => {
                                                            let key: Hmac<Sha512> = hmac::Hmac::new_from_slice(secret_in.as_bytes()).unwrap();
                                                            token.verify_with_key(&key)
                                                        }
                                                    }
                                                };
                                                match res {
                                                    Err(e) => error!("bad input token: {}", e),
                                                    Ok(claims) => { 
                                                        match claims.get("state") {
                                                            None => error!("no state in token"),
                                                            Some(state) => {
                                                                let alg: AlgorithmType = match algo_out.as_str() {
                                                                    "HS256" => AlgorithmType::Hs256,
                                                                    "HS384" => AlgorithmType::Hs384,
                                                                    _ => AlgorithmType::Hs512,
                                                                };
                                                                let header = Header {
                                                                    algorithm: alg,
                                                                    ..Default::default()
                                                                };
                                                                let mut claims: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
                                                                let date: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                                                                claims.insert("state-resp", serde_json::Value::String(state.to_string()));
                                                                claims.insert("exp", serde_json::Value::Number((date + 10).into()));
                                                                claims.insert("iat", serde_json::Value::Number(date.into()));
                                                                claims.insert("nbf", serde_json::Value::Number(date.into()));
                                                                let token = match algo_out.as_str() {
                                                                    "HS256" => {
                                                                        let key: Hmac<Sha256> = hmac::Hmac::new_from_slice(secret_out.as_bytes()).unwrap();
                                                                        Token::new(header, claims).sign_with_key(&key).unwrap()
                                                                    },
                                                                    "HS384" => {
                                                                        let key: Hmac<Sha384> = hmac::Hmac::new_from_slice(secret_out.as_bytes()).unwrap();
                                                                        Token::new(header, claims).sign_with_key(&key).unwrap()
                                                                    },
                                                                    "HS512" => {
                                                                        let key: Hmac<Sha512> = hmac::Hmac::new_from_slice(secret_out.as_bytes()).unwrap();
                                                                        Token::new(header, claims).sign_with_key(&key).unwrap()
                                                                    },
                                                                    _ => {
                                                                        let key: Hmac<Sha512> = hmac::Hmac::new_from_slice(secret_out.as_bytes()).unwrap();
                                                                        Token::new(header, claims).sign_with_key(&key).unwrap()
                                                                    }
                                                                };
                                                                headers.insert(
                                                                    http::HeaderName::from_str(header_name_out.as_str()).unwrap(),
                                                                    http::HeaderValue::from_str(token.as_str()).unwrap(),
                                                                );
                                                            }
                                                        };
                                                    }
                                                };
                                            } else {
                                                headers.insert(http::HeaderName::from_str(header_name_out.as_str()).unwrap(), state.to_owned());
                                            }
                                        }
                                    };
                                }
                            }
                        }
                    }
                }
            };
        }
        headers
    }
}

impl Service<Request<Body>> for Svc {

    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let mut headers = req.headers().clone();
        let default_value = http::HeaderValue::from_str("localhost").unwrap();
        let authority_value = http::HeaderValue::from_str(req.uri().authority().map(|a| a.host()).unwrap_or("localhost")).unwrap();
        let orig_host = headers.get("Host").or(headers.get(":authority")).or(Some(&authority_value)).unwrap_or(&default_value);
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

struct MakeSvc {
    client: InboundProxyClient,
    config: OtoroshiSidecarConfig,
    cache: Arc<SidecarCache>,
}

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(& mut self, _: T) -> Self::Future {
        let client = self.client.clone();
        let config = self.config.clone();
        let cache = self.cache.clone();
        let future = async move { Ok(Svc { client, config, cache }) };
        Box::pin(future)
    }
}