use crate::cli::cliopts::CliOpts;
use crate::cli_stderr_printline;
use crate::cli_stdout_printline;
use crate::utils::interactive::{resolve_bool, resolve_param, resolve_password, resolve_port};
use crate::utils::otoroshi::Otoroshi;
use secrecy::ExposeSecret;

/// Mailer plugin ID constant - used for both verification and route creation
const MAILER_PLUGIN_ID: &str =
    "cp:otoroshi_plugins.com.cloud.apim.otoroshi.plugins.mailer.MailerEndpoint";

/// ApikeyCalls plugin ID constant
const APIKEY_PLUGIN_ID: &str = "cp:otoroshi.next.plugins.ApikeyCalls";

/// Build the mailer plugin configuration
fn build_mailer_plugin_config(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    smtps: bool,
    starttls: bool,
) -> serde_json::Value {
    serde_json::json!({
        "host": host,
        "port": port,
        "user": user,
        "password": password,
        "auth": true,
        "starttls_enabled": starttls,
        "smtps": smtps,
        "max_retries": 5
    })
}

/// Build the ApikeyCalls plugin configuration
fn build_apikey_plugin_config() -> serde_json::Value {
    serde_json::json!({
        "validate": true,
        "mandatory": true,
        "update_quotas": true,
        "wipe_backend_request": true
    })
}

/// Extract and update domain path in route frontend
fn update_route_domain_path(route: &mut serde_json::Value, path_suffix: &str) {
    let Some(frontend) = route.get_mut("frontend") else {
        return;
    };
    let Some(domains) = frontend.get_mut("domains") else {
        return;
    };
    let Some(domains_arr) = domains.as_array_mut() else {
        return;
    };
    let Some(first_domain) = domains_arr.first_mut() else {
        return;
    };
    let Some(domain_str) = first_domain.as_str() else {
        return;
    };
    let new_domain = format!("{}{}", domain_str.trim_end_matches('/'), path_suffix);
    *first_domain = serde_json::Value::String(new_domain);
}

/// Extract URL from route frontend domains
fn extract_route_url(route: &serde_json::Value) -> String {
    route
        .get("frontend")
        .and_then(|f| f.get("domains"))
        .and_then(|d| d.as_array())
        .and_then(|arr| arr.first())
        .and_then(|v| v.as_str())
        .map(|domain| {
            if domain.starts_with("http") {
                domain.to_string()
            } else {
                format!("https://{}", domain)
            }
        })
        .unwrap_or_else(|| "https://unknown/mailer".to_string())
}

/// Result structure for add_mailer
#[derive(serde::Serialize)]
struct MailerResult {
    route: RouteInfo,
    apikey: ApiKeyInfo,
    url: String,
}

#[derive(serde::Serialize)]
struct RouteInfo {
    id: String,
    name: String,
}

#[derive(serde::Serialize)]
struct ApiKeyInfo {
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "clientSecret")]
    client_secret: String,
    #[serde(rename = "bearerToken")]
    bearer_token: String,
}

pub struct ToolboxCommands {}

