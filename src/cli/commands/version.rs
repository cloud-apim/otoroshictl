use crate::cli::cliopts::{CliOpts, Commands};
use crate::utils::otoroshi::Otoroshi;
use crate::{cli_stderr_printline, cli_stdout_printline};

use cli_table::{Cell, Style, Table, print_stdout};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiVersion {
    #[serde(alias = "raw")]
    pub version: String,
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub build: Option<i32>,
    pub suffix: Option<String>,
    pub suffix_version: Option<i32>,
}

pub struct VersionCommand {}

impl VersionCommand {
    fn default_display(version: OtoroshiVersion) {
        let table = vec![vec![
            version.version.cell(),
            version.major.cell(),
            version.minor.cell(),
            version.patch.cell(),
            version
                .build
                .map(|v| v.to_string())
                .unwrap_or_default()
                .cell(),
            version
                .suffix
                .map(|v| v.to_string())
                .unwrap_or_default()
                .cell(),
            version
                .suffix_version
                .map(|v| v.to_string())
                .unwrap_or_default()
                .cell(),
        ]]
        .table()
        .title(vec![
            "version"
                .cell()
                .bold(true)
                .foreground_color(Some(cli_table::Color::Green)),
            "major".cell().bold(true),
            "minor".cell().bold(true),
            "patch".cell().bold(true),
            "build".cell().bold(true),
            "suffix".cell().bold(true),
            "suffix version".cell().bold(true),
        ]);
        let _ = print_stdout(table);
    }

    pub async fn display(cli_opts: CliOpts, _command: &Commands) {
        match Otoroshi::get_version(cli_opts.clone()).await {
            None => {
                cli_stderr_printline!("error while fetching version");
                std::process::exit(-1)
            }
            Some(version) => {
                match cli_opts.ouput {
                    Some(str) => match str.as_str() {
                        "json" => {
                            cli_stdout_printline!("{}", serde_json::to_string(&version).unwrap())
                        }
                        "json_pretty" => cli_stdout_printline!(
                            "{}",
                            serde_json::to_string_pretty(&version).unwrap()
                        ),
                        "yaml" => {
                            cli_stdout_printline!("{}", serde_yaml::to_string(&version).unwrap())
                        }
                        "raw" => cli_stdout_printline!("{}", version.version),
                        _ => Self::default_display(version),
                    },
                    _ => Self::default_display(version),
                };
            }
        };
    }
}
