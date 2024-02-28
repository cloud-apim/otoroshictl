use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::cli::cliopts::ConfigSubCommand;
use crate::cli::config::OtoroshiCtlConfig;
use crate::cli::cliopts::CliOpts;
use crate::cli::config::OtoroshiCtlConfigSpecCluster;
use crate::cli::config::OtoroshiCtlConfigSpecUser;
use crate::cli::config::OtoroshiCtlConfigSpecContext;
use crate::cli_stderr_printline;
use crate::utils::table::TableResource;
use crate::utils::table::TableHelper;
use crate::cli_stdout_printline;
use json_value_merge::Merge;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct DisplayPath {
    path: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContextHolder {
    pub context: String
}

pub struct ConfigCommand {}

impl ConfigCommand {

    pub async fn handle_command(command: &ConfigSubCommand, cli_opts: CliOpts) -> () {
        match command {
            ConfigSubCommand::CurrentConfig {} => {
                Self::display_current_config(cli_opts).await
            },
            ConfigSubCommand::EditCurrentConfig {} => {
                Self::edit_current_config(cli_opts).await
            },
            ConfigSubCommand::CurrentLocation {} => {
                Self::display_current_config_location(cli_opts).await
            },
            ConfigSubCommand::CurrentContext {} => {
                Self::display_current_context(cli_opts).await
            },
            ConfigSubCommand::UseContext { name } => {
                Self::use_context(name, cli_opts.clone()).await;
            },
            ConfigSubCommand::Use { name } => {
                Self::use_context(name, cli_opts.clone()).await;
            },
            ConfigSubCommand::RenameContext { old_name, new_name } => {
                Self::rename_context(old_name, new_name, cli_opts.clone()).await;
            },
            ConfigSubCommand::ListClusters {} => {
                Self::list_clusters(cli_opts).await;
            },
            ConfigSubCommand::ListContexts {} => {
                Self::list_contexts(cli_opts).await;
            },
            ConfigSubCommand::ListUsers {} => {
                Self::list_users(cli_opts).await;
            },
            ConfigSubCommand::SetCluster { name, hostname, port, tls, routing_hostname, routing_port, routing_tls } => {
                Self::set_cluster(name, hostname, port, tls, routing_hostname, routing_port, routing_tls, cli_opts.clone()).await;
            },
            ConfigSubCommand::SetUser { name, client_id, client_secret, health_key } => {
                Self::set_user(name, client_id, client_secret, health_key, cli_opts.clone()).await;
            },
            ConfigSubCommand::SetContext { name, cluster, user } => {
                Self::set_context(name, cluster, user, cli_opts.clone()).await;
            },
            ConfigSubCommand::DeleteCluster { name } => {
                Self::delete_cluster(name, cli_opts.clone()).await;
            },
            ConfigSubCommand::DeleteUser { name } => {
                Self::delete_user(name, cli_opts.clone()).await;
            },
            ConfigSubCommand::DeleteContext { name } => {
                Self::delete_context(name, cli_opts.clone()).await;
            },
            ConfigSubCommand::Reset {} => {
                Self::nuke_config(cli_opts.clone()).await;
            },
            ConfigSubCommand::Import { file, overwrite, current, name } => {
                Self::import_context(file, overwrite, current, name, cli_opts.clone()).await;
            },
            ConfigSubCommand::List {} => {
                Self::display_context_list(cli_opts.clone()).await;
            }
            ConfigSubCommand::Delete { name } => {
                Self::delete_full_context(name, cli_opts.clone()).await;
            }
            ConfigSubCommand::Add { name, client_id, client_secret, health_key, hostname, port, tls , current, routing_hostname, routing_port, routing_tls } => {
                Self::set_cluster(name, hostname, port, tls, routing_hostname, routing_port, routing_tls, cli_opts.clone()).await;
                Self::set_user(name, client_id, client_secret, health_key, cli_opts.clone()).await;
                Self::set_context(name, name, name, cli_opts.clone()).await;
                if *current {
                    Self::use_context(name, cli_opts.clone()).await;
                }
            }
        }
    }

    fn get_config_file_path(cli_opts: CliOpts) -> String {
        match cli_opts.config_file {
            Some(file) => file,
            None => confy::get_configuration_file_path("io.otoroshi.otoroshictl", Some("config")).unwrap().to_string_lossy().to_string(),
        }
    }

    pub async fn import_context(path: &String, overwrite: &bool, change_current: &bool, name: &Option<String>, cli_opts: CliOpts) -> () {
        match crate::utils::file::FileHelper::get_content_string_result(&path).await {
            Err(err) => cli_stderr_printline!("{}", err),
            Ok(content) => {
                let imported_config = OtoroshiCtlConfig::read_from_string(&content).unwrap();
                match name {
                    Some(name) => {
                        let mut config = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
                        match imported_config.contexts.into_iter().find(|c| c.name == name.to_string()) {
                            Some(ctx) => {
                                let user = imported_config.users.into_iter().find(|u| u.name == ctx.user).unwrap();
                                let cluster = imported_config.clusters.into_iter().find(|u| u.name == ctx.cluster).unwrap();

                                config.users.push(user);
                                config.clusters.push(cluster);
                                config.contexts.push(ctx);
                                config.contexts.dedup_by(|a, b| a.name == b.name);
                                config.clusters.dedup_by(|a, b| a.name == b.name);
                                config.users.dedup_by(|a, b| a.name == b.name);
                                if change_current.to_owned() {
                                    config.current_context = name.to_string();
                                }
                                OtoroshiCtlConfig::write_current_config(config);
                            }, 
                            _ => {
                                cli_stdout_printline!("context name '{}' does not exists and cannot be imported", name);
                                std::process::exit(-1)
                            }
                        }
                    }, 
                    _ => {
                        if overwrite.to_owned() {
                            OtoroshiCtlConfig::write_current_config(imported_config);
                        } else {
                            let config = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
                            
                            let mut users: HashMap<String, OtoroshiCtlConfigSpecUser> = HashMap::new();
                            let mut contexts: HashMap<String, OtoroshiCtlConfigSpecContext> = HashMap::new();
                            let mut clusters: HashMap<String, OtoroshiCtlConfigSpecCluster> = HashMap::new();
        
                            for user in config.users.into_iter() {
                                users.insert(user.name.clone(), user);
                            }
                            for context in config.contexts.into_iter() {
                                contexts.insert(context.name.clone(), context);
                            }
                            for cluster in config.clusters.into_iter() {
                                clusters.insert(cluster.name.clone(), cluster);
                            }
                            for user in imported_config.users.into_iter() {
                                users.insert(user.name.clone(), user);
                            }
                            for context in imported_config.contexts.into_iter() {
                                contexts.insert(context.name.clone(), context);
                            }
                            for cluster in imported_config.clusters.into_iter() {
                                clusters.insert(cluster.name.clone(), cluster);
                            }
        
                            let mut new_config = OtoroshiCtlConfig::empty();
                            new_config.cloud_apim = config.cloud_apim;
                            new_config.users = users.into_iter().map(|i| i.1).collect();
                            new_config.clusters = clusters.into_iter().map(|i| i.1).collect();
                            new_config.contexts = contexts.into_iter().map(|i| i.1).collect();
                            if change_current.to_owned() {
                                new_config.current_context = imported_config.current_context;
                            } else {
                                new_config.current_context = config.current_context;
                            }
                            OtoroshiCtlConfig::write_current_config(new_config);
                        }
                    }
                }
            }
        }
    }

    async fn display_current_config(cli_opts: CliOpts) -> () {
        let path = Self::get_config_file_path(cli_opts.clone());
        match crate::utils::file::FileHelper::get_content_string_result(&path).await {
            Err(err) => cli_stderr_printline!("{}", err),
            Ok(content) =>  cli_stdout_printline!("{}", content),
        }
    }

    pub async fn delete_full_context(name: &Option<String>, cli_opts: CliOpts) -> () {
        let config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match name {
            None => {
                match config.contexts.into_iter().find(|c| c.name == config.current_context) {
                    None => {
                        cli_stderr_printline!("context named {} not found", config.current_context);
                    },
                    Some(context) => {
                        Self::delete_cluster(&context.cluster, cli_opts.clone()).await;
                        Self::delete_user(&context.user, cli_opts.clone()).await;
                        Self::delete_context(&context.name, cli_opts.clone()).await;
                        let mut new_config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
                        new_config.current_context = new_config.contexts.first().map(|i| i.name.to_string()).unwrap_or("none".to_string());
                        OtoroshiCtlConfig::write_current_config(new_config);
                    }
                }
            },
            Some(name) => {
                match config.contexts.into_iter().find(|c| c.name == name.to_string()) {
                    None => {
                        cli_stderr_printline!("context named {} not found", name);
                    },
                    Some(context) => {
                        Self::delete_cluster(&context.cluster, cli_opts.clone()).await;
                        Self::delete_user(&context.user, cli_opts.clone()).await;
                        Self::delete_context(&context.name, cli_opts.clone()).await;
                        let mut new_config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
                        new_config.current_context = new_config.contexts.first().map(|i| i.name.to_string()).unwrap_or("none".to_string());
                        OtoroshiCtlConfig::write_current_config(new_config);
                    }
                }
            }
        }
    }

    pub async fn display_context_list(cli_opts: CliOpts) -> () {
        let config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match cli_opts.ouput {
            Some(str) if str == "json".to_string() => cli_stdout_printline!("{}", serde_json::to_string(&config.contexts.into_iter().map(|c| c.name).collect::<Vec<_>>()).unwrap()),
            Some(str) if str == "json_pretty".to_string() => cli_stdout_printline!("{}", serde_json::to_string_pretty(&config.contexts.into_iter().map(|c| c.name).collect::<Vec<_>>()).unwrap()),
            Some(str) if str == "yaml".to_string() => cli_stdout_printline!("{}", serde_yaml::to_string(&config.contexts.into_iter().map(|c| c.name).collect::<Vec<_>>()).unwrap()),
            _ => {
                let columns = vec![
                    "name".to_string(),
                    "current".to_string(),
                    "cloud_apim".to_string(),
                ];
                let vec = config.contexts.into_iter().map(|u| {
                    if u.name == config.current_context {
                        let current = serde_json::json!({"current":"yes"});
                        let cloud_apim_value = if u.cloud_apim {
                            "yes"
                        } else {
                            ""
                        };
                        let cloud_apim = serde_json::json!({"cloud_apim":cloud_apim_value});
                        let mut res = serde_json::to_value(u).unwrap();
                        res.merge(&current);
                        res.merge(&cloud_apim);
                        TableResource {
                            raw: res,
                        }
                    } else {
                        let current = serde_json::json!({"current":""});
                        let cloud_apim_value = if u.cloud_apim {
                            "yes"
                        } else {
                            ""
                        };
                        let cloud_apim = serde_json::json!({"cloud_apim":cloud_apim_value});
                        let mut res = serde_json::to_value(u).unwrap();
                        res.merge(&current);
                        res.merge(&cloud_apim);
                        TableResource {
                            raw: res,
                        }
                    }
                }).collect();
                TableHelper::display_table_of_resources_with_custom_columns(vec, columns)
            }
        };
    }

    async fn edit_current_config(cli_opts: CliOpts) -> () {
        let path = Self::get_config_file_path(cli_opts.clone());
        match crate::utils::file::FileHelper::get_content_string_result(&path).await {
            Err(err) => cli_stderr_printline!("{}", err),
            Ok(content) => {
                let edited = edit::edit(content.clone()).unwrap_or(content.clone());
                std::fs::write(path, edited).unwrap();
            }
        }
    }

    async fn display_current_config_location(cli_opts: CliOpts) -> () {
        let path = Self::get_config_file_path(cli_opts.clone());
        let current = DisplayPath {
            path: path.clone(),
        };
        match cli_opts.ouput {
            Some(output) if output == "json".to_string() => cli_stdout_printline!("{}", serde_json::to_string(&current).unwrap()),
            Some(output) if output == "json_pretty".to_string() => cli_stdout_printline!("{}", serde_json::to_string_pretty(&current).unwrap()),
            Some(output) if output == "yaml".to_string() => cli_stdout_printline!("{}", serde_yaml::to_string(&current).unwrap()),
            _ => cli_stdout_printline!("{}", path.clone()),
        }
    }

    pub async fn display_current_context(cli_opts: CliOpts) -> () {
        let config = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        let current = ContextHolder { 
            context: config.current_context,
        };
        match cli_opts.ouput {
            Some(str) if str == "json".to_string() => cli_stdout_printline!("{}", serde_json::to_string(&current).unwrap()),
            Some(str) if str == "json_pretty".to_string() => cli_stdout_printline!("{}", serde_json::to_string_pretty(&current).unwrap()),
            Some(str) if str == "yaml".to_string() => cli_stdout_printline!("{}", serde_yaml::to_string(&current).unwrap()),
            _ => cli_stdout_printline!("{}", current.context),
        };
    }

    pub async fn use_context(name: &String, cli_opts: CliOpts) -> () {
        let mut config = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.contexts.clone().into_iter().find(|i| i.name == name.to_string()) {
            None => cli_stdout_printline!("context '{}' does not exists", name),
            Some(_) => {
                config.current_context = name.to_string();
                OtoroshiCtlConfig::write_current_config(config);
            }
        }
    }

    pub async fn rename_context(old_name: &String, new_name: &String, cli_opts: CliOpts) -> () {
        let mut config = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.contexts.clone().into_iter().find(|i| i.name == old_name.to_string()) {
            None => cli_stdout_printline!("context '{}' does not exists", old_name),
            Some(_) => {
                let new_contexts = config.contexts.clone().into_iter().map(|mut c| {
                    if c.name == old_name.to_string() {
                        c.name = new_name.to_string();
                        c
                    } else {
                        c
                    }
                }).collect();
                config.contexts = new_contexts;
                OtoroshiCtlConfig::write_current_config(config);
            }
        }
    }

    pub async fn list_clusters(cli_opts: CliOpts) -> () {
        let config = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match cli_opts.ouput {
            Some(str) if str == "json".to_string() => cli_stdout_printline!("{}", serde_json::to_string(&config.clusters).unwrap()),
            Some(str) if str == "json_pretty".to_string() => cli_stdout_printline!("{}", serde_json::to_string_pretty(&config.clusters).unwrap()),
            Some(str) if str == "yaml".to_string() => cli_stdout_printline!("{}", serde_yaml::to_string(&config.clusters).unwrap()),
            _ => {
                let columns = vec![
                    "name".to_string(),
                    "hostname".to_string(),
                    "port".to_string(),
                    "tls".to_string(),
                    "client_cert".to_string(),
                ];
                let vec = config.clusters.into_iter().map(|u| {
                    TableResource {
                        raw: serde_json::to_value(u).unwrap(),
                    }
                }).collect();
                TableHelper::display_table_of_resources_with_custom_columns(vec, columns)
            },
        };
    }

    pub async fn list_contexts(cli_opts: CliOpts) -> () {
        let config = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match cli_opts.ouput {
            Some(str) if str == "json".to_string() => cli_stdout_printline!("{}", serde_json::to_string(&config.contexts).unwrap()),
            Some(str) if str == "json_pretty".to_string() => cli_stdout_printline!("{}", serde_json::to_string_pretty(&config.contexts).unwrap()),
            Some(str) if str == "yaml".to_string() => cli_stdout_printline!("{}", serde_yaml::to_string(&config.contexts).unwrap()),
            _ => {
                let columns = vec![
                    "name".to_string(),
                    "cluster".to_string(),
                    "user".to_string(),
                    "cloud_apim".to_string(),
                ];
                let vec = config.contexts.into_iter().map(|u| {
                    TableResource {
                        raw: serde_json::to_value(u).unwrap(),
                    }
                }).collect();
                TableHelper::display_table_of_resources_with_custom_columns(vec, columns)
            },
        };
    }

    pub async fn list_users(cli_opts: CliOpts) -> () {
        let config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match cli_opts.ouput {
            Some(str) if str == "json".to_string() => cli_stdout_printline!("{}", serde_json::to_string(&config.users).unwrap()),
            Some(str) if str == "json_pretty".to_string() => cli_stdout_printline!("{}", serde_json::to_string_pretty(&config.users).unwrap()),
            Some(str) if str == "yaml".to_string() => cli_stdout_printline!("{}", serde_yaml::to_string(&config.users).unwrap()),
            _ => {
                let columns = vec![
                    "name".to_string(),
                    "client_id".to_string(),
                    "client_secret".to_string(),
                ];
                let vec = config.users.into_iter().map(|u| {
                    TableResource {
                        raw: serde_json::to_value(u).unwrap(),
                    }
                }).collect();
                TableHelper::display_table_of_resources_with_custom_columns(vec, columns)
            }
        };
    }

    pub async fn delete_cluster(name: &String, cli_opts: CliOpts) -> () {
        let mut config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.clusters.clone().into_iter().find(|i| i.name == name.to_string()) {
            None => cli_stdout_printline!("cluster '{}' does not exists", name),
            Some(_) => {
                let new_clusters = config.clusters.clone().into_iter().filter(|c| {
                    c.name != name.to_string()
                }).collect();
                config.clusters = new_clusters;
                OtoroshiCtlConfig::write_current_config(config);
            }
        }
    }

    pub async fn delete_user(name: &String, cli_opts: CliOpts) -> () {
        let mut config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.users.clone().into_iter().find(|i| i.name == name.to_string()) {
            None => cli_stdout_printline!("user '{}' does not exists", name),
            Some(_) => {
                let new_users = config.users.clone().into_iter().filter(|c| {
                    c.name != name.to_string()
                }).collect();
                config.users = new_users;
                OtoroshiCtlConfig::write_current_config(config);
            }
        }
    }

    pub async fn nuke_config(_cli_opts: CliOpts) -> () {
        OtoroshiCtlConfig::write_current_config(OtoroshiCtlConfig::default_instance());
    }

    pub async fn delete_context(name: &String, cli_opts: CliOpts) -> () {
        let mut config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.contexts.clone().into_iter().find(|i| i.name == name.to_string()) {
            None => cli_stdout_printline!("context '{}' does not exists", name),
            Some(_) => {
                let new_contexts = config.contexts.clone().into_iter().filter(|c| {
                    c.name != name.to_string()
                }).collect();
                config.contexts = new_contexts;
                OtoroshiCtlConfig::write_current_config(config);
            }
        }
    }

    pub async fn set_cluster(name: &String, hostname: &String, port: &u16, tls: &bool, routing_hostname: &Option<String>, routing_port: &Option<u16>, routing_tls: &Option<bool>, cli_opts: CliOpts) -> () {
        let mut config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.clusters.clone().into_iter().find(|i| i.name == name.to_string()) {
            None => {
                let mut new_clusters = config.clusters.clone();
                new_clusters.push(OtoroshiCtlConfigSpecCluster {
                    name: name.to_string(),
                    hostname: hostname.to_string(),
                    port: *port,
                    ip_addresses: None,
                    tls: *tls,
                    client_cert: None,
                    routing_hostname: routing_hostname.clone(), 
                    routing_port: *routing_port, 
                    routing_tls: *routing_tls,
                    routing_ip_addresses: None,
                });
                config.clusters = new_clusters;
                OtoroshiCtlConfig::write_current_config(config);
            },
            Some(_) => {
                let new_clusters = config.clusters.clone().into_iter().map(|mut c| {
                    if c.name == name.to_string() {
                        c.name = name.to_string();
                        c.hostname = hostname.to_string();
                        c.port = *port;
                        c.tls = *tls;
                        c.routing_hostname = routing_hostname.clone();
                        c.routing_port = *routing_port;
                        c.routing_tls = *routing_tls;
                        c
                    } else {
                        c
                    }
                }).collect();
                config.clusters = new_clusters;
                OtoroshiCtlConfig::write_current_config(config);
            }
        }
    }

    pub async fn set_user(name: &String, client_id: &String, client_secret: &String, health_key: &Option<String>, cli_opts: CliOpts) -> () {
        let mut config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.users.clone().into_iter().find(|i| i.name == name.to_string()) {
            None => {
                let mut new_users = config.users.clone();
                new_users.push(OtoroshiCtlConfigSpecUser {
                    name: name.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    health_key: health_key.to_owned(),
                });
                config.users = new_users;
                OtoroshiCtlConfig::write_current_config(config);
            },
            Some(_) => {
                let new_users = config.users.clone().into_iter().map(|mut c| {
                    if c.name == name.to_string() {
                        c.name = name.to_string();
                        c.client_id = client_id.to_string();
                        c.client_secret = client_secret.to_string();
                        c
                    } else {
                        c
                    }
                }).collect();
                config.users = new_users;
                OtoroshiCtlConfig::write_current_config(config);
            }
        }
    }

    pub async fn set_context(name: &String, cluster: &String, user: &String, cli_opts: CliOpts) -> () {
        let mut config: OtoroshiCtlConfig = OtoroshiCtlConfig::get_current_config(cli_opts.clone()).await;
        match config.clusters.clone().into_iter().find(|i| i.name == cluster.to_string()) {
            None => cli_stdout_printline!("cluster '{}' does not exists", name),
            Some(_) => {
                match config.users.clone().into_iter().find(|i| i.name == user.to_string()) {
                    None => cli_stdout_printline!("user '{}' does not exists", name),
                    Some(_) => {
                        match config.contexts.clone().into_iter().find(|i| i.name == name.to_string()) {
                            None => {
                                let mut new_contexts = config.contexts.clone();
                                new_contexts.push(OtoroshiCtlConfigSpecContext {
                                    name: name.to_string(),
                                    cluster: cluster.to_string(),
                                    user: user.to_string(),
                                    cloud_apim: false,
                                });
                                config.contexts = new_contexts;
                                OtoroshiCtlConfig::write_current_config(config);
                            },
                            Some(_) => {
                                let new_contexts = config.contexts.clone().into_iter().map(|mut c| {
                                    if c.name == name.to_string() {
                                        c.name = name.to_string();
                                        c.user = user.to_string();
                                        c.cluster = cluster.to_string();
                                        c
                                    } else {
                                        c
                                    }
                                }).collect();
                                config.contexts = new_contexts;
                                OtoroshiCtlConfig::write_current_config(config);
                            }
                        }
                    }
                }
            }
        }
    }

}