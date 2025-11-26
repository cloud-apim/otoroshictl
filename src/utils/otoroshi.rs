use std::collections::HashMap;

use hyper::{Client, Method, Request};
use hyper_rustls::HttpsConnector;
use rustls::{OwnedTrustAnchor, RootCertStore};
use serde::{Deserialize, Serialize};

use base64::{Engine as _, engine::general_purpose};

use crate::cli::cliopts::CliOpts;
use crate::cli::commands::{
    entities::OtoroshExposedResources, health::OtoroshiHealth, infos::OtoroshiInfos,
    metrics::OtoroshiMetrics, version::OtoroshiVersion,
};
use crate::cli::config::OtoroshiCtlConfigSpecClusterClientCert;
use crate::cli_stderr_printline;
use crate::commands::entities::OtoroshExposedResource;
use crate::sidecar::cache::OtoroshiCertificate;
use crate::tunnels::remote::RemoteTunnelCommandOpts;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiApiSingleResult {
    pub id: String,
    pub body: serde_json::Value,
}

pub struct OtoroshiApiMultiResult {
    pub body: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiConnectionConfig {
    /// the otoroshi api hostname
    pub host: String,
    pub hostname: String,
    pub port: u16,
    pub ip_addresses: Option<Vec<String>>,
    /// the otoroshi client-id
    pub cid: String,
    /// the otoroshi client-secret
    pub csec: String,
    /// the otoroshi health key
    pub chealth: Option<String>,
    /// enable tls want mode
    pub tls: bool,
    /// use mtls to contact otoroshi
    pub mtls: Option<OtoroshiCtlConfigSpecClusterClientCert>,
    pub routing_hostname: Option<String>,
    pub routing_port: Option<u16>,
    pub routing_tls: Option<bool>,
    pub routing_ip_addresses: Option<Vec<String>>,
}

pub struct OtoroshiResponse {
    pub status: u16,
    pub body_bytes: hyper::body::Bytes,
    pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshRemoteTunnelsInfos {
    pub domain: String,
    pub scheme: String,
    pub exposed_port_http: i16,
    pub exposed_port_https: i16,
}

pub struct Otoroshi {}

impl Otoroshi {
    pub async fn get_connection_config(opts: CliOpts) -> OtoroshiConnectionConfig {
        crate::cli::config::OtoroshiCtlConfig::get_current_config(opts)
            .await
            .to_connection_config()
    }

