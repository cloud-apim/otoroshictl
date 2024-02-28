use serde::{Serialize, Deserialize};
use cli_table::{print_stdout, Cell, Style, Table};


use crate::cli::cliopts::{CliOpts, Commands};
use crate::{cli_stderr_printline, cli_stdout_printline};
use crate::utils::otoroshi::Otoroshi;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshExposedResourceVersion {
    pub name: String,
    pub served: bool,
    pub deprecated: bool,
    pub storage: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshExposedResource {
    pub kind: String,
    pub plural_name: String,
    pub singular_name: String,
    pub group: String,
    pub version: OtoroshExposedResourceVersion,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshExposedResources {
    pub version: String,
    pub resources: Vec<OtoroshExposedResource>
}

pub struct EntitiesCommand {}

impl EntitiesCommand {
    fn default_display(resources: OtoroshExposedResources) -> () {
        let table = resources.resources.into_iter().map(|item| {
            vec![  
                item.kind.cell(),
                item.singular_name.cell(),
                item.plural_name.cell(),
                item.group.cell(),
                item.version.name.cell(),
                item.version.served.cell(),
                item.version.deprecated.cell(),
                item.version.storage.cell(),
            ]
        })
        .table()
        .title(vec![
            "kind".cell().bold(true),
            "singular_name".cell().bold(true),
            "plural_name".cell().bold(true),
            "group".cell().bold(true),
            "version".cell().bold(true),
            "served".cell().bold(true),
            "deprecated".cell().bold(true),
            "storage".cell().bold(true),
        ]);
        let _ = print_stdout(table);
    }

    pub async fn display(cli_opts: CliOpts, _command: &Commands) -> () {
        match Otoroshi::get_exposed_resources(cli_opts.clone()).await {
            None => {
                cli_stderr_printline!("error while fetching exposed resources");
                std::process::exit(-1)
            },
            Some(resources) => {
                match cli_opts.ouput {
                    Some(str) => {
                        match str.as_str() {
                            "json" => cli_stdout_printline!("{}", serde_json::to_string(&resources).unwrap()),
                            "json_pretty" => cli_stdout_printline!("{}", serde_json::to_string_pretty(&resources).unwrap()),
                            "yaml" => cli_stdout_printline!("{}", serde_yaml::to_string(&resources).unwrap()),
                            _ => Self::default_display(resources),
                        }
                    },
                    _ => Self::default_display(resources),
                };
            },
        };    
    }
}