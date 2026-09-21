#![forbid(unsafe_code)]

#[macro_use]
extern crate log;
extern crate base64;

pub mod challenge;
pub mod cli;
pub mod http_utils;
pub mod otoroshi;
pub mod sidecar;
pub mod tunnels;

#[macro_use]
mod utils;

use std::fs;

use cli::cliopts::{
    ChallengeSubCommand, CloudApimSubCommand, SidecarSubCommand, ToolboxSubCommand,
};
use sidecar::config::OtoroshiSidecarConfig;

use crate::cli::cliopts::{CliOpts, Commands};
use crate::cli::commands;

#[tokio::main]
async fn main() {
    let cli_opts: CliOpts = CliOpts::build_from_command_line();

    let def_log = if cli_opts.verbose || cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(
        env_logger::Env::new()
            .filter_or("OTOROSHICTL", def_log)
            .write_style_or("OTOROSHICTL_LOG_STYLE", "always"),
    )
    .format_timestamp(None)
    .format_module_path(false)
    .format_target(true)
    .init();

    match &cli_opts.command {
        Some(Commands::Resources { command }) => {
            commands::resources::ResourcesCommand::display(cli_opts.clone(), command).await
        }
        Some(Commands::Config { command }) => {
            cli::commands::config::ConfigCommand::handle_command(command, cli_opts.clone()).await
        }

        Some(command @ Commands::Version {}) => {
            commands::version::VersionCommand::display(cli_opts.clone(), command).await
        }
        Some(command @ Commands::Infos {}) => {
            commands::infos::InfosCommand::display(cli_opts.clone(), command).await
        }
        Some(command @ Commands::Entities {}) => {
            commands::entities::EntitiesCommand::display(cli_opts.clone(), command).await
        }
        Some(command @ Commands::Health {}) => {
            commands::health::HealthCommand::display(cli_opts.clone(), command).await
        }
        Some(command @ Commands::Metrics { columns, filters }) => {
            commands::metrics::MetricsCommand::display(
                cli_opts.clone(),
                command,
                columns.to_vec(),
                filters.to_owned(),
            )
            .await
        }
        Some(Commands::RemoteTunnel {
            local_host,
            local_port,
            local_tls,
            expose,
            remote_domain,
            remote_subdomain,
            tunnel,
            tls,
        }) => {
            tunnels::remote::RemoteTunnelCommand::start(
                cli_opts.clone(),
                tunnels::remote::RemoteTunnelCommandOpts {
                    local_host: local_host.to_string(),
                    local_port: local_port.to_owned(),
                    local_tls: local_tls.to_owned(),
                    expose: expose.to_owned(),
                    remote_domain: remote_domain.clone(),
                    remote_subdomain: remote_subdomain.clone(),
                    tls: tls.to_owned(),
                    tunnel: tunnel.to_string(),
                },
            )
            .await
        }
        Some(Commands::Sidecar { command }) => match command {
            SidecarSubCommand::Howto {} => {
                crate::sidecar::sidecar::Sidecar::how_to();
                std::process::exit(0)
            }
            SidecarSubCommand::GenerateConfig { file } => {
                let path = file.clone().unwrap_or("./sidecar.yaml".to_string());
                let config = OtoroshiSidecarConfig::default();
                fs::write(path.clone(), serde_yaml::to_string(&config).unwrap()).unwrap();
                cli_stdout_printline!("new sidecar config. file generated at {}", path.clone());
                std::process::exit(0)
            }
            SidecarSubCommand::Install {
                file,
                dry_run,
                user,
            } => match file {
                Some(file) => match OtoroshiSidecarConfig::read_from(file).await {
                    Err(err) => {
                        cli_stderr_printline!("{}", err);
                        std::process::exit(-1)
                    }
                    Ok(sidecar_config) => {
                        crate::sidecar::sidecar::Sidecar::install(sidecar_config, user, dry_run);
                        std::process::exit(0)
                    }
                },
                None => {
                    cli_stderr_printline!("you have to provide a sidecar configuration file");
                    std::process::exit(-1)
                }
            },
            SidecarSubCommand::Uninstall { dry_run } => {
                crate::sidecar::sidecar::Sidecar::uninstall(dry_run);
                std::process::exit(0)
            }
            SidecarSubCommand::Run { file } => match file {
                Some(file) => match OtoroshiSidecarConfig::read_from(file).await {
                    Err(err) => {
                        cli_stderr_printline!("{}", err);
                        std::process::exit(-1)
                    }
                    Ok(sidecar_config) => {
                        crate::sidecar::sidecar::Sidecar::start(cli_opts, sidecar_config, &None)
                            .await;
                        std::process::exit(0)
                    }
                },
                None => {
                    cli_stderr_printline!("you have to provide a sidecar configuration file");
                    std::process::exit(-1)
                }
            },
        },
        Some(Commands::UdpTunnel {
            host,
            tls,
            local_host,
            local_port,
            remote_host,
            remote_port,
            access_type,
            apikey_client_id,
            apikey_client_secret,
            bearer_token,
            session_token,
        }) => {
            tunnels::udp::UdpTunnel::start(tunnels::udp::UdpTunnelOpts {
                host: host.to_string(),
                tls: *tls,
                local_host: local_host.to_string(),
                local_port: *local_port,
                remote_host: remote_host.clone(),
                remote_port: *remote_port,
                access_type: access_type.to_string(),
                apikey_client_id: apikey_client_id.clone(),
                apikey_client_secret: apikey_client_secret.clone(),
                bearer_token: bearer_token.clone(),
                session_token: session_token.clone(),
            })
            .await
        }
        Some(Commands::TcpTunnel {
            host,
            tls,
            local_host,
            local_port,
            remote_host,
            remote_port,
            access_type,
            apikey_client_id,
            apikey_client_secret,
            bearer_token,
            session_token,
        }) => {
            tunnels::tcp::TcpTunnel::start(tunnels::tcp::TcpTunnelOpts {
                host: host.to_string(),
                tls: *tls,
                local_host: local_host.to_string(),
                local_port: *local_port,
                remote_host: remote_host.clone(),
                remote_port: *remote_port,
                access_type: access_type.to_string(),
                apikey_client_id: apikey_client_id.clone(),
                apikey_client_secret: apikey_client_secret.clone(),
                bearer_token: bearer_token.clone(),
                session_token: session_token.clone(),
            })
            .await
        }
        Some(Commands::Toolbox { command }) => match command {
            ToolboxSubCommand::Mtls { mode } => {
                crate::commands::toolbox::ToolboxCommands::mtls(cli_opts.clone(), mode.clone())
                    .await;
            }
            ToolboxSubCommand::AddMailer {
                host,
                port,
                user,
                smtps,
                starttls,
            } => {
                if let Err(e) = crate::commands::toolbox::ToolboxCommands::add_mailer(
                    cli_opts.clone(),
                    host.clone(),
                    *port,
                    user.clone(),
                    *smtps,
                    *starttls,
                )
                .await
                {
                    cli_stderr_printline!("{}", e);
                    std::process::exit(-1);
                }
            }
            ToolboxSubCommand::Open => {
                if let Err(e) =
                    crate::commands::toolbox::ToolboxCommands::open(cli_opts.clone()).await
                {
                    cli_stderr_printline!("{}", e);
                    std::process::exit(-1);
                }
            }
        },
        Some(Commands::CloudApim { command }) => match command {
            CloudApimSubCommand::Login => {
                crate::commands::cloud_apim::CloudApimCommands::login(cli_opts).await;
            }
            CloudApimSubCommand::Logout => {
                crate::commands::cloud_apim::CloudApimCommands::logout(cli_opts).await;
            }
            CloudApimSubCommand::List => {
                crate::commands::cloud_apim::CloudApimCommands::display_deployments(cli_opts).await;
            }
            CloudApimSubCommand::Link { name } => {
                crate::commands::cloud_apim::CloudApimCommands::link(
                    cli_opts.clone(),
                    name.to_string(),
                    false,
                )
                .await;
            }
            CloudApimSubCommand::Use { name } => {
                crate::commands::cloud_apim::CloudApimCommands::link(
                    cli_opts.clone(),
                    name.to_string(),
                    true,
                )
                .await;
            }
            CloudApimSubCommand::Restart { name } => {
                crate::commands::cloud_apim::CloudApimCommands::restart(
                    cli_opts.clone(),
                    name.to_string(),
                )
                .await;
            }
        },
        Some(Commands::Challenge { command }) => match command {
            ChallengeSubCommand::Proxy {
                port,
                backend_port,
                backend_host,
                secret,
                secret_base64,
                state_header,
                state_resp_header,
                timeout,
                token_ttl,
                alg,
                public_key,
                response_secret,
                response_secret_base64,
                response_alg,
                v1,
                consumer_info,
                consumer_info_header,
                consumer_info_out_header,
                consumer_info_alg,
                consumer_info_secret,
                consumer_info_secret_base64,
                consumer_info_public_key,
                consumer_info_permissive,
                keep_otoroshi_headers,
                exclude_path,
            } => {
                crate::challenge::server::run(
                    *port,
                    backend_host.clone(),
                    *backend_port,
                    secret.clone(),
                    *secret_base64,
                    state_header.clone(),
                    state_resp_header.clone(),
                    *timeout,
                    *token_ttl,
                    alg.clone(),
                    public_key.clone(),
                    response_secret.clone(),
                    *response_secret_base64,
                    response_alg.clone(),
                    *v1,
                    *consumer_info,
                    consumer_info_header.clone(),
                    consumer_info_out_header.clone(),
                    consumer_info_alg.clone(),
                    consumer_info_secret.clone(),
                    *consumer_info_secret_base64,
                    consumer_info_public_key.clone(),
                    !consumer_info_permissive,
                    !keep_otoroshi_headers,
                    exclude_path.clone(),
                )
                .await;
            }
        },
        None => {}
    }
}
