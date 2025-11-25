use std::sync::Arc;

use moka::sync::Cache;
use moka::sync::CacheBuilder;
use rustls::Certificate;
use rustls::PrivateKey;
use serde::Deserialize;
use serde::Serialize;

use crate::cli::cliopts::CliOpts;
use crate::cli::config::OtoroshiCtlConfigSpecClusterClientCert;
use crate::sidecar::config::OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation;
use crate::utils::otoroshi::OtoroshiConnectionConfig;

use super::config::OtoroshiSidecarConfig;
use super::config::OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials;
use super::config::OtoroshiSidecarConfigSpecOtoroshiSettingsLocation;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct OtoroshiChallengePluginAlgo {
    pub secret: String,
    pub size: i32,
    pub base64: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct OtoroshiChallengePlugin {
    pub version: String,
    pub ttl: i32,
    pub request_header_name: Option<String>,
    pub response_header_name: Option<String>,
    pub state_resp_leeway: i32,
    pub algo_to_backend: OtoroshiChallengePluginAlgo,
    pub algo_from_backend: OtoroshiChallengePluginAlgo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct OtoroshiPlugins {
    pub plugin: String,
    pub config: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct OtoroshiRoute {
    pub id: String,
    pub name: String,
    pub plugins: Vec<OtoroshiPlugins>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct OtoroshiCertificate {
    pub id: String,
    pub name: String,
    pub chain: String,
    pub privateKey: String,
    pub subject: String,
}

impl OtoroshiCertificate {
    pub fn certs(&self) -> Vec<Certificate> {
        let streader = stringreader::StringReader::new(self.chain.as_str());
        let mut bufreader = std::io::BufReader::new(streader);
        let certs = rustls_pemfile::certs(&mut bufreader).unwrap();
        certs.into_iter().map(Certificate).collect()
    }
    pub fn key(&self) -> PrivateKey {
        let streader = stringreader::StringReader::new(self.privateKey.as_str());
        let mut bufreader = std::io::BufReader::new(streader);
        let keys_raw = rustls_pemfile::pkcs8_private_keys(&mut bufreader).unwrap();
        let keys: Vec<PrivateKey> = keys_raw.into_iter().map(PrivateKey).collect();
        keys.first().unwrap().to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct OtoroshiApikey {
    pub clientId: String,
    pub clientSecret: String,
    pub clientName: Option<String>,
}

pub struct SidecarCache {
    cli_opts: CliOpts,
    sidecar_config: OtoroshiSidecarConfig,
    routes_cache: Cache<String, Arc<OtoroshiRoute>>,
    certificates_cache: Cache<String, Arc<OtoroshiCertificate>>,
    apikeys_cache: Cache<String, Arc<OtoroshiApikey>>,
}

impl SidecarCache {

    pub fn new(sidecar_config: OtoroshiSidecarConfig, cli_opts: CliOpts) -> SidecarCache {
        SidecarCache {
            cli_opts,
            sidecar_config,
            routes_cache: CacheBuilder::new(50)
                .time_to_live(std::time::Duration::from_secs(120))
                .build(),
            certificates_cache: CacheBuilder::new(500)
                .time_to_live(std::time::Duration::from_secs(120))
                .build(),
            apikeys_cache: CacheBuilder::new(2000)
                .time_to_live(std::time::Duration::from_secs(120))
                .build()
        }
    }

    fn otoroshi_config(&self) -> OtoroshiConnectionConfig {
        let otoroshi_settings = self.sidecar_config.spec.compute_otoroshi(self.cli_opts.clone());
        let otoroshi_location = otoroshi_settings.location.unwrap_or(OtoroshiSidecarConfigSpecOtoroshiSettingsLocation::default());
        let otoroshi_routing_location = otoroshi_settings.routing_location;
        let otoroshi_credentials = otoroshi_settings.credentials.unwrap_or(OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials::default());
        let hostname_with_port: String = if self.sidecar_config.clone().spec.kubernetes {
            let kube = otoroshi_location.clone().kubernetes.unwrap_or(OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation::default());
            format!("{}.{}.svc.cluster.local:{}", kube.service, kube.namespace, otoroshi_location.port)
        } else {
            format!("{}:{}", otoroshi_location.clone().hostname.unwrap(), otoroshi_location.clone().port)
        };
        let hostname: String = if self.sidecar_config.clone().spec.kubernetes {
            let kube = otoroshi_location.clone().kubernetes.unwrap_or(OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation::default());
            format!("{}.{}.svc.cluster.local", kube.service, kube.namespace)
        } else {
            otoroshi_location.clone().hostname.unwrap().to_string()
        };
        
        OtoroshiConnectionConfig {
            host: hostname_with_port,
            hostname,
            port: otoroshi_location.port,
            ip_addresses: otoroshi_location.ip_addresses,
            cid: otoroshi_credentials.client_id,
            csec: otoroshi_credentials.client_secret,
            chealth: None, 
            tls: otoroshi_location.tls,
            routing_hostname: otoroshi_routing_location.clone().and_then(|i| i.hostname),
            routing_port: otoroshi_routing_location.clone().map(|i| i.port),
            routing_tls: otoroshi_routing_location.clone().map(|i| i.tls),
            routing_ip_addresses: otoroshi_routing_location.clone().and_then(|i| i.ip_addresses),
            mtls: otoroshi_settings.client_cert.map(|cert| {
                OtoroshiCtlConfigSpecClusterClientCert {
                    cert_location: cert.cert_location,
                    cert_value: cert.cert_value,
                    key_location: cert.key_location,
                    key_value: cert.key_value,
                    ca_location: cert.ca_location,
                    ca_value: cert.ca_value,
                }
            })
        }
    }

    pub async fn update(&self) {
        let inbounds = &self.sidecar_config.spec.inbound;
        let outbounds = &self.sidecar_config.spec.outbounds.outbounds;
        for tls in inbounds.tls.clone().into_iter() {
            let _ = self.async_get_cert_by_id(tls.cert_id).await;
        }
        for mtls in inbounds.mtls.clone().into_iter() {
            let _ = self.async_get_cert_by_id(mtls.ca_cert_id).await;
        }
        for protocol in inbounds.otoroshi_protocol.clone().into_iter() {
            for route_id in protocol.route_id.clone().into_iter() {
                let _ = self.async_get_route_by_id(route_id).await;
            }
        }
        for outbound in outbounds.iter() {
            for apikey in outbound.1.apikey.clone().into_iter() {
                let _ = self.async_get_apikey_by_id(apikey.apikey_id).await;
            }
            for mtls in outbound.1.mtls.clone().into_iter() {
                let _ = self.async_get_cert_by_id(mtls.client_cert_id).await;
            }
        }
    }

    pub async fn async_get_cert_by_id(&self, id: String) -> Option<OtoroshiCertificate> {
        let key = id.clone();
        match self.certificates_cache.get(&key) {
            Some(cert) => Some(cert.as_ref().clone()),
            None => {
                let config = self.otoroshi_config();
                match crate::utils::otoroshi::Otoroshi::get_one_resource_with_config("certificates".to_string(), id, config).await {
                    None => None,
                    Some(resp) => {
                        let cert: OtoroshiCertificate = serde_json::from_value(resp.body).unwrap();
                        self.certificates_cache.insert(key, Arc::new(cert.clone()));
                        Some(cert)
                    },
                }
            }
        }
    }

    pub fn wait_and_get_cert_by_id(&self, id: String, max: i32) -> OtoroshiCertificate {
        let cert: OtoroshiCertificate;
        let mut count = 0;
        loop {
            if count > max {
                panic!("too much retries ({}) to certificate with id '{}'", max, id);
            }
            match self.get_cert_by_id(id.clone()) {
                None => std::thread::sleep(std::time::Duration::from_secs(1)),
                Some(c) => {
                    cert = c;
                    break;
                }
            }
            count += 1;
        }
        cert
    }

    pub fn get_cert_by_id(&self, id: String) -> Option<OtoroshiCertificate> {
        let key = id.clone();
        self.certificates_cache.get(&key).map(|cert| cert.as_ref().clone())
    }


    pub async fn async_get_route_by_id(&self, id: String) -> Option<OtoroshiRoute> {
        let key = id.clone();
        match self.routes_cache.get(&key) {
            Some(route) => Some(route.as_ref().clone()),
            None => {
                let config = self.otoroshi_config();
                match crate::utils::otoroshi::Otoroshi::get_one_resource_with_config("routes".to_string(), id, config).await {
                    None => None,
                    Some(resp) => {
                        let route: OtoroshiRoute = serde_json::from_value(resp.body).unwrap();
                        self.routes_cache.insert(key, Arc::new(route.clone()));
                        Some(route)
                    },
                }
            }
        }
    }

    pub fn get_route_by_id(&self, id: String) -> Option<OtoroshiRoute> {
        let key = id.clone();
        self.routes_cache.get(&key).map(|route| route.as_ref().clone())
    }

    pub fn wait_and_get_route_by_id(&self, id: String, max: i32) -> OtoroshiRoute {
        let route: OtoroshiRoute;
        let mut count = 0;
        loop {
            if count > max {
                panic!("too much retries ({}) to route with id '{}'", max, id);
            }
            match self.get_route_by_id(id.clone()) {
                None => std::thread::sleep(std::time::Duration::from_secs(1)),
                Some(c) => {
                    route = c;
                    break;
                }
            }
            count += 1;
        }
        route
    }

    pub async fn async_get_apikey_by_id(&self, id: String) -> Option<OtoroshiApikey> {
        let key = id.clone();
        match self.apikeys_cache.get(&key) {
            Some(apikey) => Some(apikey.as_ref().clone()),
            None => {
                let config = self.otoroshi_config();
                match crate::utils::otoroshi::Otoroshi::get_one_resource_with_config("apikeys".to_string(), id, config).await {
                    None => None,
                    Some(resp) => {
                        let apikey: OtoroshiApikey = serde_json::from_value(resp.body).unwrap();
                        self.apikeys_cache.insert(key, Arc::new(apikey.clone()));
                        Some(apikey)
                    },
                }
            }
        }
    }

    pub fn get_apikey_by_id(&self, id: String) -> Option<OtoroshiApikey> {
        let key = id.clone();
        self.apikeys_cache.get(&key).map(|apikey| apikey.as_ref().clone())
    }

    pub fn wait_and_get_apikey_by_id(&self, id: String, max: i32) -> OtoroshiApikey {
        let apikey: OtoroshiApikey;
        let mut count = 0;
        loop {
            if count > max {
                panic!("too much retries ({}) to apikey with id '{}'", max, id);
            }
            match self.get_apikey_by_id(id.clone()) {
                None => std::thread::sleep(std::time::Duration::from_secs(1)),
                Some(c) => {
                    apikey = c;
                    break;
                }
            }
            count += 1;
        }
        apikey
    }

}