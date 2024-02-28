
use cli_table::{print_stdout, Cell, Style, Table, CellStruct, Color};

use std::vec::Vec;

pub struct TableResource {
    pub raw: serde_json::Value,
}

pub struct TableHelper {}

impl TableHelper {
    fn walk_through_object(path: Vec<String>, value: &serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
        if path.is_empty() {
            value.to_owned()
        } else {
            value.get(path.first().unwrap()).unwrap().as_object().unwrap().to_owned()
        }
    }

    fn column_as_cell(name: &String, value: &serde_json::Map<String, serde_json::Value>) -> CellStruct {
        match value.get(name) {
            Some(serde_json::Value::Array(arr)) => arr.len().cell().justify(cli_table::format::Justify::Center),
            Some(serde_json::Value::Object(obj)) => obj.len().cell().justify(cli_table::format::Justify::Center),
            Some(serde_json::Value::Bool(true)) => "yes".cell().justify(cli_table::format::Justify::Center),
            Some(serde_json::Value::Bool(false)) => "no".cell().justify(cli_table::format::Justify::Center),
            Some(serde_json::Value::Number(v)) => v.cell().justify(cli_table::format::Justify::Center),
            Some(serde_json::Value::String(str)) => str.cell(),
            _ => "".cell(),
        }
    }

    pub fn display_table_of_resources_with_custom_columns(vec: Vec<TableResource>, columns: Vec<String>) -> () {
        let table =  vec.into_iter().map(|item| {
            let value = item.raw.as_object().unwrap(); 
            let values: Vec<CellStruct> = columns.to_vec().into_iter().map(|name| {
                if name.contains(".") {
                    let path: Vec<String> = name.split(".").map(|i| i.to_string()).collect();
                    let last_name: String = path.last().unwrap().to_string();
                    Self::column_as_cell(&last_name, &Self::walk_through_object(path.get(0..path.len() - 1).unwrap().to_vec(), value))
                } else {
                    Self::column_as_cell(&name, value)
                }
            }).collect();
            values
        })
        .table()
        .title(
            columns.into_iter().map(|name| {
                if name.contains(".") {
                    name.split(".").last().unwrap_or(name.as_str()).cell().bold(true).foreground_color(Some(Color::White))
                } else {
                    name.cell().bold(true).background_color(Some(Color::Rgb(0, 0, 0))).foreground_color(Some(Color::White))
                }
            })
        );
        let _ = print_stdout(table);
    }

    pub fn display_table_of_resources_default(vec: Vec<TableResource>) -> () {
        Self::display_table_of_resources_with_custom_columns(vec, vec![
            "id".to_string(),
            "name".to_string(),
            "description".to_string(),
            "enabled".to_string(),
            "tags".to_string(),
            "metadata".to_string(),
        ])
    }
}