    pub async fn otoroshi_call(
        method: hyper::Method,
        path: &str,
        accept: Option<String>,
        body: Option<hyper::Body>,
        content_type: Option<String>,
        opts: OtoroshiConnectionConfig,
    ) -> OtoroshiResponse {
        let client_id = opts.cid;
        let client_secret = opts.csec;
        let scheme = if opts.tls { "https" } else { "http" };
        let host = opts.host;
        let mut uri: String = format!("{}://{}{}", scheme, host, path);
        if (uri.ends_with("monitoring/health") || uri.ends_with("monitoring/metrics"))
            && opts.chealth.is_some()
        {
            if uri.contains("?") {
                uri = format!("{}&access_key={}", uri, opts.chealth.unwrap());
            } else {
                uri = format!("{}?access_key={}", uri, opts.chealth.unwrap());
            }
        }
        debug!("calling {} {}", method, uri);
        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .header("host", host.clone())
            .header("accept", accept.unwrap_or("application/json".to_string()))
            .header(
                "Authorization",
                format!(
                    "Basic {}",
                    general_purpose::STANDARD_NO_PAD
                        .encode(format!("{}:{}", client_id, client_secret))
                ),
            );
        if body.is_some() && content_type.is_some() {
            // builder = builder.header("Content-Type", "application/json")
            builder = builder.header("Content-Type", content_type.unwrap());
        }
        let req: Request<hyper::Body> = builder.body(body.unwrap_or(hyper::Body::empty())).unwrap();
        let resp_result = if opts.tls {
            if opts.mtls.is_some() {
                let mtls = opts.mtls.unwrap();
                let client_cert: OtoroshiCertificate =
                    match (mtls.ca_location, mtls.cert_location, mtls.key_location) {
                        (Some(ca_location), Some(cert_location), Some(key_location)) => {
                            OtoroshiCertificate {
                                id: "tmp".to_string(),
                                name: "tmp".to_string(),
                                chain: format!(
                                    "{}\n\n{}",
                                    std::fs::read_to_string(cert_location).unwrap(),
                                    std::fs::read_to_string(ca_location).unwrap()
                                ),
                                privateKey: std::fs::read_to_string(key_location).unwrap(),
                                subject: "tmp".to_string(),
                            }
                        }
                        _ => match (mtls.ca_value, mtls.cert_value, mtls.key_value) {
                            (Some(ca_location), Some(cert_location), Some(key_location)) => {
                                OtoroshiCertificate {
                                    id: "tmp".to_string(),
                                    name: "tmp".to_string(),
                                    chain: format!("{}\n\n{}", cert_location, ca_location),
                                    privateKey: key_location,
                                    subject: "tmp".to_string(),
                                }
                            }
                            _ => {
                                cli_stderr_printline!("bad client cert options");
                                std::process::exit(-1);
                            }
                        },
                    };
                let client: Client<HttpsConnector<hyper::client::HttpConnector>> = {
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
                client.request(req).await
            } else {
                let https = hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .build();
                let client: Client<HttpsConnector<hyper::client::HttpConnector>> =
                    Client::builder().build::<_, hyper::Body>(https);
                client.request(req).await
            }
        } else {
            let client = Client::new();
            client.request(req).await
        };
        match resp_result {
            Err(err) => {
                cli_stderr_printline!("error while calling otoroshi api: \n\n{}", err);
                std::process::exit(-1)
            }
            Ok(resp) => {
                let status = resp.status().as_u16();
                let mut headers = HashMap::new();
                for header in resp.headers().into_iter() {
                    headers.insert(
                        header.0.as_str().to_string(),
                        header.1.to_str().unwrap().to_string(),
                    );
                }
                let body_bytes = hyper::body::to_bytes(resp).await.unwrap();
                // debug!("status: {}, body: {:?}", status, body_bytes);
                OtoroshiResponse {
                    status,
                    headers,
                    body_bytes,
                }
            }
        }
    }

    async fn get_otoroshi_resource(
        path: &str,
        accept: Option<String>,
        opts: OtoroshiConnectionConfig,
    ) -> Option<hyper::body::Bytes> {
        let response = Self::otoroshi_call(
            Method::GET,
            path,
            accept,
            None,
            Some("application/json".to_string()),
            opts,
        )
        .await;
        if response.status == 200 || response.status == 201 {
            Some(response.body_bytes)
        } else {
            println!(
                "status: {}, body: {:?}",
                response.status, response.body_bytes
            );
            None
        }
    }

    pub async fn get_one_resource_with_config(
        entity: OtoroshExposedResource,
        id: String,
        config: OtoroshiConnectionConfig,
    ) -> Option<OtoroshiApiSingleResult> {
        match Self::get_otoroshi_resource(
            format!(
                "/apis/{}/{}/{}/{}",
                entity.group, entity.version.name, entity.plural_name, id
            )
            .as_str(),
            None,
            config,
        )
        .await
        {
            None => None,
            Some(body_bytes) => match serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                Ok(infos) => Some(OtoroshiApiSingleResult { id, body: infos }),
                Err(e) => {
                    debug!("parse error: {}", e);
                    None
                }
            },
        }
    }

    pub async fn get_one_resource(
        entity: OtoroshExposedResource,
        id: String,
        opts: CliOpts,
    ) -> Option<OtoroshiApiSingleResult> {
        let config = Self::get_connection_config(opts).await;
        match Self::get_otoroshi_resource(
            format!(
                "/apis/{}/{}/{}/{}",
                entity.group, entity.version.name, entity.plural_name, id
            )
            .as_str(),
            None,
            config,
        )
        .await
        {
            None => None,
            Some(body_bytes) => match serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                Ok(infos) => Some(OtoroshiApiSingleResult { id, body: infos }),
                Err(e) => {
                    debug!("parse error: {}", e);
                    None
                }
            },
        }
    }

    pub async fn get_global_config(opts: CliOpts) -> Option<OtoroshiApiSingleResult> {
        let config = Self::get_connection_config(opts).await;
        match Self::get_otoroshi_resource("/api/globalconfig", None, config).await {
            None => None,
            Some(body_bytes) => match serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                Ok(global_config) => Some(OtoroshiApiSingleResult {
                    id: "singleton".to_string(),
                    body: global_config,
                }),
                Err(e) => {
                    debug!("parse error: {}", e);
                    None
                }
            },
        }
    }

