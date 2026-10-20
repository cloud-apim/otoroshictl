use cli_table::{Cell, Style, Table, print_stdout};
use http::Method;
use http::Request;
use serde::Deserialize;
use serde::Serialize;

use crate::cli::cliopts::CliOpts;
use crate::cli::config::OtoroshiCtlConfigCloudApim;
use crate::cli_stderr_printline;
use crate::cli_stdout_printline;
use crate::http_utils::client::{RequestBody, error_chain, native_roots_client, read_body};

/// The public API of Cloud APIM, served by its console, overridable with `CLOUD_APIM_API_URL` (a console in
/// development, for instance).
const DEFAULT_API_URL: &str = "https://console-next.cloud-apim.com";
const DEVICE_GRANT: &str = "urn:ietf:params:oauth:grant-type:device_code";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CloudApimNamed {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

/// A managed Otoroshi instance, as `GET /v1/managed` lists it.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CloudApimInstance {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub plan: Option<CloudApimNamed>,
    pub org: CloudApimNamed,
    #[serde(default, rename = "createdAt")]
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CloudApimInstances {
    instances: Vec<CloudApimInstance>,
}

#[derive(Deserialize, Clone, Debug)]
struct DeviceAuthorization {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: Option<String>,
    expires_in: u64,
    interval: u64,
}

#[derive(Deserialize, Clone, Debug)]
struct DeviceToken {
    access_token: String,
    email: Option<String>,
    org_id: Option<String>,
    scope: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct ApiError {
    error: String,
    error_description: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct OtoroshictlConfigResponse {
    config: String,
}

pub struct CloudApimResponse {
    pub status: u16,
    pub body_bytes: hyper::body::Bytes,
}

// a value of a form: what is not unreserved, percent-encoded
fn form_value(value: &str) -> String {
    value
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{:02X}", b),
        })
        .collect()
}

pub struct CloudApimCommands {}

impl CloudApimCommands {
    fn api_url(config: Option<&OtoroshiCtlConfigCloudApim>) -> String {
        std::env::var("CLOUD_APIM_API_URL")
            .ok()
            .or_else(|| config.and_then(|c| c.api_url.clone()))
            .unwrap_or(DEFAULT_API_URL.to_string())
            .trim_end_matches('/')
            .to_string()
    }

    /// A call of the public api: `body` is its content type and its content.
    async fn call(
        method: Method,
        url: String,
        token: Option<&str>,
        body: Option<(&str, String)>,
    ) -> CloudApimResponse {
        let mut builder = Request::builder()
            .method(method)
            .uri(url)
            .header("accept", "application/json");
        if let Some(token) = token {
            builder = builder.header("Authorization", format!("Bearer {}", token));
        }
        let req: Request<RequestBody> = match body {
            Some((content_type, content)) => builder
                .header("Content-Type", content_type)
                .body(RequestBody::from(content))
                .unwrap(),
            None => builder.body(RequestBody::default()).unwrap(),
        };
        match native_roots_client().request(req).await {
            Err(err) => {
                cli_stderr_printline!(
                    "error while calling the cloud-apim api: \n\n{}",
                    error_chain(&err)
                );
                std::process::exit(-1)
            }
            Ok(resp) => {
                let status = resp.status().as_u16();
                let body_bytes = read_body(resp).await.unwrap();
                CloudApimResponse { status, body_bytes }
            }
        }
    }

    /// The signed in session, or a message to sign in.
    async fn session(cli_opts: CliOpts) -> OtoroshiCtlConfigCloudApim {
        let config =
            crate::cli::config::OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.cloud_apim {
            Some(session) if session.access_token.is_some() => session,
            Some(_) => {
                cli_stderr_printline!(
                    "your cloud-apim session is from the previous console: sign in again with 'otoroshictl cloud-apim login'"
                );
                std::process::exit(-1)
            }
            None => {
                cli_stderr_printline!(
                    "no cloud-apim user found. try login first with 'otoroshictl cloud-apim login'"
                );
                std::process::exit(-1)
            }
        }
    }

    /// A call with the token of the session; an error of the api is printed, and ends the command.
    async fn authed(
        session: &OtoroshiCtlConfigCloudApim,
        method: Method,
        path: String,
        body: Option<(&str, String)>,
    ) -> hyper::body::Bytes {
        let url = format!("{}{}", Self::api_url(Some(session)), path);
        let resp = Self::call(method, url, session.access_token.as_deref(), body).await;
        match resp.status {
            200 | 201 => resp.body_bytes,
            401 => {
                cli_stderr_printline!(
                    "your cloud-apim session expired or was revoked: sign in again with 'otoroshictl cloud-apim login'"
                );
                std::process::exit(-1)
            }
            status => {
                let message = serde_json::from_slice::<ApiError>(&resp.body_bytes)
                    .map(|e| e.error_description.unwrap_or(e.error))
                    .unwrap_or_else(|_| String::from_utf8_lossy(&resp.body_bytes).to_string());
                cli_stderr_printline!("cloud-apim answered {}: {}", status, message);
                std::process::exit(-1)
            }
        }
    }

    async fn instances(session: &OtoroshiCtlConfigCloudApim) -> Vec<CloudApimInstance> {
        let body = Self::authed(session, Method::GET, "/v1/managed".to_string(), None).await;
        match serde_json::from_slice::<CloudApimInstances>(&body) {
            Ok(list) => list.instances,
            Err(e) => {
                debug!("parse error: {}", e);
                Vec::new()
            }
        }
    }

    /// An instance by its name (or its id): two of the same name in two organisations need the id.
    async fn instance(session: &OtoroshiCtlConfigCloudApim, name: &str) -> CloudApimInstance {
        let found: Vec<CloudApimInstance> = Self::instances(session)
            .await
            .into_iter()
            .filter(|i| i.name == name || i.id == name)
            .collect();
        match found.len() {
            0 => {
                cli_stderr_printline!("no cloud-apim managed instance found with name '{}'", name);
                std::process::exit(-1)
            }
            1 => found.into_iter().next().unwrap(),
            _ => {
                cli_stderr_printline!(
                    "several managed instances are named '{}' ({}): use the id of one instead",
                    name,
                    found
                        .iter()
                        .map(|i| format!("{} in {}", i.id, i.org.name))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                std::process::exit(-1)
            }
        }
    }

    fn default_display(resources: Vec<CloudApimInstance>) {
        let table = resources
            .into_iter()
            .map(|item| {
                vec![
                    item.name.cell(),
                    item.org.name.cell(),
                    item.version.unwrap_or("--".to_string()).cell(),
                    item.status.cell(),
                    item.region.unwrap_or("--".to_string()).cell(),
                    item.plan.map(|p| p.name).unwrap_or("--".to_string()).cell(),
                    item.created_at.unwrap_or("--".to_string()).cell(),
                    item.id.cell(),
                ]
            })
            .table()
            .title(vec![
                "name".cell().bold(true),
                "organisation".cell().bold(true),
                "version".cell().bold(true),
                "status".cell().bold(true),
                "region".cell().bold(true),
                "plan".cell().bold(true),
                "created_at".cell().bold(true),
                "id".cell().bold(true),
            ]);
        let _ = print_stdout(table);
    }

    pub async fn display_deployments(cli_opts: CliOpts) {
        let session = Self::session(cli_opts.clone()).await;
        let instances = Self::instances(&session).await;
        match cli_opts.ouput {
            Some(str) => match str.as_str() {
                "json" => cli_stdout_printline!("{}", serde_json::to_string(&instances).unwrap()),
                "json_pretty" => {
                    cli_stdout_printline!("{}", serde_json::to_string_pretty(&instances).unwrap())
                }
                "yaml" => cli_stdout_printline!("{}", serde_yaml::to_string(&instances).unwrap()),
                _ => Self::default_display(instances),
            },
            _ => Self::default_display(instances),
        };
    }

    /// Signs out: the token is revoked on the console, then forgotten here.
    pub async fn logout(cli_opts: CliOpts) {
        let mut config =
            crate::cli::config::OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        if let Some(session) = config.cloud_apim.clone() {
            if let Some(token) = session.access_token.as_deref() {
                let url = format!("{}/v1/me/token", Self::api_url(Some(&session)));
                let _ = Self::call(Method::DELETE, url, Some(token), None).await;
            }
        }
        config.cloud_apim = None;
        crate::cli::config::OtoroshiCtlConfig::write_current_config(config);
        // the token file of the previous console, if any
        if let Ok(path) =
            confy::get_configuration_file_path("io.otoroshi.otoroshictl", Some("cloud_apim_token"))
        {
            let _ = std::fs::remove_file(path);
        }
    }

    /// Signs in with a device authorization (RFC 8628): a code shown here, approved in the console.
    pub async fn login(cli_opts: CliOpts) {
        Self::logout(cli_opts.clone()).await;
        let api_url = Self::api_url(None);
        let start = Self::call(
            Method::POST,
            format!("{}/v1/cli/device", api_url),
            None,
            Some((
                "application/json",
                serde_json::json!({ "clientName": "otoroshictl" }).to_string(),
            )),
        )
        .await;
        if start.status != 200 {
            cli_stderr_printline!(
                "unable to start the login: cloud-apim answered {}",
                start.status
            );
            std::process::exit(-1)
        }
        let device: DeviceAuthorization = serde_json::from_slice(&start.body_bytes).unwrap();
        let page = device
            .verification_uri_complete
            .clone()
            .unwrap_or(device.verification_uri.clone());
        cli_stdout_printline!(
            "To sign in, approve the code {} in the console: {}",
            device.user_code,
            page
        );
        // (not on a machine without one: OTOROSHICTL_NO_BROWSER=1, the page is opened by hand)
        if std::env::var("OTOROSHICTL_NO_BROWSER").is_err() {
            if let Err(e) = crate::utils::browser::open_url(&page) {
                cli_stderr_printline!("Warning: {}", e);
            }
        }
        let mut interval = device.interval.max(1);
        let deadline =
            std::time::Instant::now() + std::time::Duration::from_secs(device.expires_in);
        loop {
            if std::time::Instant::now() > deadline {
                cli_stderr_printline!("the code expired: run 'otoroshictl cloud-apim login' again");
                std::process::exit(-1)
            }
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
            let form = format!(
                "grant_type={}&device_code={}",
                form_value(DEVICE_GRANT),
                form_value(&device.device_code)
            );
            let resp = Self::call(
                Method::POST,
                format!("{}/v1/cli/token", api_url),
                None,
                Some(("application/x-www-form-urlencoded", form)),
            )
            .await;
            if resp.status == 200 {
                let token: DeviceToken = serde_json::from_slice(&resp.body_bytes).unwrap();
                let mut config =
                    crate::cli::config::OtoroshiCtlConfig::get_current_config(cli_opts.clone())
                        .await;
                config.cloud_apim = Some(OtoroshiCtlConfigCloudApim {
                    access_token: Some(token.access_token),
                    api_url: std::env::var("CLOUD_APIM_API_URL").ok(),
                    email: token.email.clone(),
                    org_id: token.org_id.clone(),
                    ..Default::default()
                });
                crate::cli::config::OtoroshiCtlConfig::write_current_config(config);
                cli_stdout_printline!(
                    "Signed in as {} ({})",
                    token.email.unwrap_or("you".to_string()),
                    token.scope.unwrap_or_default()
                );
                break;
            }
            match serde_json::from_slice::<ApiError>(&resp.body_bytes) {
                Ok(e) if e.error == "authorization_pending" => {}
                Ok(e) if e.error == "slow_down" => interval += 5,
                Ok(e) if e.error == "access_denied" => {
                    cli_stderr_printline!("the login was refused in the console");
                    std::process::exit(-1)
                }
                Ok(e) => {
                    cli_stderr_printline!(
                        "unable to sign in: {}",
                        e.error_description.unwrap_or(e.error)
                    );
                    std::process::exit(-1)
                }
                Err(_) => {
                    cli_stderr_printline!("unable to sign in: cloud-apim answered {}", resp.status);
                    std::process::exit(-1)
                }
            }
        }
    }

    /// The configuration of otoroshictl for an instance, imported as a context (and made the current one for `use`).
    pub async fn link(cli_opts: CliOpts, name: String, change_current: bool) {
        let session = Self::session(cli_opts.clone()).await;
        let instance = Self::instance(&session, &name).await;
        let body = Self::authed(
            &session,
            Method::POST,
            format!(
                "/v1/orgs/{}/managed/{}/_otoroshictl",
                instance.org.id, instance.id
            ),
            Some(("application/json", "{}".to_string())),
        )
        .await;
        let response: OtoroshictlConfigResponse = serde_json::from_slice(&body).unwrap();
        crate::commands::config::ConfigCommand::import_context_from_string(
            response.config,
            &false,
            &change_current,
            &None,
            cli_opts.clone(),
        )
        .await;
    }

    pub async fn restart(cli_opts: CliOpts, name: String) {
        let session = Self::session(cli_opts.clone()).await;
        let instance = Self::instance(&session, &name).await;
        Self::authed(
            &session,
            Method::POST,
            format!(
                "/v1/orgs/{}/managed/{}/_restart",
                instance.org.id, instance.id
            ),
            Some(("application/json", "{}".to_string())),
        )
        .await;
        cli_stdout_printline!(
            "your managed instance '{}' is restarting ...",
            instance.name
        );
    }
}
