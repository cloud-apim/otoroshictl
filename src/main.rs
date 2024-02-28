#![forbid(unsafe_code)]

#[macro_use]
extern crate log;
extern crate base64;

pub mod cli;
pub mod sidecar;
pub mod tunnels;

#[macro_use]
mod utils;

use std::fs;

use cli::cliopts::{SidecarSubCommand, CloudApimSubCommand};
use sidecar::config::OtoroshiSidecarConfig;

use crate::cli::cliopts::{CliOpts, Commands};
use crate::cli::commands;

#[tokio::main]
async fn main() {

    let cli_opts: CliOpts = CliOpts::build_from_command_line();

    let def_log = if cli_opts.verbose {
        "debug"
    } else if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(
        env_logger::Env::new()
        .filter_or("OTOROSHICTL", def_log)
        .write_style_or("OTOROSHICTL_LOG_STYLE", "always")
    )
    .format_timestamp(None)
    .format_module_path(false)
    .format_target(true)
    .init();

    match &cli_opts.command {

        Some(Commands::Resources { command }) => commands::resources::ResourcesCommand::display(cli_opts.clone(), command).await,
        Some(Commands::Config { command }) => cli::commands::config::ConfigCommand::handle_command(command, cli_opts.clone()).await,

        Some(command @ Commands::Version { }) => commands::version::VersionCommand::display(cli_opts.clone(), command).await,
        Some(command @ Commands::Infos { }) => commands::infos::InfosCommand::display(cli_opts.clone(), command).await,
        Some(command @ Commands::Entities { }) => commands::entities::EntitiesCommand::display(cli_opts.clone(), command).await,
        Some(command @ Commands::Health { }) => commands::health::HealthCommand::display(cli_opts.clone(), command).await,
        Some(command @ Commands::Metrics { columns, filters }) => commands::metrics::MetricsCommand::display(cli_opts.clone(), command, columns.to_vec(), filters.to_owned()).await,
        Some(Commands::RemoteTunnel { local_host, local_port, local_tls, expose, remote_domain, remote_subdomain, tunnel, tls }) => {
            tunnels::remote::RemoteTunnelCommand::start(cli_opts.clone(), tunnels::remote::RemoteTunnelCommandOpts {
                local_host: local_host.to_string(),
                local_port: local_port.to_owned(),
                local_tls: local_tls.to_owned(),
                expose: expose.to_owned(),
                remote_domain: remote_domain.clone(),
                remote_subdomain: remote_subdomain.clone(),
                tls: tls.to_owned(),
                tunnel: tunnel.to_string(),
            }).await
        },
        Some(Commands::Sidecar { command }) => {
            match command {
                SidecarSubCommand::Howto { } => {
                    crate::sidecar::sidecar::Sidecar::how_to();
                    std::process::exit(0)
                },
                SidecarSubCommand::GenerateConfig { file } => {
                    let path = file.clone().unwrap_or("./sidecar.yaml".to_string());
                    let config = OtoroshiSidecarConfig::default();
                    let _ = fs::write(path.clone(), serde_yaml::to_string(&config).unwrap()).unwrap();
                    cli_stdout_printline!("new sidecar config. file generated at {}", path.clone());
                    std::process::exit(0)
                },
                SidecarSubCommand::Install { file, dry_run, user } => {
                    match file {
                        Some(file) => {
                            match OtoroshiSidecarConfig::read_from(file).await {
                                Err(err) => {
                                    cli_stderr_printline!("{}", err);
                                    std::process::exit(-1)
                                },
                                Ok(sidecar_config) => {
                                    crate::sidecar::sidecar::Sidecar::install(sidecar_config, user, dry_run);
                                    std::process::exit(0)
                                }
                            }
                        },
                        None => {
                            cli_stderr_printline!("you have to provide a sidecar configuration file");
                            std::process::exit(-1)
                        }
                    }
                },
                SidecarSubCommand::Uninstall { dry_run } => {
                    crate::sidecar::sidecar::Sidecar::uninstall(dry_run);
                    std::process::exit(0)
                },
                SidecarSubCommand::Run { file } => {
                    match file {
                        Some(file) => {
                            match OtoroshiSidecarConfig::read_from(file).await {
                                Err(err) => {
                                    cli_stderr_printline!("{}", err);
                                    std::process::exit(-1)
                                },
                                Ok(sidecar_config) => {
                                    crate::sidecar::sidecar::Sidecar::start(cli_opts, sidecar_config, &None).await;
                                    std::process::exit(0)
                                }
                            }
                        },
                        None => {
                            cli_stderr_printline!("you have to provide a sidecar configuration file");
                            std::process::exit(-1)
                        }
                    }
                }
            }            
        },
        Some(Commands::TcpTunnel {}) => {
            // crate::tunnels::tcp::TcpTunnel::start().await;
            error!("not implemented yet !") // TODO: finish implementation !!!
        },
        Some(Commands::CloudApim { command }) => {
            match command {
                CloudApimSubCommand::Login {} => {
                    crate::commands::cloud_apim::CloudApimCommands::login(cli_opts).await;
                },
                CloudApimSubCommand::Logout {} => {
                    crate::commands::cloud_apim::CloudApimCommands::logout(cli_opts).await;
                },
                CloudApimSubCommand::List {} => {
                    crate::commands::cloud_apim::CloudApimCommands::display_deployments(cli_opts).await;
                },
                CloudApimSubCommand::Link { name } => {
                    crate::commands::cloud_apim::CloudApimCommands::link(cli_opts.clone(), name.to_string(), false).await;
                },
                CloudApimSubCommand::Use { name } => {
                    crate::commands::cloud_apim::CloudApimCommands::link(cli_opts.clone(), name.to_string(), true).await;
                },
                CloudApimSubCommand::Restart { name } => {
                    crate::commands::cloud_apim::CloudApimCommands::restart(cli_opts.clone(), name.to_string()).await;
                },
            }
        },
        None => {
        }
    }

}