    pub async fn update_global_config(opts: CliOpts, body: String) -> bool {
        let config = Self::get_connection_config(opts).await;
        match Self::otoroshi_call(
            Method::PUT,
            "/api/globalconfig",
            None,
            Some(hyper::Body::from(body)),
            Some("application/json".to_string()),
            config,
        )
        .await
        {
            resp if resp.status == 200 || resp.status == 201 => true,
            _ => false,
        }
    }

    pub async fn delete_one_resource(
        entity: OtoroshExposedResource,
        id: String,
        opts: CliOpts,
    ) -> bool {
        let config: OtoroshiConnectionConfig = Self::get_connection_config(opts).await;
        match Self::otoroshi_call(
            Method::DELETE,
            format!(
                "/apis/{}/{}/{}/{}",
                entity.group, entity.version.name, entity.plural_name, id
            )
            .as_str(),
            None,
            None,
            Some("application/json".to_string()),
            config,
        )
        .await
        {
            resp if resp.status == 200 => true,
            _ => false,
        }
    }

    pub async fn upsert_one_resource(
        entity: OtoroshExposedResource,
        id: String,
        body: String,
        opts: CliOpts,
    ) -> bool {
        let config: OtoroshiConnectionConfig = Self::get_connection_config(opts).await;
        match Self::otoroshi_call(
            Method::POST,
            format!(
                "/apis/{}/{}/{}/{}",
                entity.group, entity.version.name, entity.plural_name, id
            )
            .as_str(),
            None,
            Some(hyper::Body::from(body)),
            Some("application/json".to_string()),
            config,
        )
        .await
        {
            resp if resp.status == 200 || resp.status == 201 => true,
            _ => false,
        }
    }

    pub async fn upsert_one_resource_with_content_type(
        entity: OtoroshExposedResource,
        id: String,
        body: String,
        content_type: String,
        opts: CliOpts,
    ) -> bool {
        let config: OtoroshiConnectionConfig = Self::get_connection_config(opts).await;
        match Self::otoroshi_call(
            Method::POST,
            format!(
                "/apis/{}/{}/{}/{}",
                entity.group, entity.version.name, entity.plural_name, id
            )
            .as_str(),
            None,
            Some(hyper::Body::from(body)),
            Some(content_type),
            config,
        )
        .await
        {
            resp if resp.status == 200 || resp.status == 201 => true,
            _ => false,
        }
    }

    pub async fn create_one_resource_with_content_type(
        entity: OtoroshExposedResource,
        body: String,
        content_type: String,
        opts: CliOpts,
    ) -> bool {
        let config: OtoroshiConnectionConfig = Self::get_connection_config(opts).await;
        match Self::otoroshi_call(
            Method::POST,
            format!(
                "/apis/{}/{}/{}",
                entity.group, entity.version.name, entity.plural_name
            )
            .as_str(),
            None,
            Some(hyper::Body::from(body)),
            Some(content_type),
            config,
        )
        .await
        {
            resp if resp.status == 200 || resp.status == 201 => true,
            _ => false,
        }
    }

    pub async fn get_resource_template(
        entity: OtoroshExposedResource,
        opts: CliOpts,
    ) -> Option<serde_json::Value> {
        let config: OtoroshiConnectionConfig = Self::get_connection_config(opts).await;
        match Self::otoroshi_call(
            Method::GET,
            format!(
                "/apis/{}/{}/{}/_template",
                entity.group, entity.version.name, entity.plural_name
            )
            .as_str(),
            None,
            None,
            Some("application/json".to_string()),
            config,
        )
        .await
        {
            resp if resp.status == 200 => {
                //println!("body: {:?}", resp.body_bytes);
                match serde_json::from_slice::<serde_json::Value>(&resp.body_bytes) {
                    Ok(payload) => Some(payload),
                    Err(e) => {
                        debug!("parse error: {}", e);
                        None
                    }
                }
            }
            _ => None,
        }
    }

