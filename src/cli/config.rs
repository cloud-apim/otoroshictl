use std::{collections::HashMap, vec};
use serde::{Serialize, Deserialize};
use crate::{cli::cliopts::CliOpts, cli_stderr_printline, utils::http::HttpContentKind};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct OtoroshiCtlConfigSpecUser {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub health_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct OtoroshiCtlConfigSpecContext {
    pub name: String,
    pub cluster: String,
    pub user: String,
    pub cloud_apim: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiCtlConfigSpecClusterClientCert {
    pub cert_location: Option<String>,
    pub cert_value: Option<String>,
    pub key_location: Option<String>,
    pub key_value: Option<String>,
    pub ca_location: Option<String>,
    pub ca_value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct OtoroshiCtlConfigSpecCluster {
    pub name: String,
    pub hostname: String,
    pub ip_addresses: Option<Vec<String>>,
    pub port: u16,
    pub tls: bool,
    pub client_cert: Option<OtoroshiCtlConfigSpecClusterClientCert>,
    pub routing_hostname: Option<String>,
    pub routing_port: Option<u16>,
    pub routing_tls: Option<bool>,
    pub routing_ip_addresses: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct OtoroshiCtlConfigCloudApim {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct OtoroshiCtlConfig {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub metadata: HashMap<String, String>,
    pub cloud_apim: Option<OtoroshiCtlConfigCloudApim>,
    pub users: Vec<OtoroshiCtlConfigSpecUser>,
    pub contexts: Vec<OtoroshiCtlConfigSpecContext>,
    pub clusters: Vec<OtoroshiCtlConfigSpecCluster>,
    pub current_context: String,
}

impl OtoroshiCtlConfig {

    pub fn empty() -> OtoroshiCtlConfig {
        OtoroshiCtlConfig {
            api_version: "v1".to_string(),
            kind: "OtoroshiCtlConfig".to_string(),
            metadata: HashMap::new(),
            users: Vec::new(),
            contexts: Vec::new(),
            clusters: Vec::new(),
            cloud_apim: None,
            current_context: "none".to_string(),
        }
    }

    pub async fn read_from(path: &String) -> Result<OtoroshiCtlConfig, String> {
        if path.starts_with("http://") {
            Self::read_from_url(path, false).await
        } else if path.starts_with("https://") {
            Self::read_from_url(path, false).await
        } else {
            Self::read_from_file(path)
        }
    }

    pub fn read_from_file(file: &String) -> Result<OtoroshiCtlConfig, String> {
        if file.ends_with(".json") {
            serde_json::from_str::<OtoroshiCtlConfig>(std::fs::read_to_string(file).unwrap().as_str()).map_err(|e| e.to_string())
        } else {
            serde_yaml::from_str::<OtoroshiCtlConfig>(std::fs::read_to_string(file).unwrap().as_str()).map_err(|e| e.to_string())
        }
    }

    pub fn read_from_string(content: &String) -> Result<OtoroshiCtlConfig, String> {
        serde_yaml::from_str::<OtoroshiCtlConfig>(&content).map_err(|e| e.to_string())
    }

    pub fn read_from_json_value(content: serde_json::Value) -> Result<OtoroshiCtlConfig, String> {
        let yaml: serde_yaml::Value = serde_json::from_value::<serde_yaml::Value>(content).unwrap();
        serde_yaml::from_value::<OtoroshiCtlConfig>(yaml).map_err(|e| e.to_string())
    }

    pub async fn read_from_url(url: &String, _tls: bool) -> Result<OtoroshiCtlConfig, String> {
        match crate::utils::http::Http::get(url).await {
            Err(err) => {
                std::result::Result::Err(format!("{}", err))
            }, 
            Ok(content) => {
                match content.kind {
                    HttpContentKind::JSON => serde_json::from_slice::<OtoroshiCtlConfig>(&content.content).map_err(|e| e.to_string()),
                    HttpContentKind::YAML => serde_yaml::from_slice::<OtoroshiCtlConfig>(&content.content).map_err(|e| e.to_string()),
                }
            }
        }
    }

    pub fn to_connection_config(&self) -> crate::utils::otoroshi::OtoroshiConnectionConfig {

        let spec = self.clone();
        let current_context_name = spec.current_context;
        let current_context = spec.contexts.into_iter().find(|i| i.name == current_context_name).unwrap();
        let current_cluster = spec.clusters.into_iter().find(|i| i.name == current_context.cluster).unwrap();
        let current_user = spec.users.into_iter().find(|i| i.name == current_context.user).unwrap();
 
        crate::utils::otoroshi::OtoroshiConnectionConfig {
            host: format!("{}:{}", current_cluster.hostname, current_cluster.port).to_string(),
            hostname: current_cluster.hostname,
            port: current_cluster.port,
            ip_addresses: current_cluster.ip_addresses,
            cid: current_user.client_id,
            csec: current_user.client_secret,
            chealth: current_user.health_key,
            tls: current_cluster.tls,
            mtls: current_cluster.client_cert,
            routing_hostname: current_cluster.routing_hostname,
            routing_port: current_cluster.routing_port,
            routing_tls: current_cluster.routing_tls,
            routing_ip_addresses: current_cluster.routing_ip_addresses,
        }
    }

    pub fn get_context(&self, current_context_name: String) -> crate::utils::otoroshi::OtoroshiConnectionConfig {

        let spec = self.clone();
        let current_context = spec.contexts.into_iter().find(|i| i.name == current_context_name).unwrap();
        let current_cluster = spec.clusters.into_iter().find(|i| i.name == current_context.cluster).unwrap();
        let current_user = spec.users.into_iter().find(|i| i.name == current_context.user).unwrap();
 
        crate::utils::otoroshi::OtoroshiConnectionConfig {
            host: format!("{}:{}", current_cluster.hostname, current_cluster.port).to_string(),
            hostname: current_cluster.hostname,
            port: current_cluster.port,
            ip_addresses: current_cluster.ip_addresses,
            cid: current_user.client_id,
            csec: current_user.client_secret,
            chealth: current_user.health_key,
            tls: current_cluster.tls,
            mtls: current_cluster.client_cert,
            routing_hostname: current_cluster.routing_hostname,
            routing_port: current_cluster.routing_port,
            routing_tls: current_cluster.routing_tls,
            routing_ip_addresses: current_cluster.routing_ip_addresses,
        }
    }

    pub fn get_current_config_blocking(opts: CliOpts) -> OtoroshiCtlConfig {
        futures::executor::block_on(async {
            Self::get_current_config(opts.clone()).await
        })
    }

    pub async fn get_current_config(opts: CliOpts) -> OtoroshiCtlConfig {
        match std::env::var("OTOROSHICTL_USE_ENV_CONFIG") {
            Ok(v) if v == "true".to_string() => {
                let hostname = std::env::var("OTOROSHICTL_CLUSTER_HOSTNAME").unwrap();
                let port: u16 = std::env::var("OTOROSHICTL_CLUSTER_PORT").unwrap_or("443".to_string()).parse::<u16>().unwrap_or(443);
                let tls: bool = std::env::var("OTOROSHICTL_CLUSTER_TLS").unwrap_or("true".to_string()).parse::<bool>().unwrap_or(true);
                let ip_addresses: Option<Vec<String>> = std::env::var("OTOROSHICTL_CLUSTER_IP_ADDRESSES").ok().map(|addresses| addresses.split(",").map(|i| i.to_string()).collect());
                let routing_hostname: Option<String> = std::env::var("OTOROSHICTL_CLUSTER_ROUTING_HOSTNAME").ok();
                let routing_port: Option<u16> = std::env::var("OTOROSHICTL_CLUSTER_ROUTING_PORT").map(|i| i.parse::<u16>().unwrap_or(443)).ok();
                let routing_tls: Option<bool> = std::env::var("OTOROSHICTL_CLUSTER_ROUTING_TLS").map(|i| i.parse::<bool>().unwrap_or(true)).ok();
                let routing_ip_addresses: Option<Vec<String>> = std::env::var("OTOROSHICTL_CLUSTER_ROUTING_IP_ADDRESSES").ok().map(|addresses| addresses.split(",").map(|i| i.to_string()).collect());
                let cert = std::env::var("OTOROSHICTL_CLUSTER_CERT_LOCATION").ok();
                let key = std::env::var("OTOROSHICTL_CLUSTER_KEY_LOCATION").ok();
                let ca = std::env::var("OTOROSHICTL_CLUSTER_CA_LOCATION").ok();
                let certv = std::env::var("OTOROSHICTL_CLUSTER_CERT_VALUE").ok();
                let keyv = std::env::var("OTOROSHICTL_CLUSTER_KEY_VALUE").ok();
                let cav = std::env::var("OTOROSHICTL_CLUSTER_CA_VALUE").ok();
                let client_id = std::env::var("OTOROSHICTL_USER_CLIENT_ID").unwrap();
                let client_secret = std::env::var("OTOROSHICTL_USER_CLIENT_SECRET").unwrap();
                let health_key = std::env::var("OTOROSHICTL_USER_HEALTH_KEY").ok();
                let client_cert = if cert.is_some() || key.is_some() || ca.is_some() || certv.is_some() || keyv.is_some() || cav.is_some() {
                    Some(OtoroshiCtlConfigSpecClusterClientCert { 
                        cert_location: cert, 
                        cert_value: certv, 
                        key_location: key, 
                        key_value: keyv, 
                        ca_location: ca, 
                        ca_value: cav,
                    })
                } else {
                    None
                };

                let mut tmp = OtoroshiCtlConfig::empty();
                tmp.current_context = "env".to_string();
                tmp.users.push(OtoroshiCtlConfigSpecUser { name: "env".to_string(), client_id: client_id, client_secret: client_secret, health_key: health_key });
                
                tmp.clusters.push(OtoroshiCtlConfigSpecCluster { 
                    name: "env".to_string(), 
                    hostname, 
                    port, 
                    ip_addresses,
                    tls, 
                    client_cert, 
                    routing_hostname,
                    routing_port,
                    routing_tls,
                    routing_ip_addresses,
                });                    
                tmp.contexts.push(OtoroshiCtlConfigSpecContext { name: "env".to_string(), cluster: "env".to_string(), user: "env".to_string(), cloud_apim: false });
                tmp
            }, 
            _ => {
                match (opts.otoroshi_cluster_hostname, opts.otoroshi_cluster_port, opts.otoroshi_user_client_id, opts.otoroshi_user_client_secret, opts.otoroshi_user_health_key) {
                    (Some(hostname), Some(port), Some(client_id), Some(client_secret), health_key) => {
                        let mut tmp = OtoroshiCtlConfig::empty();
                        tmp.current_context = "tmp".to_string();
                        tmp.users.push(OtoroshiCtlConfigSpecUser { name: "tmp".to_string(), client_id: client_id, client_secret: client_secret, health_key: health_key });
                        match (opts.otoroshi_cluster_cert_location, opts.otoroshi_cluster_key_location, opts.otoroshi_cluster_ca_location) {
                            (Some(cert), Some(key), Some(ca)) => {
                                tmp.clusters.push(OtoroshiCtlConfigSpecCluster { 
                                    name: "tmp".to_string(), 
                                    hostname, 
                                    port, 
                                    tls: opts.otoroshi_cluster_tls, 
                                    ip_addresses: None,
                                    routing_hostname: opts.otoroshi_cluster_routing_hostname,
                                    routing_port: opts.otoroshi_cluster_routing_port,
                                    routing_tls: opts.otoroshi_cluster_routing_tls,
                                    routing_ip_addresses: None,
                                    client_cert: Some(OtoroshiCtlConfigSpecClusterClientCert { 
                                        cert_location: Some(cert), 
                                        cert_value: None, 
                                        key_location: Some(key), 
                                        key_value: None, 
                                        ca_location: Some(ca), 
                                        ca_value: None 
                                    }) 
                                });
                            },
                            _ => {
                                tmp.clusters.push(OtoroshiCtlConfigSpecCluster { 
                                    name: "tmp".to_string(), 
                                    hostname, 
                                    port, 
                                    tls: opts.otoroshi_cluster_tls, 
                                    ip_addresses: None,
                                    routing_hostname: opts.otoroshi_cluster_routing_hostname,
                                    routing_port: opts.otoroshi_cluster_routing_port,
                                    routing_tls: opts.otoroshi_cluster_routing_tls,
                                    routing_ip_addresses: None,
                                    client_cert: None 
                                });
                            }
                        };
                        tmp.contexts.push(OtoroshiCtlConfigSpecContext { name: "tmp".to_string(), cluster: "tmp".to_string(), user: "tmp".to_string(), cloud_apim: false });
                        tmp
                    },
                    _ => {
                        match opts.config_file {
                            Some(user_config) => {
                                match OtoroshiCtlConfig::read_from(&user_config).await {
                                    Err(err) => {
                                        cli_stderr_printline!("{}", err);
                                        std::process::exit(-1)
                                    },
                                    Ok(cfg) => cfg
                                }
                            },
                            _ => {
                                let cfg: OtoroshiCtlConfig = confy::load("io.otoroshi.otoroshictl", Some("config")).unwrap();
                                if cfg.current_context.is_empty() {
                                    // Self::write_current_config(Self::default_instance());
                                    // cfg = confy::load("io.otoroshi.otoroshictl", Some("config")).unwrap();
                                    cli_stderr_printline!("No config file found or found with no context, use 'otoroshictl config reset' to create one !");
                                    std::process::exit(-1)
                                }
                                cfg
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn write_current_config(cfg: OtoroshiCtlConfig) -> () {
        let _ = confy::store("io.otoroshi.otoroshictl", Some("config"), cfg).unwrap();
    }

    pub fn default_instance() -> OtoroshiCtlConfig {
        OtoroshiCtlConfig {
            api_version: "v1".to_string(),
            kind: "OtoroshiCtlConfig".to_string(),
            metadata: HashMap::new(),
            cloud_apim: None,
            users: vec![
                OtoroshiCtlConfigSpecUser {
                    name: "default".to_string(),
                    client_id: "admin-api-apikey-id".to_string(),
                    client_secret: "admin-api-apikey-secret".to_string(),
                    health_key: None,
                }
            ],
            contexts: vec![
                OtoroshiCtlConfigSpecContext {
                    name: "default".to_string(),
                    cluster: "default".to_string(),
                    user: "default".to_string(),
                    cloud_apim: false,
                }
            ],
            clusters: vec![
                OtoroshiCtlConfigSpecCluster {
                    name: "default".to_string(),
                    hostname: "otoroshi-api.oto.tools".to_string(),
                    port: 9999,
                    tls: false,
                    ip_addresses: None,
                    client_cert: None,
                    routing_hostname: None,
                    routing_port: None,
                    routing_tls: None,
                    routing_ip_addresses: None
                }
            ],
            current_context: "default".to_string()
        }
    }
}