impl ToolboxCommands {
    /// Add a mailer endpoint with SMTP configuration and API key
    pub async fn add_mailer(
        cli_opts: CliOpts,
        host: Option<String>,
        port: Option<u16>,
        user: Option<String>,
        smtps: Option<bool>,
        starttls: Option<bool>,
    ) -> Result<(), String> {
        // 1. Resolve all parameters (now with Result handling)
        let smtp_host = resolve_param(host, "OTOROSHI_MAILER_HOST", "SMTP Host")?;
        let smtp_port = resolve_port(port, "OTOROSHI_MAILER_PORT", "SMTP Port", 465)?;
        let smtp_user = resolve_param(user, "OTOROSHI_MAILER_USER", "SMTP User")?;
        let smtp_password = resolve_password("OTOROSHI_MAILER_PASSWORD", "SMTP Password")?;
        let use_smtps = resolve_bool(smtps, "OTOROSHI_MAILER_SMTPS", "Use SMTPS?", true)?;
        let use_starttls =
            resolve_bool(starttls, "OTOROSHI_MAILER_STARTTLS", "Use STARTTLS?", true)?;

        // Validate inputs
        if smtp_host.is_empty() {
            return Err("error: SMTP host is required".to_string());
        }
        if smtp_user.is_empty() {
            return Err("error: SMTP user is required".to_string());
        }
        if smtp_password.expose_secret().is_empty() {
            return Err("error: SMTP password is required".to_string());
        }

        // Check plugin availability first (fail-fast)
        if !Otoroshi::is_plugin_available(MAILER_PLUGIN_ID, cli_opts.clone()).await {
            return Err(
                "error: MailerEndpoint plugin not found in Otoroshi.\n\
                 Please install the plugin from: https://github.com/cloud-apim/otoroshi-plugin-mailer"
                    .to_string(),
            );
        }

        // Show progress only in interactive mode (not JSON/YAML output)
        let is_structured_output = matches!(
            cli_opts.ouput.as_deref(),
            Some("json") | Some("json_pretty") | Some("yaml")
        );
        if !is_structured_output {
            cli_stdout_printline!("\nCreating mailer...");
        }

        // 2. Get route template
        let template = Otoroshi::get_route_template(cli_opts.clone())
            .await
            .ok_or_else(|| "error: failed to get route template from Otoroshi".to_string())?;

        // 3. Build the route with plugins
        let mut route = template;

        // Update frontend domain to add /mailer path (using refactored function)
        update_route_domain_path(&mut route, "/mailer");

        // Build plugins array using constants
        let mailer_plugin = serde_json::json!({
            "enabled": true,
            "debug": false,
            "plugin": MAILER_PLUGIN_ID,
            "include": [],
            "exclude": [],
            "config": build_mailer_plugin_config(
                &smtp_host,
                smtp_port,
                &smtp_user,
                smtp_password.expose_secret(),
                use_smtps,
                use_starttls
            ),
            "bound_listeners": [],
            "plugin_index": {}
        });

        let apikey_plugin = serde_json::json!({
            "enabled": true,
            "debug": false,
            "plugin": APIKEY_PLUGIN_ID,
            "include": [],
            "exclude": [],
            "config": build_apikey_plugin_config(),
            "bound_listeners": [],
            "plugin_index": {}
        });

        route["plugins"] = serde_json::json!([mailer_plugin, apikey_plugin]);

        // 4. Create the route
        let created_route = Otoroshi::create_route(route, cli_opts.clone())
            .await
            .ok_or_else(|| "error: failed to create route in Otoroshi".to_string())?;

        let route_id = created_route
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "error: route response missing 'id' field".to_string())?
            .to_string();

        // 5. Update route name
        let id_suffix = route_id.replace("route_", "");
        let mailer_name = format!("mailer_{}", id_suffix);
        let mut updated_route = created_route.clone();
        updated_route["name"] = serde_json::Value::String(mailer_name.clone());

        let final_route =
            match Otoroshi::update_route(&route_id, updated_route, cli_opts.clone()).await {
                Some(r) => r,
                None => {
                    cli_stderr_printline!("warning: failed to update route name, continuing...");
                    created_route
                }
            };

        // 6. Get apikey template and create apikey
        let apikey_template = Otoroshi::get_apikey_template(cli_opts.clone())
            .await
            .ok_or_else(|| "error: failed to get apikey template from Otoroshi".to_string())?;

        let mut apikey = apikey_template;
        apikey["clientName"] = serde_json::Value::String(mailer_name.clone());
        apikey["description"] =
            serde_json::Value::String(format!("API key for mailer route {}", route_id));

        let created_apikey = Otoroshi::create_apikey_for_route(&route_id, apikey, cli_opts.clone())
            .await
            .ok_or_else(|| "error: failed to create API key for route".to_string())?;

        let client_id = created_apikey
            .get("clientId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "error: apikey response missing 'clientId' field".to_string())?
            .to_string();

        let client_secret = created_apikey
            .get("clientSecret")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "error: apikey response missing 'clientSecret' field".to_string())?
            .to_string();

        // 7. Get bearer token
        let bearer_token = match Otoroshi::get_bearer_token(&client_id, cli_opts.clone()).await {
            Some(b) => b,
            None => {
                cli_stderr_printline!("warning: failed to get bearer token, using empty");
                String::new()
            }
        };

        // 8. Build URL (using refactored function)
        let url = extract_route_url(&final_route);

        // 9. Display result
        let result = MailerResult {
            route: RouteInfo {
                id: route_id,
                name: mailer_name,
            },
            apikey: ApiKeyInfo {
                client_id,
                client_secret,
                bearer_token,
            },
            url,
        };

