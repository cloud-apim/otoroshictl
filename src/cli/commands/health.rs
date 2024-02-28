use serde::{Serialize, Deserialize};
use cli_table::{print_stdout, Cell, Style, Table};

use crate::cli::cliopts::{CliOpts, Commands};
use crate::{cli_stderr_printline, cli_stdout_printline};
use crate::utils::otoroshi::Otoroshi;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentHealth {
   pub initialized: Option<bool>,
   pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClusterHealth {
   pub health: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiHealth {
   pub otoroshi: String,
   pub datastore: String,
   pub proxy: ComponentHealth,
   pub storage: ComponentHealth,
   pub eventstore: ComponentHealth,
   pub certificates: ComponentHealth,
   pub scripts: ComponentHealth,
   pub cluster: Option<ClusterHealth>,
}

pub struct HealthCommand {}

impl HealthCommand {

    fn healthy_cell(value: String) -> cli_table::CellStruct {
        match value.as_str() {
            "healthy" => value.cell().foreground_color(Some(cli_table::Color::Green)),
            "unhealthy" => value.cell().foreground_color(Some(cli_table::Color::Magenta)),
            "down" => value.cell().foreground_color(Some(cli_table::Color::Red)),
            "unreachable" => value.cell().foreground_color(Some(cli_table::Color::Red)),
            _ => value.cell().foreground_color(Some(cli_table::Color::White)),
        }
    }

    fn loaded_cell(value: String) -> cli_table::CellStruct {
        match value.as_str() {
            "loaded" => value.cell().foreground_color(Some(cli_table::Color::Green)),
            _ => value.cell().foreground_color(Some(cli_table::Color::White)),
        }
    }

    fn default_display(health: OtoroshiHealth) -> () {
        let table = vec![
            vec![  
                Self::healthy_cell(health.otoroshi),
                Self::healthy_cell(health.datastore),
                Self::healthy_cell(health.storage.status),
                Self::healthy_cell(health.eventstore.status),
                Self::loaded_cell(health.certificates.status),
                Self::loaded_cell(health.scripts.status),
                match health.cluster {
                    None => "".cell(),
                    Some(cluster) => Self::healthy_cell(cluster.health),
                },
            ]
        ]
        .table()
        .title(vec![
            "otoroshi".cell().bold(true),
            "datastore".cell().bold(true),
            "storage".cell().bold(true),
            "eventstore".cell().bold(true),
            "certificates".cell().bold(true),
            "scripts".cell().bold(true),
            "cluster".cell().bold(true),
        ]);
        let _ = print_stdout(table);
    }

    pub async fn display(cli_opts: CliOpts, _command: &Commands) -> () {
        match Otoroshi::get_health(cli_opts.clone()).await {
            None => {
                cli_stderr_printline!("error while fetching health");
                std::process::exit(-1)
            },
            Some(health) => {
                match cli_opts.ouput {
                    Some(str) => {
                        match str.as_str() {
                            "json" => cli_stdout_printline!("{}", serde_json::to_string(&health).unwrap()),
                            "json_pretty" => cli_stdout_printline!("{}", serde_json::to_string_pretty(&health).unwrap()),
                            "yaml" => cli_stdout_printline!("{}", serde_yaml::to_string(&health).unwrap()),
                            _ => Self::default_display(health),
                        }
                    },
                    _ => Self::default_display(health),
                };
            },
        };    
    }
}