    pub async fn get_resources(
        entity: OtoroshExposedResource,
        page: u32,
        page_size: u32,
        filter: Vec<String>,
        opts: CliOpts,
    ) -> Option<OtoroshiApiMultiResult> {
        let config = Self::get_connection_config(opts).await;
        let filtering: String = if filter.is_empty() {
            "".to_string()
        } else {
            let terms = filter
                .into_iter()
                .flat_map(|item| {
                    item.split(",")
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<String>>()
                .into_iter()
                .map(|item: String| {
                    if item.starts_with("filter.") {
                        item.to_string()
                    } else {
                        format!("filter.{}", item).to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("&");
            format!("&{}", terms).to_string()
        };
        match Self::get_otoroshi_resource(
            format!(
                "/apis/{}/{}/{}?page={}&pageSize={}{}",
                entity.group, entity.version.name, entity.plural_name, page, page_size, filtering
            )
            .as_str(),
            None,
            config,
        )
        .await
        {
            None => None,
            Some(body_bytes) => match serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                Ok(infos) => Some(OtoroshiApiMultiResult {
                    body: infos.as_array().unwrap().to_vec(),
                }),
                Err(e) => {
                    debug!("parse error: {}", e);
                    None
                }
            },
        }
    }

    pub async fn get_health(opts: CliOpts) -> Option<OtoroshiHealth> {
        let config: OtoroshiConnectionConfig = Self::get_connection_config(opts).await;
        match Self::get_otoroshi_resource("/.well-known/otoroshi/monitoring/health", None, config)
            .await
        {
            None => None,
            Some(body_bytes) => match serde_json::from_slice::<OtoroshiHealth>(&body_bytes) {
                Ok(infos) => Some(infos),
                Err(e) => {
                    debug!("parse error: {}", e);
                    None
                }
            },
        }
    }

    pub async fn get_metrics(opts: CliOpts) -> Option<OtoroshiMetrics> {
        let config = Self::get_connection_config(opts).await;
        match Self::get_otoroshi_resource("/.well-known/otoroshi/monitoring/metrics", None, config)
            .await
        {
            None => None,
            Some(body_bytes) => match serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                Ok(infos) => Some(OtoroshiMetrics { body: infos }),
                Err(e) => {
                    debug!("parse error: {}", e);
                    None
                }
            },
        }
    }

    pub async fn get_version(opts: CliOpts) -> Option<OtoroshiVersion> {
        let config = Self::get_connection_config(opts).await;
        match Self::get_otoroshi_resource("/api/version", None, config).await {
            None => None,
            Some(body_bytes) => match serde_json::from_slice::<OtoroshiVersion>(&body_bytes) {
                Ok(infos) => Some(infos),
                Err(e) => {
                    debug!("parse error: {}", e);
                    None
                }
            },
        }
    }

    pub async fn get_infos(opts: CliOpts) -> Option<OtoroshiInfos> {
        let config = Self::get_connection_config(opts).await;
        match Self::get_otoroshi_resource("/api/infos", None, config).await {
            None => None,
            Some(body_bytes) => match serde_json::from_slice::<OtoroshiInfos>(&body_bytes) {
                Ok(infos) => Some(infos),
                Err(e) => {
                    debug!("parse error: {}", e);
                    None
                }
            },
        }
    }

    pub async fn get_export_json(
        accept: Option<String>,
        opts: CliOpts,
    ) -> Option<hyper::body::Bytes> {
        let config = Self::get_connection_config(opts).await;
        Self::get_otoroshi_resource("/api/otoroshi.json", accept, config).await
    }

    pub async fn get_exposed_resources(opts: CliOpts) -> Option<OtoroshExposedResources> {
        let config = Self::get_connection_config(opts).await;
        match Self::get_otoroshi_resource("/apis/entities", None, config).await {
            None => None,
            Some(body_bytes) => {
                match serde_json::from_slice::<OtoroshExposedResources>(&body_bytes) {
                    Ok(infos) => Some(infos),
                    Err(e) => {
                        debug!("parse error: {}", e);
                        None
                    }
                }
            }
        }
    }

    pub async fn get_remote_tunnels_infos(opts: CliOpts) -> Option<OtoroshRemoteTunnelsInfos> {
        let config: OtoroshiConnectionConfig = Self::get_connection_config(opts).await;
        match Self::get_otoroshi_resource("/api/tunnels/infos", None, config).await {
            None => None,
            Some(body_bytes) => {
                match serde_json::from_slice::<OtoroshRemoteTunnelsInfos>(&body_bytes) {
                    Ok(infos) => Some(infos),
                    Err(e) => {
                        debug!("parse error: {}", e);
                        None
                    }
                }
            }
        }
    }

    pub async fn maybe_expose_local_process(
        tunnel_opts: RemoteTunnelCommandOpts,
        opts: CliOpts,
        infos: OtoroshRemoteTunnelsInfos,
    ) -> String {
        let cloned_opts = opts.clone();
        let config = Self::get_connection_config(opts).await;
        let resp = Self::otoroshi_call(
            hyper::Method::GET,
            format!("/api/routes/route_{}", tunnel_opts.tunnel).as_str(),
            Some("application/json".to_string()),
            None,
            None,
            config,
        )
        .await;
        if resp.status == 200 {
            debug!("route already exists ...");
            let body_bytes = resp.body_bytes;
            let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
            let domain = json
                .get("frontend")
                .unwrap()
                .as_object()
                .unwrap()
                .get("domains")
                .unwrap()
                .as_array()
                .unwrap()
                .first()
                .unwrap()
                .as_str()
                .unwrap();
            String::from(domain)
        } else {
            debug!("creating route ...");
            Self::expose_local_process(tunnel_opts.clone(), cloned_opts, infos.clone()).await
        }
    }

    pub async fn expose_local_process(
        tunnel_opts: RemoteTunnelCommandOpts,
        opts: CliOpts,
        infos: OtoroshRemoteTunnelsInfos,
    ) -> String {
        let config = Self::get_connection_config(opts).await;
        let tunnel_id = tunnel_opts.tunnel;
        let local_host = tunnel_opts.local_host;
        let local_port = tunnel_opts.local_port;
        let local_tls = tunnel_opts.local_tls;
        let local_port_str = format!("{}", local_port);
        let local_tls_str = format!("{}", local_tls);
        let id = uuid::Uuid::new_v4().to_string();
        let domain = format!(
            "{}.{}",
            tunnel_opts.remote_subdomain.unwrap_or(id + "-tunnel"),
            tunnel_opts.remote_domain.unwrap_or(infos.domain)
        );
        let json = r###"{
            "id": "route_$tunnel_id",
            "name": "exposed-cli-tunnel-$tunnel_id",
            "description": "exposed-cli-tunnel-$tunnel_id",
            "tags": [],
            "metadata": {},
            "enabled": true,
            "debug_flow": false,
            "export_reporting": false,
            "capture": false,
            "groups": [
              "default"
            ],
            "frontend": {
              "domains": [
                "$domain"
              ],
              "strip_path": true,
              "exact": false,
              "headers": {},
              "query": {},
              "methods": []
            },
            "backend": {
              "targets": [
                {
                  "id": "target_1",
                  "hostname": "$local_host",
                  "port": $local_port,
                  "tls": $local_tls,
                  "weight": 1,
                  "predicate": {
                    "type": "AlwaysMatch"
                  },
                  "protocol": "HTTP/1.1",
                  "ip_address": null
                }
              ],
              "target_refs": [],
              "root": "/",
              "rewrite": false,
              "load_balancing": {
                "type": "RoundRobin"
              },
              "health_check": null
            },
            "backend_ref": null,
            "plugins": [
              {
                "enabled": true,
                "debug": false,
                "plugin": "cp:otoroshi.next.tunnel.TunnelPlugin",
                "include": [],
                "exclude": [],
                "config": {
                  "tunnel_id": "$tunnel_id"
                },
                "plugin_index": { }
              }
            ]
          }"###
            .replace("$tunnel_id", tunnel_id.as_str())
            .replace("$domain", domain.as_str())
            .replace("$local_host", local_host.as_str())
            .replace("$local_port", local_port_str.as_str())
            .replace("$local_tls", local_tls_str.as_str());
        let resp = Self::otoroshi_call(
            hyper::Method::POST,
            "/api/routes",
            Some("application/json".to_string()),
            Some(hyper::Body::from(json)),
            Some("application/json".to_string()),
            config,
        )
        .await;
        debug!("route created ! - {}", resp.status);
        domain
    }
}