        match cli_opts.ouput {
            Some(ref format) => match format.as_str() {
                "json" => {
                    cli_stdout_printline!(
                        "{}",
                        serde_json::to_string(&result).expect("Failed to serialize result")
                    );
                }
                "json_pretty" => {
                    cli_stdout_printline!(
                        "{}",
                        serde_json::to_string_pretty(&result).expect("Failed to serialize result")
                    );
                }
                "yaml" => {
                    cli_stdout_printline!(
                        "{}",
                        serde_yaml::to_string(&result).expect("Failed to serialize result")
                    );
                }
                _ => Self::display_mailer_result(&result, &smtp_user),
            },
            None => Self::display_mailer_result(&result, &smtp_user),
        }
        Ok(())
    }

    fn display_mailer_result(result: &MailerResult, smtp_user: &str) {
        cli_stdout_printline!("\nMailer created successfully!\n");
        cli_stdout_printline!("  Route ID: {}", result.route.id);
        cli_stdout_printline!("  Name: {}", result.route.name);
        cli_stdout_printline!("  URL: {}", result.url);
        cli_stdout_printline!("\nAPI Key (save these credentials, they won't be shown again):");
        cli_stdout_printline!("  Client ID: {}", result.apikey.client_id);
        cli_stdout_printline!("  Client Secret: {}", result.apikey.client_secret);
        cli_stdout_printline!("  Bearer Token: {}", result.apikey.bearer_token);
        cli_stdout_printline!("\nTest with:");
        cli_stdout_printline!("  curl -X POST '{}' \\", result.url);
        cli_stdout_printline!(
            "    -H 'Authorization: Bearer {}' \\",
            result.apikey.bearer_token
        );
        cli_stdout_printline!("    -H 'Content-Type: application/json' \\");
        cli_stdout_printline!("    -d '{{");
        cli_stdout_printline!("      \"subject\": \"Test Email\",");
        cli_stdout_printline!("      \"from\": \"{}\",", smtp_user);
        cli_stdout_printline!("      \"to\": [\"recipient@example.com\"],");
        cli_stdout_printline!("      \"text\": \"This is a test email\"");
        cli_stdout_printline!("    }}'");
        cli_stdout_printline!(
            "\nDocumentation: https://github.com/cloud-apim/otoroshi-plugin-mailer"
        );
    }

    pub async fn mtls(cli_opts: CliOpts, mode: Option<String>) {
        match mode {
            None => {
                let config = Otoroshi::get_global_config(cli_opts.clone()).await;
                match config {
                    None => {
                        cli_stderr_printline!("error while fetching global otoroshi config");
                        std::process::exit(-1)
                    }
                    Some(config) => {
                        let mode = config
                            .body
                            .get("tlsSettings")
                            .unwrap()
                            .get("clientAuth")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let doc = serde_json::json!({"mode": mode});
                        match cli_opts.ouput {
                            Some(str) => match str.as_str() {
                                "json" => cli_stdout_printline!(
                                    "{}",
                                    serde_json::to_string(&doc).unwrap()
                                ),
                                "json_pretty" => cli_stdout_printline!(
                                    "{}",
                                    serde_json::to_string_pretty(&doc).unwrap()
                                ),
                                "yaml" => cli_stdout_printline!(
                                    "{}",
                                    serde_yaml::to_string(&doc).unwrap()
                                ),
                                _ => cli_stdout_printline!("mTLS mode: {}", mode),
                            },
                            _ => cli_stdout_printline!("mTLS mode: {}", mode),
                        }
                    }
                }
            }
            Some(mode) => {
                let config = Otoroshi::get_global_config(cli_opts.clone()).await;
                match config {
                    None => {
                        cli_stderr_printline!("error while fetching global otoroshi config");
                        std::process::exit(-1)
                    }
                    Some(config) => {
                        let mut doc = config.body;
                        match mode.to_lowercase().as_str() {
                            "none" => {
                                doc["tlsSettings"]["clientAuth"] = "None".into();
                                let body_str = serde_json::to_string(&doc).unwrap();
                                Otoroshi::update_global_config(cli_opts.clone(), body_str).await;
                            }
                            "want" => {
                                doc["tlsSettings"]["clientAuth"] = "Want".into();
                                let body_str = serde_json::to_string(&doc).unwrap();
                                Otoroshi::update_global_config(cli_opts.clone(), body_str).await;
                            }
                            "need" => {
                                doc["tlsSettings"]["clientAuth"] = "Need".into();
                                let body_str = serde_json::to_string(&doc).unwrap();
                                Otoroshi::update_global_config(cli_opts.clone(), body_str).await;
                            }
                            other => {
                                cli_stderr_printline!("unknown mTLS mode: {}", other);
                                std::process::exit(-1)
                            }
                        }
                    }
                }
            }
        }
    }
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Tests for constants
    // -------------------------------------------------------------------------

    #[test]
    fn test_mailer_plugin_id_format() {
        assert!(MAILER_PLUGIN_ID.starts_with("cp:"));
        assert!(MAILER_PLUGIN_ID.contains("MailerEndpoint"));
    }

    #[test]
    fn test_apikey_plugin_id_format() {
        assert!(APIKEY_PLUGIN_ID.starts_with("cp:"));
        assert!(APIKEY_PLUGIN_ID.contains("ApikeyCalls"));
    }

    // -------------------------------------------------------------------------
    // Tests for build_mailer_plugin_config
    // -------------------------------------------------------------------------

    #[test]
    fn test_build_mailer_plugin_config_structure() {
        let config = build_mailer_plugin_config(
            "smtp.example.com",
            465,
            "user@example.com",
            "secret123",
            true,
            false,
        );

        assert_eq!(config["host"], "smtp.example.com");
        assert_eq!(config["port"], 465);
        assert_eq!(config["user"], "user@example.com");
        assert_eq!(config["password"], "secret123");
        assert_eq!(config["auth"], true);
        assert_eq!(config["smtps"], true);
        assert_eq!(config["starttls_enabled"], false);
        assert_eq!(config["max_retries"], 5);
    }

    #[test]
    fn test_build_mailer_plugin_config_with_starttls() {
        let config = build_mailer_plugin_config(
            "mail.test.org",
            587,
            "sender@test.org",
            "password",
            false,
            true,
        );

        assert_eq!(config["port"], 587);
        assert_eq!(config["smtps"], false);
        assert_eq!(config["starttls_enabled"], true);
    }

    // -------------------------------------------------------------------------
    // Tests for build_apikey_plugin_config
    // -------------------------------------------------------------------------

    #[test]
    fn test_build_apikey_plugin_config_structure() {
        let config = build_apikey_plugin_config();

        assert_eq!(config["validate"], true);
        assert_eq!(config["mandatory"], true);
        assert_eq!(config["update_quotas"], true);
        assert_eq!(config["wipe_backend_request"], true);
    }

    #[test]
    fn test_build_apikey_plugin_config_has_all_required_fields() {
        let config = build_apikey_plugin_config();

        // Verify all expected fields exist
        assert!(config.get("validate").is_some());
        assert!(config.get("mandatory").is_some());
        assert!(config.get("update_quotas").is_some());
        assert!(config.get("wipe_backend_request").is_some());
    }

    // -------------------------------------------------------------------------
    // Tests for update_route_domain_path
    // -------------------------------------------------------------------------

    #[test]
    fn test_update_route_domain_path_adds_suffix() {
        let mut route = serde_json::json!({
            "frontend": {
                "domains": ["example.com"]
            }
        });
        update_route_domain_path(&mut route, "/mailer");
        assert_eq!(route["frontend"]["domains"][0], "example.com/mailer");
    }

    #[test]
    fn test_update_route_domain_path_trims_trailing_slash() {
        let mut route = serde_json::json!({
            "frontend": {
                "domains": ["example.com/"]
            }
        });
        update_route_domain_path(&mut route, "/mailer");
        assert_eq!(route["frontend"]["domains"][0], "example.com/mailer");
    }

    #[test]
    fn test_update_route_domain_path_handles_missing_frontend() {
        let mut route = serde_json::json!({});
        update_route_domain_path(&mut route, "/mailer");
        // Should not panic, just return without modification
        assert!(route.get("frontend").is_none());
    }

    // -------------------------------------------------------------------------
    // Tests for extract_route_url
    // -------------------------------------------------------------------------

    #[test]
    fn test_extract_route_url_adds_https() {
        let route = serde_json::json!({
            "frontend": {
                "domains": ["example.com/mailer"]
            }
        });
        assert_eq!(extract_route_url(&route), "https://example.com/mailer");
    }

    #[test]
    fn test_extract_route_url_preserves_existing_scheme() {
        let route = serde_json::json!({
            "frontend": {
                "domains": ["http://example.com/mailer"]
            }
        });
        assert_eq!(extract_route_url(&route), "http://example.com/mailer");
    }

    #[test]
    fn test_extract_route_url_returns_default_on_missing() {
        let route = serde_json::json!({});
        assert_eq!(extract_route_url(&route), "https://unknown/mailer");
    }
}
