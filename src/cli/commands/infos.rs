use cli_table::{Cell, Style, Table, print_stdout};
use serde::{Deserialize, Serialize};

use crate::cli::cliopts::{CliOpts, Commands};
use crate::utils::otoroshi::Otoroshi;
use crate::{cli_stderr_printline, cli_stdout_printline};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JavaVersion {
    pub version: String,
    pub vendor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OsVersion {
    pub name: String,
    pub version: String,
    pub arch: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiInfos {
    pub otoroshi_cluster_id: String,
    pub otoroshi_version: String,
    pub java_version: JavaVersion,
    pub os: OsVersion,
    pub datastore: String,
    pub env: String,
    #[serde(default)]
    pub backoffice_url: Option<String>,
}

pub struct InfosCommand {}

impl InfosCommand {
    fn default_display(infos: OtoroshiInfos) {
        let table = vec![vec![
            infos.otoroshi_cluster_id.cell(),
            infos.otoroshi_version.cell(),
            infos.datastore.cell(),
            infos.env.cell(),
            (format!(
                "{} - {}",
                infos.java_version.version, infos.java_version.vendor
            ))
            .cell(),
            (format!(
                "{} - {} - {}",
                infos.os.name, infos.os.version, infos.os.arch
            ))
            .cell(),
        ]]
        .table()
        .title(vec![
            "cluster_id".cell().bold(true),
            "version".cell().bold(true),
            "datastore".cell().bold(true),
            "env".cell().bold(true),
            "java_version".cell().bold(true),
            "os".cell().bold(true),
        ]);
        let _ = print_stdout(table);
    }

    pub async fn display(cli_opts: CliOpts, _command: &Commands) {
        match Otoroshi::get_infos(cli_opts.clone()).await {
            None => {
                cli_stderr_printline!("error while fetching cluster infos");
                std::process::exit(-1)
            }
            Some(infos) => {
                match cli_opts.ouput {
                    Some(str) => match str.as_str() {
                        "json" => {
                            cli_stdout_printline!("{}", serde_json::to_string(&infos).unwrap())
                        }
                        "json_pretty" => cli_stdout_printline!(
                            "{}",
                            serde_json::to_string_pretty(&infos).unwrap()
                        ),
                        "yaml" => {
                            cli_stdout_printline!("{}", serde_yaml::to_string(&infos).unwrap())
                        }
                        _ => Self::default_display(infos),
                    },
                    _ => Self::default_display(infos),
                };
            }
        };
    }
}
