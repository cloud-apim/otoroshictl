use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::{utils::http::HttpContentKind, cli::cliopts::CliOpts};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOtoroshiSettings {
    pub api_context_name: Option<String>,
    pub location: Option<OtoroshiSidecarConfigSpecOtoroshiSettingsLocation>,
    pub routing_location: Option<OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation>,
    pub credentials: Option<OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials>,
    pub client_cert: Option<OtoroshiSidecarConfigSpecOtoroshiSettingsClientCert>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation {
    pub hostname: Option<String>,
    pub ip_addresses: Option<Vec<String>>,
    pub kubernetes: Option<OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation>,
    pub port: u16,
    pub tls: bool,
}

impl OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation {
    pub fn default() -> OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation {
        OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation {
            hostname: Some("otoroshi.oto.tools".to_string()),
            ip_addresses: None,
            kubernetes: None,
            port: 9999,
            tls: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation {
    pub service: String,
    pub namespace: String,
}

impl OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation {
    pub fn default() -> OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation {
        OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation {
            service: "otoroshi".to_string(),
            namespace: "otoroshi".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOtoroshiSettingsLocation {
    pub hostname: Option<String>,
    pub ip_addresses: Option<Vec<String>>,
    pub kubernetes: Option<OtoroshiSidecarConfigSpecOtoroshiSettingsLocationKubernetesLocation>,
    pub port: u16,
    pub tls: bool,
}

impl OtoroshiSidecarConfigSpecOtoroshiSettingsLocation {
    pub fn default() -> OtoroshiSidecarConfigSpecOtoroshiSettingsLocation {
        OtoroshiSidecarConfigSpecOtoroshiSettingsLocation {
            hostname: Some("otoroshi-api.oto.tools".to_string()),
            ip_addresses: None,
            kubernetes: None,
            port: 9999,
            tls: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials {
    pub client_id: String,
    pub client_secret: String,
}

impl OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials {
    pub fn default() -> OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials {
        OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials {
            client_id: "admin-api-apikey-id".to_string(),
            client_secret: "admin-api-apikey-secret".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOtoroshiSettingsClientCert {
    pub cert_location: Option<String>,
    pub cert_value: Option<String>,
    pub key_location: Option<String>,
    pub key_value: Option<String>,
    pub ca_location: Option<String>,
    pub ca_value: Option<String>,
}

impl OtoroshiSidecarConfigSpecOtoroshiSettingsClientCert {
    pub fn default() -> OtoroshiSidecarConfigSpecOtoroshiSettingsClientCert {
        OtoroshiSidecarConfigSpecOtoroshiSettingsClientCert {
            cert_location: None,
            cert_value: None,
            key_location: None,
            key_value: None,
            ca_location: None,
            ca_value: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecInboundSettingsTls {
    pub enabled: bool,
    pub cert_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecInboundSettingsMtls {
    pub enabled: bool,
    pub ca_cert_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOtoroshiProtocol {
    pub enabled: bool,
    pub version: Option<String>,
    pub route_id: Option<String>,
    pub secret_in: Option<String>,
    pub secret_out: Option<String>,
    pub algo_in: Option<String>,
    pub algo_out: Option<String>,
    pub header_in_name: Option<String>,
    pub header_out_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecInboundSettings {
    pub port: Option<u16>,
    pub target_port: Option<u16>,
    pub target_hostname: Option<String>,
    pub target_version: Option<String>,
    pub tls: Option<OtoroshiSidecarConfigSpecInboundSettingsTls>,
    pub mtls: Option<OtoroshiSidecarConfigSpecInboundSettingsMtls>,
    pub otoroshi_protocol: Option<OtoroshiSidecarConfigSpecOtoroshiProtocol>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOutboundsSettingsOutboundApikey {
    pub enabled: bool,
    pub apikey_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOutboundsSettingsOutboundMtls {
    pub enabled: bool,
    pub client_cert_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOutboundsSettingsOutbound {
    pub hostname: Option<String>,
    pub path: Option<String>,
    pub apikey: Option<OtoroshiSidecarConfigSpecOutboundsSettingsOutboundApikey>,
    pub mtls: Option<OtoroshiSidecarConfigSpecOutboundsSettingsOutboundMtls>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpecOutboundsSettings {
    pub port: Option<u16>,
    #[serde(flatten)]
    pub outbounds: HashMap<String, OtoroshiSidecarConfigSpecOutboundsSettingsOutbound>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfigSpec {
    pub kubernetes: bool,
    pub dns_integration: bool,
    pub dns_port: Option<u16>,
    pub dns_ns: Option<String>,
    pub dns_domain: Option<String>,
    pub dns_ttl: Option<u32>,
    pub otoroshi: Option<OtoroshiSidecarConfigSpecOtoroshiSettings>,
    pub inbound: OtoroshiSidecarConfigSpecInboundSettings,
    pub outbounds: OtoroshiSidecarConfigSpecOutboundsSettings,

}

impl OtoroshiSidecarConfigSpec {
    pub fn compute_otoroshi(&self, opts: CliOpts) -> OtoroshiSidecarConfigSpecOtoroshiSettings {
        match self.otoroshi.clone() {
            None => {
                // use the current context
                let config = crate::cli::config::OtoroshiCtlConfig::get_current_config_blocking(opts).to_connection_config();
                OtoroshiSidecarConfigSpecOtoroshiSettings {
                    api_context_name: None,
                    client_cert: config.mtls.map(|mtls| {
                        OtoroshiSidecarConfigSpecOtoroshiSettingsClientCert {
                            cert_location: mtls.cert_location,
                            cert_value: mtls.cert_value,
                            key_location: mtls.key_location,
                            key_value: mtls.key_value,
                            ca_location: mtls.ca_location,
                            ca_value: mtls.ca_value,
                        }
                    }),
                    credentials: Some(OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials { 
                        client_id: config.cid, 
                        client_secret: config.csec, 
                    }), 
                    location: Some(OtoroshiSidecarConfigSpecOtoroshiSettingsLocation { 
                        hostname: Some(config.hostname), 
                        ip_addresses: config.ip_addresses, 
                        kubernetes: None, 
                        port: config.port, 
                        tls: config.tls,
                    }),
                    routing_location: config.routing_hostname.clone().map(|_| {
                        OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation { 
                            hostname: config.routing_hostname.clone(), 
                            ip_addresses: config.routing_ip_addresses,
                            kubernetes: None, 
                            port: config.routing_port.unwrap(), 
                            tls: config.routing_tls.unwrap() 
                        }
                    }),
                }
            },
            Some(otoroshi) => {
                match otoroshi.api_context_name {
                    None => otoroshi.clone(),
                    Some(context) => {
                        let config = crate::cli::config::OtoroshiCtlConfig::get_current_config_blocking(opts).get_context(context);
                        OtoroshiSidecarConfigSpecOtoroshiSettings {
                            api_context_name: None,
                            client_cert: config.mtls.map(|mtls| {
                                OtoroshiSidecarConfigSpecOtoroshiSettingsClientCert {
                                    cert_location: mtls.cert_location,
                                    cert_value: mtls.cert_value,
                                    key_location: mtls.key_location,
                                    key_value: mtls.key_value,
                                    ca_location: mtls.ca_location,
                                    ca_value: mtls.ca_value,
                                }
                            }),
                            credentials: Some(OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials { 
                                client_id: config.cid, 
                                client_secret: config.csec, 
                            }), 
                            location: Some(OtoroshiSidecarConfigSpecOtoroshiSettingsLocation { 
                                hostname: Some(config.hostname), 
                                ip_addresses: None, 
                                kubernetes: None, 
                                port: config.port, 
                                tls: config.tls,
                            }),
                            routing_location: config.routing_hostname.clone().map(|_| {
                                OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation { 
                                    hostname: config.routing_hostname.clone(), 
                                    ip_addresses: None, 
                                    kubernetes: None, 
                                    port: config.routing_port.unwrap(), 
                                    tls: config.routing_tls.unwrap() 
                                }
                            }),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiSidecarConfig {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub metadata: HashMap<String, String>,
    pub spec: OtoroshiSidecarConfigSpec,
}

impl OtoroshiSidecarConfig {

    pub fn default() -> OtoroshiSidecarConfig {
        let mut metadata: HashMap<String, String> = HashMap::new();
        metadata.insert("name".to_string(), "default-sidecar".to_string());
        OtoroshiSidecarConfig {
            api_version: "proxy.otoroshi.io/v1".to_string(),
            kind: "Sidecar".to_string(),
            metadata: metadata,
            spec: OtoroshiSidecarConfigSpec {
                kubernetes: false,
                dns_integration: true,
                dns_domain: Some(".otoroshi.mesh".to_string()),
                dns_port: Some(2053),
                dns_ttl: Some(300),
                dns_ns: None,
                otoroshi: Some(OtoroshiSidecarConfigSpecOtoroshiSettings {
                    api_context_name: None,
                    location: Some(OtoroshiSidecarConfigSpecOtoroshiSettingsLocation {
                        hostname: Some("otoroshi-api.oto.tools".to_string()),
                        ip_addresses: None,
                        kubernetes: None,
                        port: 443,
                        tls: true,
                    }),
                    routing_location: Some(OtoroshiSidecarConfigSpecOtoroshiSettingsRoutingLocation {
                        hostname: Some("otoroshi-routing.oto.tools".to_string()),
                        ip_addresses: None,
                        kubernetes: None,
                        port: 443,
                        tls: true
                    }),
                    credentials: Some(OtoroshiSidecarConfigSpecOtoroshiSettingsCredentials::default()),
                    client_cert: None,
                }),
                inbound: OtoroshiSidecarConfigSpecInboundSettings {
                    port: Some(8443),
                    target_port: Some(14080),
                    target_hostname: None,
                    target_version: None,
                    tls: Some(OtoroshiSidecarConfigSpecInboundSettingsTls {
                        enabled: true,
                        cert_id: "a_cert_id".to_string(),
                    }),
                    mtls: Some(OtoroshiSidecarConfigSpecInboundSettingsMtls {
                        enabled: true,
                        ca_cert_id: "a_ca_cert_id".to_string(),
                    }),
                    otoroshi_protocol: Some(OtoroshiSidecarConfigSpecOtoroshiProtocol {
                        enabled: true,
                        version: Some("V2".to_string()),
                        route_id: None,
                        secret_in: Some("secret".to_string()),
                        secret_out: Some("secret".to_string()),
                        algo_in: Some("HS512".to_string()),
                        algo_out: Some("HS512".to_string()),
                        header_in_name: None,
                        header_out_name: None,
                    }), 
                },
                outbounds: OtoroshiSidecarConfigSpecOutboundsSettings {
                    port: Some(15000),
                    outbounds: {
                        let mut map: HashMap<String, OtoroshiSidecarConfigSpecOutboundsSettingsOutbound> = HashMap::new();
                        map.insert("a.oto.tools".to_string(), OtoroshiSidecarConfigSpecOutboundsSettingsOutbound{
                            hostname: Some("a.otoroshi.mesh".to_string()),
                            path: Some("/".to_string()),
                            apikey: Some(OtoroshiSidecarConfigSpecOutboundsSettingsOutboundApikey {
                                enabled: true,
                                apikey_id: "an_apikey_id".to_string(),
                            }),
                            mtls: Some(OtoroshiSidecarConfigSpecOutboundsSettingsOutboundMtls {
                                enabled: true,
                                client_cert_id: "a_cert_id".to_string(),
                            }),
                        });
                        map
                    }
                },
            }
        }
    }

    pub async fn read_from(path: &String) -> Result<OtoroshiSidecarConfig, String> {
        if path.starts_with("http://") {
            Self::read_from_url(path, false).await
        } else if path.starts_with("https://") {
            Self::read_from_url(path, false).await
        } else {
            Self::read_from_file(path)
        }
    }

    pub fn read_from_file(file: &String) -> Result<OtoroshiSidecarConfig, String> {
        if file.ends_with(".json") {
            serde_json::from_str::<OtoroshiSidecarConfig>(std::fs::read_to_string(file).unwrap().as_str()).map_err(|e| e.to_string())
        } else {
            serde_yaml::from_str::<OtoroshiSidecarConfig>(std::fs::read_to_string(file).unwrap().as_str()).map_err(|e| e.to_string())
        }
    }

    pub async fn read_from_url(url: &String, _tls: bool) -> Result<OtoroshiSidecarConfig, String> {
        match crate::utils::http::Http::get(url).await {
            Err(err) => {
                std::result::Result::Err(format!("{}", err))
            }, 
            Ok(content) => {
                match content.kind {
                    HttpContentKind::JSON => serde_json::from_slice::<OtoroshiSidecarConfig>(&content.content).map_err(|e| e.to_string()),
                    HttpContentKind::YAML => serde_yaml::from_slice::<OtoroshiSidecarConfig>(&content.content).map_err(|e| e.to_string()),
                }
            }
        }
    }
}

#[test]
fn test_sidecar_config_serde() {
    let res = serde_yaml::from_str::<OtoroshiSidecarConfig>(std::fs::read_to_string("./src/sidecar/config.yaml").unwrap().as_str()).unwrap();
    let _ = std::fs::write("./src/sidecar/config-out.yaml", serde_yaml::to_string(&res).unwrap()).unwrap();
    println!("res: {:?}", res)
}