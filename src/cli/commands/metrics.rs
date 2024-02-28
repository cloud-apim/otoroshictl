use serde::{Serialize, Deserialize};

use crate::cli::cliopts::{CliOpts, Commands};
use crate::{cli_stderr_printline, cli_stdout_printline};
use crate::utils::otoroshi::Otoroshi;
use crate::utils::table::{TableResource, TableHelper};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OtoroshiMetrics {
   pub body: serde_json::Value
}

pub struct MetricsCommand {}

impl MetricsCommand {
    fn default_display(vec: Vec<TableResource>, columns_raw: Vec<String>) -> () {
        if columns_raw.is_empty() {
            TableHelper::display_table_of_resources_with_custom_columns(vec, vec![
                "name".to_string(),
                "type".to_string(),
                "value".to_string(),
                "count".to_string(),
                "max".to_string(),
                "mean".to_string(),
                "min".to_string(),
                "p50".to_string(),
                "p75".to_string(),
                "p95".to_string(),
                "p98".to_string(),
                "p99".to_string(),
                "p999".to_string(),
                "stddev".to_string(),
                "m15_rate".to_string(),
                "m1_rate".to_string(),
                "m5_rate".to_string(),
                "mean_rate".to_string(),
                "duration_units".to_string(),
                "rate_units".to_string(),
            ])
        } else {
            let columns: Vec<String> = columns_raw.into_iter().flat_map(|col| {
                let sub_cols: Vec<String> = col.split(",").map(|s| s.to_string()).collect();
                sub_cols
            }).collect();
            TableHelper::display_table_of_resources_with_custom_columns(vec, columns);
        }
    }

    pub async fn display(cli_opts: CliOpts, _command: &Commands, columns: Vec<String>, filter: Option<String>) -> () {
        match Otoroshi::get_metrics(cli_opts.clone()).await {
            None => {
                cli_stderr_printline!("error while fetching cluster metrics");
                std::process::exit(-1)
            },
            Some(metrics) => {
                let filters: Option<Vec<String>> = filter.map(|f| f.split(",").into_iter().map(|i| i.to_string()).collect());
                let vec: Vec<TableResource> = metrics.body.as_array().unwrap().into_iter().filter(|item| {
                    match &filters {
                        None => true,
                        Some(filters) => {
                            let name = item.as_object().unwrap().get("name").unwrap().as_str().unwrap();
                            filters.into_iter().find(|filter| name.starts_with(filter.as_str())).is_some()
                        }
                    }
                }).map(|item| {
                    TableResource {
                        raw: item.to_owned()
                    }
                }).collect();
                match cli_opts.ouput {
                    Some(str) => {
                        match str.as_str() {
                            "json" => cli_stdout_printline!("{}", serde_json::to_string(&metrics).unwrap()),
                            "json_pretty" => cli_stdout_printline!("{}", serde_json::to_string_pretty(&metrics).unwrap()),
                            "yaml" => cli_stdout_printline!("{}", serde_yaml::to_string(&metrics).unwrap()),
                            _ => Self::default_display(vec, columns),
                        }
                    },
                    _ => Self::default_display(vec, columns),
                };
            },
        };    
    }
}