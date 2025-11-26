use crate::cli::cliopts::CliOpts;
use crate::sidecar::config::{
    OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation,
    OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation,
};
use futures_util::Future;
use http::Response;
use hyper::client::ResponseFuture;
use hyper::service::Service;
use hyper::{Body, Client, Request, Server};
use hyper_rustls::HttpsConnector;
use moka::sync::{Cache, CacheBuilder};
use rustls::{OwnedTrustAnchor, RootCertStore};

use super::cache::{OtoroshiCertificate, SidecarCache};
use super::config::OtoroshiSidecarConfig;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use rand::prelude::IndexedRandom;

#[derive(Clone, Debug)]
struct OutboundProxyClient {
    cli_opts: CliOpts,
    config: OtoroshiSidecarConfig,
    tls_client: Client<HttpsConnector<hyper::client::HttpConnector>>,
    mtls_clients: Cache<String, Client<HttpsConnector<hyper::client::HttpConnector>>>,
}

impl OutboundProxyClient {
    fn new(config: OtoroshiSidecarConfig, cli_opts: CliOpts) -> OutboundProxyClient {
        let tls_client: Client<HttpsConnector<hyper::client::HttpConnector>> = {
            let https = hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build();
            let client: Client<HttpsConnector<hyper::client::HttpConnector>> =
                Client::builder().build::<_, hyper::Body>(https);
            client
        };
        OutboundProxyClient {
            cli_opts,
            config,
            tls_client,
            mtls_clients: CacheBuilder::new(50)
                .time_to_live(std::time::Duration::from_secs(120))
                .build(),
        }
    }

    fn mtls_client(
        &self,
        id: String,
        client_cert: OtoroshiCertificate,
    ) -> Client<HttpsConnector<hyper::client::HttpConnector>> {
        match self.mtls_clients.get(&id) {
            Some(client) => client.clone(),
            None => {
                let client_tls: Client<HttpsConnector<hyper::client::HttpConnector>> = {
                    let mut root_store = RootCertStore::empty();
                    root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
                        OwnedTrustAnchor::from_subject_spki_name_constraints(
                            ta.subject,
                            ta.spki,
                            ta.name_constraints,
                        )
                    }));
                    let tls = rustls::ClientConfig::builder()
                        .with_safe_defaults()
                        .with_root_certificates(root_store)
                        .with_client_auth_cert(client_cert.certs(), client_cert.key())
                        .unwrap();
                    let https = hyper_rustls::HttpsConnectorBuilder::new()
                        .with_tls_config(tls)
                        .https_or_http()
                        .enable_http1()
                        .build();
                    let client: Client<HttpsConnector<hyper::client::HttpConnector>> =
                        Client::builder().build::<_, hyper::Body>(https);
                    client
                };
                self.mtls_clients.insert(id, client_tls.clone());
                client_tls.clone()
            }
        }
    }

    fn pass_through(&self, req: Request<Body>) -> ResponseFuture {
        self.tls_client.request(req)
    }

    fn request(
        &self,
        mut req: Request<Body>,
        client_cert: Option<(String, OtoroshiCertificate)>,
    ) -> ResponseFuture {
        let otoroshi_spec = self
            .config
            .clone()
            .spec
            .compute_otoroshi(self.cli_opts.clone());
        let otoroshi_routing_location = otoroshi_spec
            .routing_location
            .unwrap_or(OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation::default());
        let otoroshi_routing_location_clone = otoroshi_routing_location.clone();
        let uri_string = if self.config.clone().spec.kubernetes {
            let kube = otoroshi_routing_location_clone.kubernetes.unwrap_or(
                OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation::default(),
            );
            format!(
                "{}://{}:{}{}",
                if otoroshi_routing_location.tls {
                    "https"
                } else {
                    "http"
                },
                format!("{}.{}.svc.cluster.local", kube.service, kube.namespace),
                otoroshi_routing_location_clone.port,
                req.uri()
                    .path_and_query()
                    .map(|x| x.as_str())
                    .unwrap_or("/")
            )
        } else {
            let hostname: String = match otoroshi_routing_location_clone.ip_addresses {
                None => otoroshi_routing_location_clone.hostname.unwrap(),
                Some(vec) if vec.is_empty() => otoroshi_routing_location_clone.hostname.unwrap(),
                Some(vec) => vec
                    .choose(&mut rand::rng())
                    .unwrap()
                    .to_string(),
            };
            format!(
                "{}://{}:{}{}",
                if otoroshi_routing_location.tls {
                    "https"
                } else {
                    "http"
                },
                hostname,
                otoroshi_routing_location_clone.port,
                req.uri()
                    .path_and_query()
                    .map(|x| x.as_str())
                    .unwrap_or("/")
            )
        };
        let uri = uri_string.parse().unwrap();
        *req.uri_mut() = uri;
        match client_cert {
            None => self.tls_client.request(req),
            Some((id, client_cert)) => self.mtls_client(id, client_cert).request(req),
        }
    }
}

