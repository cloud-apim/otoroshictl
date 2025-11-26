use crate::cli::cliopts::CliOpts;
use crate::cli_stderr_printline;
use crate::cli_stdout_printline;
use crate::utils::otoroshi::Otoroshi;

pub struct ToolboxCommands {}

impl ToolboxCommands {
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
                                "json" => {
                                    cli_stdout_printline!(
                                        "{}",
                                        serde_json::to_string(&doc).unwrap()
                                    )
                                }
                                "json_pretty" => cli_stdout_printline!(
                                    "{}",
                                    serde_json::to_string_pretty(&doc).unwrap()
                                ),
                                "yaml" => {
                                    cli_stdout_printline!(
                                        "{}",
                                        serde_yaml::to_string(&doc).unwrap()
                                    )
                                }
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
