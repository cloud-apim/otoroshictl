use std::collections::HashMap;

use cli_table::{print_stdout, Cell, Style, Table};
use http::Method;
use http::Request;
use hyper::Client;
use hyper_rustls::HttpsConnector;
use serde::Deserialize;
use serde::Serialize;
use async_recursion::async_recursion;
use rand::{distributions::Alphanumeric, Rng}; 
use chrono::prelude::*;

use crate::cli::cliopts::CliOpts;
use crate::cli::config::OtoroshiCtlConfigCloudApim;
use crate::cli_stderr_printline;
use crate::cli_stdout_printline;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CloudApimDeployments {
    #[serde(flatten)]
    pub items: Vec<CloudApimDeployment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CloudApimDeployment {
    pub name: String,
    pub uid: String,
    pub region: String,
    pub version: String,
    pub kind: String,
    pub status: String,
    pub created_at: String,
    pub plan: String,
    pub link: String,
}

pub struct CloudApimResponse {
    pub status: u16,
    pub body_bytes: hyper::body::Bytes,
    pub headers: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CloudApimToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CloudApimLoginResponse {
    pub client_id: String,
    pub client_secret: String,
}

impl CloudApimToken {
    pub fn read_from_disk() -> Option<CloudApimToken> {
        confy::load("io.otoroshi.otoroshictl", Some("cloud_apim_token")).ok()
    }
    pub fn write_to_disk(cfg: CloudApimToken) {
        confy::store("io.otoroshi.otoroshictl", Some("cloud_apim_token"), cfg).unwrap();
    }
    pub fn delete_from_disk() {
        let path = confy::get_configuration_file_path("io.otoroshi.otoroshictl", Some("cloud_apim_token")).unwrap();
        let _ = std::fs::remove_file(path);
    }
}
pub struct CloudApimCommands {}

impl CloudApimCommands {

    async fn unauth_cloud_apim_call(method: hyper::Method, path: &str, accept: Option<String>, body: Option<hyper::Body>, content_type: Option<String>) -> CloudApimResponse {
        let scheme =  "https";
        let host = "cli.cloud-apim.com";
        let uri: String = format!("{}://{}{}", scheme, host, path);
        // debug!("calling {} {}", method, uri);
        let mut builder  = Request::builder()
                .method(method.clone())
                .uri(uri)
                .header("host", host)
                .header("accept", accept.clone().unwrap_or("application/json".to_string()));
        if body.is_some() && content_type.is_some() {
            // builder = builder.header("Content-Type", "application/json")
            builder = builder.header("Content-Type", content_type.clone().unwrap());
        }
        let req: Request<hyper::Body> = builder
            .body(body.unwrap_or(hyper::Body::empty()))
            .unwrap();
        let resp_result =  {
            let https = hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build();
            let client: Client<HttpsConnector<hyper::client::HttpConnector>> = Client::builder().build::<_, hyper::Body>(https);
            client.request(req).await
        };
        match resp_result {
            Err(err) => {
                cli_stderr_printline!("error while calling cloud-apim api: \n\n{}", err);
                std::process::exit(-1)
            },
            Ok(resp) => {
                let status = resp.status().as_u16();
                let mut headers = HashMap::new();
                for header in resp.headers().into_iter() {
                    headers.insert(header.0.as_str().to_string(), header.1.to_str().unwrap().to_string());
                }
                let body_bytes = hyper::body::to_bytes(resp).await.unwrap();
                // debug!("status: {}, body: {:?}", status, body_bytes);
                CloudApimResponse {
                    status,
                    headers,
                    body_bytes
                }
            }
        }
    }

    #[async_recursion]
    async fn cloud_apim_call(method: hyper::Method, path: &str, accept: Option<String>, body: Option<hyper::Body>, content_type: Option<String>, config: OtoroshiCtlConfigCloudApim, token: CloudApimToken) -> CloudApimResponse {
        let scheme =  "https";
        let host = "cli.cloud-apim.com";
        let uri: String = format!("{}://{}{}", scheme, host, path);
        // debug!("calling {} {}", method, uri);
        let mut builder  = Request::builder()
                .method(method.clone())
                .uri(uri)
                .header("host", host)
                .header("accept", accept.clone().unwrap_or("application/json".to_string()))
                .header("Authorization", format!("Bearer {}", token.access_token));
        if body.is_some() && content_type.is_some() {
            // builder = builder.header("Content-Type", "application/json")
            builder = builder.header("Content-Type", content_type.clone().unwrap());
        }
        let req: Request<hyper::Body> = builder
            .body(body.unwrap_or(hyper::Body::empty()))
            .unwrap();
        let resp_result =  {
            let https = hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build();
            let client: Client<HttpsConnector<hyper::client::HttpConnector>> = Client::builder().build::<_, hyper::Body>(https);
            client.request(req).await
        };
        match resp_result {
            Err(err) => {
                cli_stderr_printline!("error while calling cloud-apim api: \n\n{}", err);
                std::process::exit(-1)
            },
            Ok(resp) => {
                let status = resp.status().as_u16();
                if status == 401 {
                    let new_token = Self::get_token(config.clone()).await.unwrap();
                    CloudApimToken::write_to_disk(new_token.clone());
                    Self::cloud_apim_call(method.clone(), path, accept.clone(), None, content_type.clone(), config.clone(), new_token.clone()).await
                } else {
                    let mut headers = HashMap::new();
                    for header in resp.headers().into_iter() {
                        headers.insert(header.0.as_str().to_string(), header.1.to_str().unwrap().to_string());
                    }
                    let body_bytes = hyper::body::to_bytes(resp).await.unwrap();
                    // debug!("status: {}, body: {:?}", status, body_bytes);
                    CloudApimResponse {
                        status,
                        headers,
                        body_bytes
                    }
                }
            }
        }
    }

    async fn cloud_apim_call_with_token(method: hyper::Method, path: &str, accept: Option<String>, body: Option<hyper::Body>, content_type: Option<String>, config: OtoroshiCtlConfigCloudApim) -> CloudApimResponse {
        match CloudApimToken::read_from_disk() {
            Some(token) if !token.access_token.is_empty() => {
                Self::cloud_apim_call(method, path, accept, body, content_type, config.clone(), token.clone()).await
            },
            _ => {
                match Self::get_token(config.clone()).await {
                    None => CloudApimResponse {
                        status: 400,
                        body_bytes: hyper::body::Bytes::from("unable to get access_token"),
                        headers: HashMap::new(),
                    },
                    Some(token) => {
                        CloudApimToken::write_to_disk(token.clone());
                        Self::cloud_apim_call(method, path, accept, body, content_type, config.clone(), token.clone()).await
                    }
                }
            }
        }
    }


    async fn get_cloud_apim_resource(path: &str, accept: Option<String>, config: OtoroshiCtlConfigCloudApim) -> Option<hyper::body::Bytes> {
        let response = Self::cloud_apim_call_with_token(Method::GET, path, accept, None, Some("application/json".to_string()), config).await;
        if response.status == 200 || response.status == 201 {
            Some(response.body_bytes)
        } else {
            println!("status: {}, body: {:?}", response.status, response.body_bytes);
            None
        }
    }

    async fn restart_deployment(uid: String, config: OtoroshiCtlConfigCloudApim) -> Option<hyper::body::Bytes> {
        let response = Self::cloud_apim_call_with_token(Method::POST, format!("/api/deployments/{}/_restart", uid).as_str(), Some("application/json".to_string()), Some(hyper::Body::from("")), Some("application/json".to_string()), config).await;
        if response.status == 200 || response.status == 201 {
            Some(response.body_bytes)
        } else {
            println!("status: {}, body: {:?}", response.status, response.body_bytes);
            None
        }
    }

    async fn get_token(config: OtoroshiCtlConfigCloudApim) -> Option<CloudApimToken> {
        debug!("refreshing access token");
        let json = serde_json::json!({
            "grant_type": "client_credentials",
            "client_id": config.client_id,
            "client_secret": config.client_secret,
            "scope": [],
            "bearer_kind": "jwt"
        });
        let body = serde_json::to_string(&json).unwrap();
        let resp = Self::unauth_cloud_apim_call(Method::POST, "/oauth/token", Some("application/json".to_string()), Some(hyper::Body::from(body)), Some("application/json".to_string())).await;
        if resp.status == 200 {
            match serde_json::from_slice::<CloudApimToken>(&resp.body_bytes) {
                Err(e) => {
                    debug!("error while parsing token resp: {}", e);
                    None
                },
                Ok(mut token) => {
                    let dt = Local::now();
                    let naive_utc = dt.naive_utc();
                    let offset = *dt.offset();
                    let date_time = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
                    let formatted = format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"));
                    token.created_at = Some(formatted);
                    Some(token)
                }
            }
        } else {
            debug!("bad response status: {}", resp.status);
            None
        }
    }

    async fn get_deployements_list(config: OtoroshiCtlConfigCloudApim) -> Vec<CloudApimDeployment> {
        match Self::get_cloud_apim_resource("/api/deployments", Some("application/json".to_string()), config.clone()).await {
            None => Vec::new(),
            Some(body_bytes) => {
                match serde_json::from_slice::<Vec<CloudApimDeployment>>(&body_bytes) {
                    Ok(deployments) => deployments,
                    Err(e) => {
                        debug!("parse error: {}", e);
                        Vec::new()
                    },
                }
            }
        }
    }

    fn default_display(resources: Vec<CloudApimDeployment>) {
        let table = resources.into_iter().map(|item| {
            vec![ 
                //item.uid.cell(), 
                item.name.cell(),
                item.kind.cell(),
                item.version.cell(),
                item.status.cell(),
                item.region.cell(),
                item.plan.cell(),
                item.created_at.cell(),
            ]
        })
        .table()
        .title(vec![
            //"uid".cell().bold(true),
            "name".cell().bold(true),
            "kind".cell().bold(true),
            "version".cell().bold(true),
            "status".cell().bold(true),
            "region".cell().bold(true),
            "plan".cell().bold(true),
            "created_at".cell().bold(true),
        ]);
        let _ = print_stdout(table);
    }

    pub async fn display_deployments(cli_opts: CliOpts) {
        let config = crate::cli::config::OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.cloud_apim {
            None => {
                cli_stderr_printline!("no cloud-apim user found. try login first with 'otoroshictl cloud-apim login'");
                std::process::exit(0)
            },
            Some(config) => {
                let deployments = Self::get_deployements_list(config).await;
                match cli_opts.ouput {
                    Some(str) => {
                        match str.as_str() {
                            "json" => cli_stdout_printline!("{}", serde_json::to_string(&deployments).unwrap()),
                            "json_pretty" => cli_stdout_printline!("{}", serde_json::to_string_pretty(&deployments).unwrap()),
                            "yaml" => cli_stdout_printline!("{}", serde_yaml::to_string(&deployments).unwrap()),
                            _ => Self::default_display(deployments),
                        }
                    },
                    _ => Self::default_display(deployments),
                };
            }
        };
    }

    pub async fn logout(cli_opts: CliOpts) {
        let mut config = crate::cli::config::OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        config.cloud_apim = None;
        crate::cli::config::OtoroshiCtlConfig::write_current_config(config);
        CloudApimToken::delete_from_disk();
    }

    pub async fn login(cli_opts: CliOpts) {
        Self::logout(cli_opts.clone()).await;
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        let poll_path = format!("/api/cli_tokens?cli_token={}", token);
        let console_url = format!("https://console.cloud-apim.com/console/cli/login?cli_token={}", token);
        let mut count = 0;
        loop {
            if count == 1 {
                cli_stdout_printline!("Opening {} in a browser to login and link otoroshictl to your account", console_url);
                webbrowser::open(console_url.as_str()).unwrap();
            } 
            if count > 1 {
                cli_stdout_printline!("Retrying to link account ...")
            }
            if count >= 20 {
                cli_stderr_printline!("No account link after 20 retries. Please retry 'otoroshictl cloud-apim login' again !");
                std::process::exit(-1);
            }
            let mut config: crate::cli::config::OtoroshiCtlConfig = crate::cli::config::OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
            let res = Self::unauth_cloud_apim_call(Method::GET, poll_path.as_str(), Some("application/json".to_string()), None, None).await;
            if res.status == 200 {
                let response = serde_json::from_slice::<CloudApimLoginResponse>(&res.body_bytes).unwrap();
                config.cloud_apim = Some(OtoroshiCtlConfigCloudApim {
                    client_id: response.client_id,
                    client_secret: response.client_secret,
                });
                crate::cli::config::OtoroshiCtlConfig::write_current_config(config);
                break;
            } else {
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            count += 1;
        };
    }

    pub async fn link(cli_opts: CliOpts, name: String, change_current: bool) {
        let overwrite: bool = false;
        let config = crate::cli::config::OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.cloud_apim {
            None => {
                cli_stderr_printline!("no cloud-apim user found. try login first with 'otoroshictl cloud-apim login'");
                std::process::exit(0)
            },
            Some(config) => {
                let deployments = Self::get_deployements_list(config).await;
                match deployments.into_iter().find(|i| i.name == name) {
                    None => {
                        cli_stderr_printline!("no cloud-apim deployment found with name '{}'", name);
                        std::process::exit(0)
                    }, 
                    Some(deployment) => {
                        crate::commands::config::ConfigCommand::import_context(&Some(deployment.link), &overwrite, &change_current, &None, &false, cli_opts.clone()).await;
                    }
                }
            }
        };
    }

    pub async fn restart(cli_opts: CliOpts, name: String) {
        let config = crate::cli::config::OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.cloud_apim {
            None => {
                cli_stderr_printline!("no cloud-apim user found. try login first with 'otoroshictl cloud-apim login'");
                std::process::exit(0)
            },
            Some(config) => {
                let deployments = Self::get_deployements_list(config.clone()).await;
                match deployments.into_iter().find(|i| i.name == name) {
                    None => {
                        cli_stderr_printline!("no cloud-apim deployment found with name '{}'", name);
                        std::process::exit(0)
                    }, 
                    Some(deployment) => {
                        Self::restart_deployment(deployment.uid, config.clone()).await;
                        cli_stdout_printline!("your deployment '{}' is restarting ...", deployment.name);
                    }
                }
            }
        };
    }
}