pub struct OutboundProxy {}

impl OutboundProxy {
    pub fn start(
        cli_opts: CliOpts,
        config: OtoroshiSidecarConfig,
        cache: Arc<SidecarCache>,
    ) -> impl Future<Output = std::result::Result<(), hyper::Error>> {
        let in_addr: SocketAddr = SocketAddr::new(
            "127.0.0.1".parse().unwrap(),
            config.spec.outbounds.port.unwrap_or(15001),
        );
        let client = OutboundProxyClient::new(config.clone(), cli_opts);
        let server = Server::bind(&in_addr).serve(MakeSvc {
            client,
            config,
            cache,
        });
        info!(target: "outbound_proxy", "listening on http://{}", in_addr);
        server
    }
}

struct Svc {
    client: OutboundProxyClient,
    config: OtoroshiSidecarConfig,
    cache: Arc<SidecarCache>,
}

impl Svc {
    fn get_additional_headers(&self, host: String) -> HashMap<http::HeaderName, http::HeaderValue> {
        match self.config.spec.outbounds.outbounds.get(&host) {
            Some(outbound) => match outbound.clone().apikey {
                Some(apikey_config) if apikey_config.enabled => {
                    match self.cache.get_apikey_by_id(apikey_config.apikey_id.clone()) {
                        Some(apikey) => {
                            let mut headers = HashMap::new();
                            headers.insert(
                                http::HeaderName::from_static("otoroshi-client-id"),
                                http::HeaderValue::from_str(apikey.clientId.as_str()).unwrap(),
                            );
                            headers.insert(
                                http::HeaderName::from_static("otoroshi-client-secret"),
                                http::HeaderValue::from_str(apikey.clientSecret.as_str()).unwrap(),
                            );
                            headers
                        }
                        _ => {
                            error!(
                                "apikey with id '{}' does not exists",
                                apikey_config.apikey_id.clone()
                            );
                            HashMap::new()
                        }
                    }
                }
                _ => HashMap::new(),
            },
            _ => HashMap::new(),
        }
    }
    fn get_client_cert(&self, host: String) -> Option<(String, super::cache::OtoroshiCertificate)> {
        match self.config.spec.outbounds.outbounds.get(&host) {
            Some(outbound) => match outbound.clone().mtls {
                Some(tls_config) if tls_config.enabled => {
                    match self.cache.get_cert_by_id(tls_config.client_cert_id.clone()) {
                        None => {
                            error!(
                                "certificate with id '{}' does not exists",
                                tls_config.client_cert_id.clone()
                            );
                            None
                        }
                        Some(cert) => Some((tls_config.client_cert_id.clone(), cert)),
                    }
                }
                _ => None,
            },
            _ => None,
        }
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
        //let host = headers.get("Host").unwrap();
        let default_value = http::HeaderValue::from_str("localhost").unwrap();
        let authority_value = http::HeaderValue::from_str(
            req.uri()
                .authority()
                .map(|a| a.host())
                .unwrap_or("localhost"),
        )
        .unwrap();
        let host = headers
            .get("Host")
            .or(headers.get(":authority"))
            .or(Some(&authority_value))
            .unwrap_or(&default_value);
        let host_str = host.to_str().unwrap().to_string();
        if host_str.ends_with(
            self.config
                .spec
                .dns_domain
                .clone()
                .unwrap_or(".otoroshi.mesh".to_string())
                .as_str(),
        ) {
            let client_cert = self.get_client_cert(host_str.clone());
            let h_map = http::HeaderMap::from_iter(self.get_additional_headers(host_str.clone()));
            info!(target: "outbound_proxy", "{} http://{}{}, apikey: {}, client_cert: {}", req.method(), host_str, req.uri(), if h_map.is_empty() { "no" } else { "yes" }, if client_cert.is_none() { "no" } else { "yes" });
            headers.insert("Host", host.clone());
            headers.extend(h_map);
            *req.headers_mut() = headers;
            self.client.request(req, client_cert)
        } else {
            self.client.pass_through(req)
        }
    }
}

struct MakeSvc {
    client: OutboundProxyClient,
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

    fn call(&mut self, _: T) -> Self::Future {
        let client = self.client.clone();
        let config = self.config.clone();
        let cache = self.cache.clone();
        let future = async move {
            Ok(Svc {
                client,
                config,
                cache,
            })
        };
        Box::pin(future)
    }
}
