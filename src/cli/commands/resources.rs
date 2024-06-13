extern crate json_value_merge;

use crate::cli::cliopts::{CliOpts, ResourcesSubCommand};
use crate::{cli_stdout_printline, cli_stderr_printline};
use crate::utils::otoroshi::Otoroshi;
use crate::utils::entity::EntityHelper;
use crate::utils::table::{TableResource, TableHelper};

use std::collections::HashMap;
use std::fmt::format;
use std::{fs, path::PathBuf};

use std::vec::Vec;

use hyper::Method;
use json_value_merge::Merge;

use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use notify::{Watcher, RecursiveMode};

use super::entities::OtoroshExposedResources;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct KubeEntity {
    pub apiVersion: String,
    pub metadata: HashMap<String, String>,
    pub kind: String,
    pub spec: serde_json::Value,
}
impl KubeEntity {
    fn new(kind: String, name: String, spec: serde_json::Value) -> KubeEntity {
        let mut metadata = HashMap::new();
        metadata.insert("name".to_string(), name);
        KubeEntity {
            apiVersion: "proxy.otoroshi.io/v1".to_string(),
            kind: kind, 
            spec: spec,
            metadata: metadata,
        }
    }
}

pub struct ResourcesCommand {}

impl ResourcesCommand {

    async fn handle_json_entity(json: serde_json::Value, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let id = EntityHelper::extract_json_entity_id(&json).unwrap();
        let name = EntityHelper::extract_json_entity_name(&json).unwrap();
        let kind: String = json.get("kind").unwrap().as_str().unwrap().to_lowercase();
        let final_resource_name: String = exposed_resources.resources.clone().into_iter().find(|i| i.kind == kind || i.singular_name == kind).unwrap().plural_name;
        let content = serde_json::to_string(&json).unwrap();
        let config = Otoroshi::get_connection_config(cli_opts.clone()).await;
        let res = Otoroshi::otoroshi_call(hyper::Method::POST, format!("/apis/any/v1/{}/{}", final_resource_name, id).as_str(), None, Some(hyper::Body::from(content)), Some("application/json".to_string()), config).await;
        if res.status == 201 {
            cli_stdout_printline!("  - {}: created", name);
        } else if res.status == 200 {
            match res.headers.get(&"otoroshi-entity-updated".to_string()) {
                Some(v) if v.as_str() == "true" => cli_stdout_printline!("  - {}: updated", name),
                Some(v) if v.as_str() == "false" => cli_stdout_printline!("  - {}: unchanged", name),
                _ => cli_stdout_printline!("  - {}: updated", name),
            };
        } else {
            cli_stdout_printline!("  - {}: error", name);
        }
    }

    async fn delete_json_entity(json: serde_json::Value, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let id = EntityHelper::extract_json_entity_id(&json).unwrap();
        let name = EntityHelper::extract_json_entity_name(&json).unwrap();
        let kind: String = json.get("kind").unwrap().as_str().unwrap().to_lowercase();
        let final_resource_name: String = exposed_resources.resources.clone().into_iter().find(|i| i.kind == kind || i.singular_name == kind).unwrap().plural_name;
        let res = Otoroshi::delete_one_resource(final_resource_name, id, cli_opts.clone()).await;
        if res {
            cli_stdout_printline!("  - {}: deleted", name);
        } else {
            cli_stdout_printline!("  - {}: error", name);
        }
    }

    async fn handle_yaml_direct_entity(json: serde_json::Value, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let id = EntityHelper::extract_json_entity_id(&json).unwrap();
        let name = EntityHelper::extract_json_entity_name(&json).unwrap();
        let kind = json.get("kind").unwrap().as_str().unwrap().to_lowercase();
        let final_resource_name: String = exposed_resources.resources.clone().into_iter().find(|i| i.kind == kind || i.singular_name == kind).unwrap().plural_name;
        let content = serde_json::to_string(&json).unwrap();
        let config = Otoroshi::get_connection_config(cli_opts.clone()).await;
        let res = Otoroshi::otoroshi_call(hyper::Method::POST, format!("/apis/any/v1/{}/{}", final_resource_name, id).as_str(), None, Some(hyper::Body::from(content)), Some("application/yaml".to_string()), config).await;
        if res.status == 201 {
            cli_stdout_printline!("  - {}: created", name);
        } else if res.status == 200 {
            match res.headers.get(&"otoroshi-entity-updated".to_string()) {
                Some(v) if v.as_str() == "true" => cli_stdout_printline!("  - {}: updated", name),
                Some(v) if v.as_str() == "false" => cli_stdout_printline!("  - {}: unchanged", name),
                _ => cli_stdout_printline!("  - {}: updated", name),
            };
        } else {
            cli_stdout_printline!("  - {}: error", name);
        }
    }

    async fn delete_yaml_direct_entity(json: serde_json::Value, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let id = EntityHelper::extract_json_entity_id(&json).unwrap();
        let name = EntityHelper::extract_json_entity_name(&json).unwrap();
        let kind = json.get("kind").unwrap().as_str().unwrap().to_lowercase();
        let final_resource_name: String = exposed_resources.resources.clone().into_iter().find(|i| i.kind == kind || i.singular_name == kind).unwrap().plural_name;
        let res = Otoroshi::delete_one_resource(final_resource_name, id, cli_opts.clone()).await;
        if res {
            cli_stdout_printline!("  - {}: deleted", name);
        } else {
            cli_stdout_printline!("  - {}: error", name);
        }
    }

    async fn handle_yaml_kube_entity(json: serde_yaml::Value, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let spec = json.get("spec").unwrap();
        let id = EntityHelper::extract_yaml_entity_id(&spec).unwrap();
        let metadata = json.get("metadata").unwrap();
        let name = metadata.get("name").map(|i| i.as_str().unwrap().to_string()).or_else(|| EntityHelper::extract_yaml_entity_name(&spec)).unwrap();
        let kind = json.get("kind").unwrap().as_str().unwrap().to_lowercase();
        let final_resource_name: String = exposed_resources.resources.clone().into_iter().find(|i| i.kind == kind || i.singular_name == kind).unwrap().plural_name;
        let content = serde_yaml::to_string(spec).unwrap();
        let config = Otoroshi::get_connection_config(cli_opts.clone()).await;
        let res = Otoroshi::otoroshi_call(hyper::Method::POST, format!("/apis/any/v1/{}/{}", final_resource_name, id).as_str(), None, Some(hyper::Body::from(content)), Some("application/yaml".to_string()), config).await;
        if res.status == 201 {
            cli_stdout_printline!("  - {}: created", name);
        } else if res.status == 200 {
            cli_stdout_printline!("  - {}: updated", name);
        } else {
            cli_stdout_printline!("  - {}: error - {} - {:?}", name, res.status, res.body_bytes);
        }
    }

    async fn delete_yaml_kube_entity(json: serde_yaml::Value, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let spec = json.get("spec").unwrap();
        let id = EntityHelper::extract_yaml_entity_id(&spec).unwrap();
        let metadata = json.get("metadata").unwrap();
        let name = metadata.get("name").map(|i| i.as_str().unwrap().to_string()).or_else(|| EntityHelper::extract_yaml_entity_name(&spec)).unwrap();
        let kind = json.get("kind").unwrap().as_str().unwrap().to_lowercase();
        let final_resource_name: String = exposed_resources.resources.clone().into_iter().find(|i| i.kind == kind || i.singular_name == kind).unwrap().plural_name;
        let res = Otoroshi::delete_one_resource(final_resource_name, id, cli_opts.clone()).await;
        if res {
            cli_stdout_printline!("  - {}: deleted", name);
        } else {
            cli_stdout_printline!("  - {}: error", name);
        }
    }

    async fn handle_yaml_entity(yaml: serde_yaml::Value, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        if yaml.get("spec").is_some() && yaml.get("kind").is_some() {
            Self::handle_yaml_kube_entity(yaml, exposed_resources, cli_opts).await;
        } else {
            let json: serde_json::Value = serde_yaml::from_value::<serde_json::Value>(yaml).unwrap();
            Self::handle_yaml_direct_entity(json, exposed_resources, cli_opts).await;
        }
    }

    async fn delete_yaml_entity(yaml: serde_yaml::Value, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        if yaml.get("spec").is_some() && yaml.get("kind").is_some() {
            Self::delete_yaml_kube_entity(yaml, exposed_resources, cli_opts).await;
        } else {
            let json: serde_json::Value = serde_yaml::from_value::<serde_json::Value>(yaml).unwrap();
            Self::delete_yaml_direct_entity(json, exposed_resources, cli_opts).await;
        }
    }

    async fn handle_json_file(file: &PathBuf, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let content = fs::read_to_string(file).unwrap();
        let json = serde_json::from_str::<serde_json::Value>(&content).unwrap();
        if json.is_array() {
            for doc in json.as_array().unwrap() {
                Self::handle_json_entity(doc.to_owned(), exposed_resources, cli_opts.clone()).await;
            }
        } else {
            Self::handle_json_entity(json, exposed_resources, cli_opts.clone()).await;
        }
    }

    async fn delete_json_file(file: &PathBuf, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let content = fs::read_to_string(file).unwrap();
        let json = serde_json::from_str::<serde_json::Value>(&content).unwrap();
        if json.is_array() {
            for doc in json.as_array().unwrap() {
                Self::delete_json_entity(doc.to_owned(), exposed_resources, cli_opts.clone()).await;
            }
        } else {
            Self::delete_json_entity(json, exposed_resources, cli_opts.clone()).await;
        }
    }

    async fn handle_json_body(content: hyper::body::Bytes, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let json = serde_json::from_slice::<serde_json::Value>(&content).unwrap();
        if json.is_array() {
            for doc in json.as_array().unwrap() {
                Self::handle_json_entity(doc.to_owned(), exposed_resources, cli_opts.clone()).await;
            }
        } else {
            Self::handle_json_entity(json, exposed_resources, cli_opts.clone()).await;
        }
    }

    async fn delete_json_body(content: hyper::body::Bytes, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let json = serde_json::from_slice::<serde_json::Value>(&content).unwrap();
        if json.is_array() {
            for doc in json.as_array().unwrap() {
                Self::delete_json_entity(doc.to_owned(), exposed_resources, cli_opts.clone()).await;
            }
        } else {
            Self::delete_json_entity(json, exposed_resources, cli_opts.clone()).await;
        }
    }

    async fn handle_yaml_file(file: PathBuf, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let content = fs::read_to_string(file).unwrap();
        if content.contains("---\n") {
            for doc in content.split("---\n").filter(|s| !s.trim().is_empty()) {
                let yaml = serde_yaml::from_str::<serde_yaml::Value>(doc).unwrap();
                Self::handle_yaml_entity(yaml.to_owned(), exposed_resources, cli_opts.clone()).await;
            }
        } else {
            let yaml = serde_yaml::from_str::<serde_yaml::Value>(&content).unwrap();
            Self::handle_yaml_entity(yaml, exposed_resources, cli_opts.clone()).await;
        }
    }

    async fn delete_yaml_file(file: PathBuf, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let content = fs::read_to_string(file).unwrap();
        if content.contains("---\n") {
            for doc in content.split("---\n").filter(|s| !s.trim().is_empty()) {
                let yaml = serde_yaml::from_str::<serde_yaml::Value>(doc).unwrap();
                Self::delete_yaml_entity(yaml.to_owned(), exposed_resources, cli_opts.clone()).await;
            }
        } else {
            let yaml = serde_yaml::from_str::<serde_yaml::Value>(&content).unwrap();
            Self::delete_yaml_entity(yaml, exposed_resources, cli_opts.clone()).await;
        }
    }

    async fn handle_yaml_body(body: hyper::body::Bytes, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let content: String = String::from_utf8(body.to_vec()).unwrap();
        if content.contains("---\n") {
            for doc in content.split("---\n").filter(|s| !s.trim().is_empty()) {
                let yaml = serde_yaml::from_str::<serde_yaml::Value>(doc).unwrap();
                Self::handle_yaml_entity(yaml.to_owned(), exposed_resources, cli_opts.clone()).await;
            }
        } else {
            let yaml = serde_yaml::from_str::<serde_yaml::Value>(&content).unwrap();
            Self::handle_yaml_entity(yaml, exposed_resources, cli_opts.clone()).await;
        }
    }

    async fn delete_yaml_body(body: hyper::body::Bytes, exposed_resources: &OtoroshExposedResources, cli_opts: CliOpts) -> () {
        let content: String = String::from_utf8(body.to_vec()).unwrap();
        if content.contains("---\n") {
            for doc in content.split("---\n").filter(|s| !s.trim().is_empty()) {
                let yaml = serde_yaml::from_str::<serde_yaml::Value>(doc).unwrap();
                Self::delete_yaml_entity(yaml.to_owned(), exposed_resources, cli_opts.clone()).await;
            }
        } else {
            let yaml = serde_yaml::from_str::<serde_yaml::Value>(&content).unwrap();
            Self::delete_yaml_entity(yaml, exposed_resources, cli_opts.clone()).await;
        }
    }

    async fn sync_files(files: Vec<PathBuf>, cli_opts: CliOpts) -> () {
        cli_stdout_printline!("will try to sync {} files ...", files.len());
        let exposed_resources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
        for file in files.iter() {
            let fname = file.file_name().unwrap().to_str().unwrap();
            if fname.ends_with(".json") {
                Self::handle_json_file(file, &exposed_resources, cli_opts.clone()).await;
            } else if fname.ends_with(".yaml") || fname.ends_with(".yml") {
                Self::handle_yaml_file(file.to_owned(), &exposed_resources, cli_opts.clone()).await;
            }
        }
    }

    async fn delete_files(files: Vec<PathBuf>, cli_opts: CliOpts) -> () {
        cli_stdout_printline!("will try to delete {} files ...", files.len());
        let exposed_resources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
        for file in files.iter() {
            let fname = file.file_name().unwrap().to_str().unwrap();
            if fname.ends_with(".json") {
                Self::delete_json_file(file, &exposed_resources, cli_opts.clone()).await;
            } else if fname.ends_with(".yaml") || fname.ends_with(".yml") {
                Self::delete_yaml_file(file.to_owned(), &exposed_resources, cli_opts.clone()).await;
            }
        }
    }

    async fn fetch_url_http(url: String) -> (hyper::body::Bytes, String) {
        let req  = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri(url.clone())
            .header("accept", "application/json, application/yaml".to_string())
            .body(hyper::Body::empty())
            .unwrap();
        match hyper::Client::new().request(req).await {
            Err(err) => {
                cli_stderr_printline!("error while fetching https resource {}: \n\n{}", url, err);
                std::process::exit(-1)
            },
            Ok(resp) => {
                let status = resp.status().as_u16();
                let headers = resp.headers();
                let content_type = headers.get("content-type").map(|i| i.to_str().unwrap().to_string()).unwrap_or("application/json".to_string());
                if status == 200 {
                    let body_bytes: hyper::body::Bytes = hyper::body::to_bytes(resp).await.unwrap();
                    (body_bytes, content_type)
                } else {
                    cli_stderr_printline!("bad response status {} while fetching https resource {}", status, url);
                    std::process::exit(-1)
                }
            }
        }
    }

    async fn fetch_url_https(url: String) -> (hyper::body::Bytes, String) {
        let req  = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri(url.clone())
            .header("accept", "application/json, application/yaml".to_string())
            .body(hyper::Body::empty())
            .unwrap();
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        match client.request(req).await {
            Err(err) => {
                cli_stderr_printline!("error while fetching https resource {}: \n\n{}", url, err);
                std::process::exit(-1)
            },
            Ok(resp) => {
                let status = resp.status().as_u16();
                let headers = resp.headers();
                let content_type = headers.get("content-type").map(|i| i.to_str().unwrap().to_string()).unwrap_or("application/json".to_string());
                if status == 200 {
                    let body_bytes: hyper::body::Bytes = hyper::body::to_bytes(resp).await.unwrap();
                    (body_bytes, content_type)
                } else {
                    cli_stderr_printline!("bad response status {} while fetching https resource {}", status, url);
                    std::process::exit(-1)
                }
            }
        }
    }

    async fn sync_url_http(url: String, cli_opts: CliOpts) -> () {
        cli_stdout_printline!("will try to sync one url ...");
        let (body, content_type) = Self::fetch_url_http(url).await;
        let exposed_resources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
        if content_type.starts_with("application/json") {
            Self::handle_json_body(body, &exposed_resources, cli_opts.clone()).await;
        } else if content_type.starts_with("application/yaml") {
            Self::handle_yaml_body(body, &exposed_resources, cli_opts.clone()).await;
        } else {
            cli_stderr_printline!("bad response content-type {}", content_type);
            std::process::exit(-1)
        }
    }

    async fn sync_url_https(url: String, cli_opts: CliOpts) -> () {
        cli_stdout_printline!("will try to sync one url ...");
        let (body, content_type) = Self::fetch_url_https(url).await;
        let exposed_resources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
        if content_type.starts_with("application/json") {
            Self::handle_json_body(body, &exposed_resources, cli_opts.clone()).await;
        } else if content_type.starts_with("application/yaml") {
            Self::handle_yaml_body(body, &exposed_resources, cli_opts.clone()).await;
        } else {
            cli_stderr_printline!("bad response content-type {}", content_type);
            std::process::exit(-1)
        }
    }

    async fn delete_url_http(url: String, cli_opts: CliOpts) -> () {
        cli_stdout_printline!("will try to delete one url ...");
        let (body, content_type) = Self::fetch_url_http(url).await;
        let exposed_resources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
        if content_type.starts_with("application/json") {
            Self::delete_json_body(body, &exposed_resources, cli_opts.clone()).await;
        } else if content_type.starts_with("application/yaml") {
            Self::delete_yaml_body(body, &exposed_resources, cli_opts.clone()).await;
        } else {
            cli_stderr_printline!("bad response content-type {}", content_type);
            std::process::exit(-1)
        }
    }

    async fn delete_url_https(url: String, cli_opts: CliOpts) -> () {
        cli_stdout_printline!("will try to delete one url ...");
        let (body, content_type) = Self::fetch_url_https(url).await;
        let exposed_resources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
        if content_type.starts_with("application/json") {
            Self::delete_json_body(body, &exposed_resources, cli_opts.clone()).await;
        } else if content_type.starts_with("application/yaml") {
            Self::delete_yaml_body(body, &exposed_resources, cli_opts.clone()).await;
        } else {
            cli_stderr_printline!("bad response content-type {}", content_type);
            std::process::exit(-1)
        }
    }

    fn find_files(directory: &PathBuf, recursive: bool) -> Vec<PathBuf> {
        let mut res: Vec<PathBuf> = Vec::new();
        for entry in WalkDir::new(directory)
            .max_depth(if recursive { 99999 } else { 0 })
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
                let f_name = entry.file_name().to_string_lossy();
                if f_name.ends_with(".json") || f_name.ends_with(".yaml") || f_name.ends_with(".yml") {
                    res.push(entry.into_path());
                }
            }
        res.to_vec()
    }

    fn display_table_of_routes(vec: Vec<TableResource>) -> () {
        TableHelper::display_table_of_resources_with_custom_columns(vec, vec![
            "id".to_string(),
            "name".to_string(),
            "enabled".to_string(),
            "debug_flow".to_string(),
            "capture".to_string(),
            "frontend.domains".to_string(),
            "backend.targets".to_string(),
            "plugins".to_string(),
        ])
    }

    fn display_table_of_resources(kind: String, vec: Vec<TableResource>, columns_raw: Vec<String>) -> () {
        if columns_raw.is_empty() {
            match kind.as_str() {
                "route" => Self::display_table_of_routes(vec),
                _       => TableHelper::display_table_of_resources_default(vec),
            }
        } else {
            let columns: Vec<String> = columns_raw.into_iter().flat_map(|col| {
                let sub_cols: Vec<String> = col.split(",").map(|s| s.to_string()).collect();
                sub_cols
            }).collect();
            TableHelper::display_table_of_resources_with_custom_columns(vec, columns);
        }
    }

    pub fn with_kind(value: &serde_json::Value, kd: String) -> serde_json::Value {
        let mut kind: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        kind.insert("kind".to_string(), serde_json::Value::String(kd));
        let mut entity_with_kind = value.clone();
        entity_with_kind.merge(&serde_json::Value::Object(kind));
        entity_with_kind
    }

    fn run_watch(path: String, dir: bool, recursive: bool, cli_opts: CliOpts) -> () {
        let path_err = path.clone();
        let mut watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(_) => {
                    futures::executor::block_on(async {
                        if dir {
                            let files = Self::find_files(&PathBuf::from(path_err.to_owned()), recursive);
                            Self::sync_files(files, cli_opts.clone()).await;
                        } else {
                            Self::sync_files(vec![PathBuf::from(path_err.to_owned())], cli_opts.clone()).await;
                        }
                    });
                },
                Err(e) => {
                    cli_stderr_printline!("error while watching '{}': {}", path_err, e);
                },
            }
        }).unwrap();
        watcher.watch(std::path::Path::new(&path.clone()), if recursive { RecursiveMode::Recursive }  else { RecursiveMode::NonRecursive }).unwrap();
    }

    pub async fn display(cli_opts: CliOpts, command: &ResourcesSubCommand) -> () {
        match command {
            ResourcesSubCommand::Rbac { file, namespace } => {
                let exposed_resources: OtoroshExposedResources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
                let resources = exposed_resources.resources.into_iter().map(|resource| {
                    format!("      - {}", resource.plural_name)
                })
                .collect::<Vec<String>>()
                .join("\n");
                let final_namespace: String = namespace.to_owned().unwrap_or("default".to_string());
                let output: String = format!("---
kind: ServiceAccount
apiVersion: v1
metadata:
  name: otoroshi-admin-user
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: otoroshi-admin-user
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: otoroshi-admin-user
subjects:
- kind: ServiceAccount
  name: otoroshi-admin-user
  namespace: {}
---
kind: ClusterRole
apiVersion: rbac.authorization.k8s.io/v1
metadata:
  name: otoroshi-admin-user
rules:
  - apiGroups:
      - \"\"
    resources:
      - services
      - endpoints
      - secrets
      - configmaps
      - deployments
      - pods
      - namespaces
    verbs:
      - get
      - list
      - watch
  - apiGroups:
      - \"apps\"
    resources:
      - deployments
    verbs:
      - get
      - list
      - watch
  - apiGroups:
      - \"\"
    resources:
      - secrets
      - configmaps
    verbs:
      - update
      - update
      - create
      - delete
  - apiGroups:
      - extensions
    resources:
      - ingresses
      - ingressclasses
    verbs:
      - get
      - list
      - watch
  - apiGroups:
      - extensions
    resources:
      - ingresses/status
    verbs:
      - update
  - apiGroups:
      - admissionregistration.k8s.io
    resources:
      - validatingwebhookconfigurations
      - mutatingwebhookconfigurations
    verbs:
      - get
      - update
      - patch
  - apiGroups:
      - proxy.otoroshi.io
    resources:
{}
    verbs:
      - get
      - list
      - watch
", final_namespace, resources); 
                match file {
                    None => {
                        cli_stdout_printline!("{}", output)
                    },
                    Some(file) => {
                        if !file.exists() {
                            std::fs::File::create(file).unwrap();
                        } else {
                            cli_stdout_printline!("'{}' already exists, overwriting its content !", file.to_string_lossy());
                        }
                        std::fs::write(file, output).unwrap();
                    }
                }
            },
            ResourcesSubCommand::Crds { file } => {
                let exposed_resources: OtoroshExposedResources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
                let output: String = "---\n".to_owned() + &exposed_resources.resources.into_iter()
                    .map(|resource| {
                        format!("apiVersion: \"apiextensions.k8s.io/v1\"
kind: \"CustomResourceDefinition\"
metadata:
  name: \"{}.proxy.otoroshi.io\"
spec:
  group: \"proxy.otoroshi.io\"
  names:
    kind: \"{}\"
    plural: \"{}\"
    singular: \"{}\"
  scope: \"Namespaced\"
  versions:
  - name: \"v1alpha1\"
    served: false
    storage: false
    deprecated: true
    schema:
      openAPIV3Schema:
        x-kubernetes-preserve-unknown-fields: true
        type: \"object\"
  - name: \"v1\"
    served: true
    storage: true
    deprecated: false
    schema:
      openAPIV3Schema:
        x-kubernetes-preserve-unknown-fields: true
        type: \"object\"\n", resource.plural_name, resource.kind, resource.plural_name, resource.singular_name)
                    })
                    .collect::<Vec<String>>()
                    .join("---\n");
                match file {
                    None => {
                        cli_stdout_printline!("{}", output)
                    },
                    Some(file) => {
                        if !file.exists() {
                            std::fs::File::create(file).unwrap();
                        } else {
                            cli_stdout_printline!("'{}' already exists, overwriting its content !", file.to_string_lossy());
                        }
                        std::fs::write(file, output).unwrap();
                    }
                }
            },
            ResourcesSubCommand::Template { resource, kube } => {
                let resource_name = resource;
                let exposed_resources: OtoroshExposedResources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
                let exposed_resource = exposed_resources.resources.into_iter()
                    .find(|r| r.plural_name == *resource_name || r.singular_name == *resource_name || r.kind == *resource_name)
                    .unwrap();
                match Otoroshi::get_resource_template(exposed_resource.clone().plural_name, cli_opts.clone()).await {
                    Some(resp) => {
                        match cli_opts.ouput {
                            Some(str) => {
                                match str.as_str() {
                                    "json" => {
                                        cli_stdout_printline!("{}", serde_json::to_string(&resp).unwrap())
                                    },
                                    "json_pretty" => {
                                        cli_stdout_printline!("{}", serde_json::to_string_pretty(&resp).unwrap())
                                    },
                                    "pretty" => {
                                        cli_stdout_printline!("{}", serde_json::to_string_pretty(&resp).unwrap())
                                    },
                                    "yaml" => {
                                        if kube.unwrap_or(true) {
                                            let res_body = resp;
                                            let res_name = EntityHelper::extract_json_entity_name(&res_body).unwrap();
                                            let kube_res = KubeEntity::new(exposed_resource.clone().kind.clone(), res_name, res_body);
                                            cli_stdout_printline!("{}", serde_yaml::to_string(&kube_res).unwrap())
                                        } else {
                                            cli_stdout_printline!("{}", serde_yaml::to_string(&resp).unwrap())
                                        }
                                    },
                                    _ => {
                                        cli_stdout_printline!("{}", serde_yaml::to_string(&resp).unwrap())
                                    },
                                }
                            },
                            _ => {
                                cli_stdout_printline!("{}", serde_yaml::to_string(&resp).unwrap())
                            },
                        };
                    },
                    _ => {
                        cli_stderr_printline!("resource {} not found !", resource_name);
                    }
                }
            },
            ResourcesSubCommand::Get { resource, id, columns, kube, page, page_size, filters } => {
                match resource {
                    Some(resource_name) => {
                        let exposed_resources: OtoroshExposedResources = Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap();
                        let exposed_resource = exposed_resources.resources.into_iter()
                            .find(|r| r.plural_name == *resource_name || r.singular_name == *resource_name || r.kind == *resource_name)
                            .unwrap();        
                        let res_kind = exposed_resource.kind;
                        let final_resource_name: String = exposed_resource.plural_name;
                        let kind: String = exposed_resource.singular_name;
                        match id {
                            Some(resource_id) => {
                                match Otoroshi::get_one_resource(final_resource_name, resource_id.to_string(), cli_opts.clone()).await {
                                    Some(raw_resource) => {
                                        match cli_opts.ouput {
                                            Some(str) => {
                                                match str.as_str() {
                                                    "json" => cli_stdout_printline!("{}", serde_json::to_string(&Self::with_kind(&raw_resource.body, res_kind)).unwrap()),
                                                    "json_pretty" => cli_stdout_printline!("{}", serde_json::to_string_pretty(&Self::with_kind(&raw_resource.body, res_kind)).unwrap()),
                                                    "yaml" => {
                                                        if kube.unwrap_or(true) {
                                                            let res_body = raw_resource.body;
                                                            let res_name = EntityHelper::extract_json_entity_name(&res_body).unwrap();
                                                            let kube_res = KubeEntity::new(res_kind, res_name, res_body);
                                                            cli_stdout_printline!("{}", serde_yaml::to_string(&kube_res).unwrap())
                                                        } else {
                                                            cli_stdout_printline!("{}", serde_yaml::to_string(&Self::with_kind(&raw_resource.body, res_kind)).unwrap())
                                                        }
                                                    },
                                                    _ => {
                                                        let mut vec = Vec::new();
                                                        vec.push(TableResource {
                                                            raw: raw_resource.body,
                                                        });
                                                        Self::display_table_of_resources(kind.to_string(), vec, columns.to_vec())
                                                    },
                                                }
                                            },
                                            _ => {
                                                let mut vec = Vec::new();
                                                vec.push(TableResource {
                                                    raw: raw_resource.body,
                                                });
                                                Self::display_table_of_resources(kind.to_string(), vec, columns.to_vec())
                                            },
                                        };
                                    },
                                    None => {
                                        cli_stdout_printline!("resource {} with id {} not found", resource_name, resource_id)
                                    }
                                }
                            },
                            None => {
                                match Otoroshi::get_resources(final_resource_name, page.unwrap_or(1), page_size.unwrap_or(99999), filters.to_vec(), cli_opts.clone()).await {
                                    Some(raw_resources) => {
                                        match cli_opts.ouput {
                                            Some(str) => {
                                                match str.as_str() {
                                                    "json" => {
                                                        let items: Vec<serde_json::Value> = raw_resources.body.into_iter().map(|v| Self::with_kind(&v, res_kind.to_string())).collect();
                                                        cli_stdout_printline!("{}", serde_json::to_string(&items).unwrap())
                                                    },
                                                    "json_pretty" => {
                                                        let items: Vec<serde_json::Value> = raw_resources.body.into_iter().map(|v| Self::with_kind(&v, res_kind.to_string())).collect();
                                                        cli_stdout_printline!("{}", serde_json::to_string_pretty(&items).unwrap())
                                                    },
                                                    "yaml" => {
                                                        if kube.unwrap_or(true) {
                                                            let kube_entities: Vec<KubeEntity> = raw_resources.body.into_iter().map(|res_body| {
                                                                let res_name = EntityHelper::extract_json_entity_name(&res_body).unwrap();
                                                                let kube_res = KubeEntity::new(res_kind.clone(), res_name, res_body);
                                                                kube_res
                                                            }).collect();
                                                            cli_stdout_printline!("{}", serde_yaml::to_string(&kube_entities).unwrap())
                                                        } else {
                                                            let items: Vec<serde_json::Value> = raw_resources.body.into_iter().map(|v| Self::with_kind(&v, res_kind.to_string())).collect();
                                                            cli_stdout_printline!("{}", serde_yaml::to_string(&items).unwrap())
                                                        }
                                                    },
                                                    _ => {
                                                        let vec = raw_resources.body.into_iter().map(|item| {
                                                            TableResource {
                                                                raw: item
                                                            }
                                                        }).collect();
                                                        Self::display_table_of_resources(kind, vec, columns.to_vec());
                                                    },
                                                }
                                            },
                                            _ => {
                                                let vec = raw_resources.body.into_iter().map(|item| {
                                                    TableResource {
                                                        raw: item
                                                    }
                                                }).collect();
                                                Self::display_table_of_resources(kind, vec, columns.to_vec());
                                            },
                                        };
                                    },
                                    None => {
                                        cli_stdout_printline!("resources {} not found", resource_name)
                                    }
                                }
                            }
                        }
                    },
                    None => {
                        print!("no resource specified")
                    }
                }
            },
            ResourcesSubCommand::Delete { resource, ids, file, directory, recursive } => {
                match resource {
                    Some(resource) => {
                        let final_resource_name: String = if resource.ends_with("s") {
                            resource.to_string()
                        } else {
                            format!("{}s", resource)
                        };
                        let mut failed_results: Vec<String> = Vec::new();
                        for id in ids.iter() {
                            let res = Otoroshi::delete_one_resource(final_resource_name.to_string(), id.to_string(), cli_opts.clone()).await;
                            if !res {
                                failed_results.push(id.to_string());
                            }
                        }
                        if failed_results.is_empty() {
                            ()
                        } else {
                            let print_ids: Vec<String> = failed_results.into_iter().map(|s| format!("  - {}", s)).collect();
                            cli_stdout_printline!("failed to delete the following {}\n", final_resource_name);
                            cli_stdout_printline!("{}\n", print_ids.join("\n"))
                        }
                    },
                    None => {
                        match file {
                            None => {
                                match directory {
                                    None => cli_stdout_printline!("you need to provide a file or directory path"),
                                    Some(directory) => {
                                        let files = Self::find_files(directory, recursive.unwrap_or(false));
                                        Self::delete_files(files, cli_opts.clone()).await
                                    }
                                }
                            },
                            Some(file) => {
                                if file.starts_with("http://") || file.starts_with("https://") {
                                    if file.starts_with("http://") {
                                        Self::delete_url_http(file.to_owned(), cli_opts.clone()).await
                                    } else if file.starts_with("https://") {
                                        Self::delete_url_https(file.to_owned(), cli_opts.clone()).await
                                    } else {
                                        ()
                                    }
                                } else {
                                    Self::delete_files(vec![PathBuf::from(file.to_owned())], cli_opts.clone()).await
                                }
                            }
                        }
                    }
                }
            },
            ResourcesSubCommand::Create { resource, file, data, input, stdin } => {
                let final_resource_name: String = if resource.ends_with("s") {
                    resource.to_string()
                } else {
                    format!("{}s", resource)
                };
                match file {
                    Some(file) => {
                        match crate::utils::file::FileHelper::get_content_string_result(file).await {
                            Err(e) => {
                                cli_stderr_printline!("error while reading {:?}: {}", file, e);
                                std::process::exit(-1)
                            },
                            Ok(content) => {
                                let is_yaml = !content.trim().starts_with("{");
                                if is_yaml {
                                    let mut json = serde_yaml::from_str::<serde_json::Value>(&content).unwrap();
                                    let is_kube = json.get("apiVersion").is_some() && json.get("spec").is_some();
                                    if is_kube {
                                        json = json.get("spec").unwrap().clone();
                                    }
                                    let id: String = EntityHelper::extract_json_entity_id(&json).unwrap();
                                    let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id, serde_json::to_string(&json).unwrap(), cli_opts.clone()).await;
                                    ()
                                } else {
                                    let json = serde_json::from_str::<serde_json::Value>(&content).unwrap();
                                    let id = EntityHelper::extract_json_entity_id(&json).unwrap();
                                    let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id, content, cli_opts.clone()).await;
                                    ()
                                }
                            }
                        }
                    },
                    _ => {
                        if data.is_empty() {
                            if input.is_none() {
                                let to_edit = "".to_string();
                                let edited = if *stdin {
                                    std::io::read_to_string(std::io::stdin()).unwrap()
                                } else {
                                    edit::edit(to_edit.to_string()).unwrap_or(to_edit.to_string())
                                };
                                let is_yaml = !edited.trim().starts_with("{");
                                if is_yaml {
                                    let mut json = serde_yaml::from_str::<serde_json::Value>(&edited).unwrap();
                                    let is_kube = json.get("apiVersion").is_some() && json.get("spec").is_some();
                                    if is_kube {
                                        json = json.get("spec").unwrap().clone();
                                    }
                                    let id: String = EntityHelper::extract_json_entity_id(&json).unwrap();
                                    let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), serde_json::to_string(&json).unwrap(), cli_opts.clone()).await;
                                } else {
                                    let json = serde_json::from_str::<serde_json::Value>(&edited).unwrap();
                                    let id: String = EntityHelper::extract_json_entity_id(&json).unwrap();
                                    let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), edited, cli_opts.clone()).await;
                                }
                                ()
                            } else {
                                let edited = input.clone().unwrap();
                                let is_yaml = !edited.trim().starts_with("{");
                                if is_yaml {
                                    let mut json = serde_yaml::from_str::<serde_json::Value>(&edited).unwrap();
                                    let is_kube = json.get("apiVersion").is_some() && json.get("spec").is_some();
                                    if is_kube {
                                        json = json.get("spec").unwrap().clone();
                                    }
                                    let id = EntityHelper::extract_json_entity_id(&json).unwrap();
                                    let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), serde_json::to_string(&json).unwrap(), cli_opts.clone()).await;
                                } else {
                                    let json = serde_json::from_str(&edited).unwrap();
                                    let id = EntityHelper::extract_json_entity_id(&json).unwrap();
                                    let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), edited, cli_opts.clone()).await;
                                    ()
                                }
                            }
                        } else {
                            let serie: String = data.into_iter()
                                .filter(|str| str.contains("="))
                                .map(|str| {
                                    let parts: Vec<String> = str.split("=").map(|s| s.to_string()).collect();
                                    let path = parts.get(0).unwrap();
                                    let mut value = parts.get(1).unwrap().to_string();
                                    if value.starts_with("'") && value.ends_with("'") {
                                        value = value.strip_suffix("'").unwrap().strip_prefix("'").unwrap().to_string();
                                    };
                                    if value.starts_with("\"") && value.ends_with("\"") {
                                        value = value.strip_suffix("\"").unwrap().strip_prefix("\"").unwrap().to_string();
                                    };
                                    serde_json::to_string(&serde_json::json!({ "path": path, "value": value })).unwrap()
                                })
                                .collect::<Vec<String>>()
                                .join(",");
                            let _ = Otoroshi::create_one_resource_with_content_type(final_resource_name.to_string(), format!("[{}]", serie), "application/json+oto-patch".to_string(), cli_opts.clone()).await;
                            ()
                        }
                    }
                }
            }, 
            ResourcesSubCommand::Edit { resource, id, file, data, input, stdin } => {
                let final_resource_name: String = if resource.ends_with("s") {
                    resource.to_string()
                } else {
                    format!("{}s", resource)
                };
                match file {
                    Some(file) => {
                        match crate::utils::file::FileHelper::get_content_string_result(file).await {
                            Err(e) => {
                                cli_stderr_printline!("error while reading {:?}: {}", file, e);
                                std::process::exit(-1)
                            },
                            Ok(content) => {
                                let is_yaml = !content.trim().starts_with("{");
                                if is_yaml {
                                    let mut json = serde_yaml::from_str::<serde_json::Value>(&content).unwrap();
                                    let is_kube = json.get("apiVersion").is_some() && json.get("spec").is_some();
                                    if is_kube {
                                        json = json.get("spec").unwrap().clone();
                                    }
                                    let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), serde_json::to_string(&json).unwrap(), cli_opts.clone()).await;
                                } else {
                                    let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), content, cli_opts.clone()).await;
                                    ()
                                }
                            }
                        }
                    },
                    _ => {
                        match Otoroshi::get_one_resource(final_resource_name.to_string(), id.to_string(), cli_opts.clone()).await {
                            None => {
                                cli_stderr_printline!("error while fetching entity {}/{}", final_resource_name, id);
                                std::process::exit(-1)
                            },
                            Some(res) => {
                                if data.is_empty() {
                                    if input.is_none() {
                                        let to_edit = serde_json::to_string_pretty(&res.body).unwrap();
                                        let edited = if *stdin {
                                            std::io::read_to_string(std::io::stdin()).unwrap()
                                        } else {
                                            edit::edit(to_edit.to_string()).unwrap_or(to_edit.to_string())
                                        };
                                        let is_yaml = !edited.trim().starts_with("{");
                                        if is_yaml {
                                            let mut json = serde_yaml::from_str::<serde_json::Value>(&edited).unwrap();
                                            let is_kube = json.get("apiVersion").is_some() && json.get("spec").is_some();
                                            if is_kube {
                                                json = json.get("spec").unwrap().clone();
                                            }
                                            let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), serde_json::to_string(&json).unwrap(), cli_opts.clone()).await;
                                        } else {
                                            let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), edited, cli_opts.clone()).await;
                                            ()
                                        }
                                    } else {
                                        let edited = input.clone().unwrap();
                                        let is_yaml = !edited.trim().starts_with("{");
                                        if is_yaml {
                                            let mut json = serde_yaml::from_str::<serde_json::Value>(&edited).unwrap();
                                            let is_kube = json.get("apiVersion").is_some() && json.get("spec").is_some();
                                            if is_kube {
                                                json = json.get("spec").unwrap().clone();
                                            }
                                            let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), serde_json::to_string(&json).unwrap(), cli_opts.clone()).await;
                                        } else {
                                            let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), edited, cli_opts.clone()).await;
                                            ()
                                        }
                                    }
                                } else {
                                    let serie: String = data.into_iter()
                                        .filter(|str| str.contains("="))
                                        .map(|str| {
                                            let parts: Vec<String> = str.split("=").map(|s| s.to_string()).collect();
                                            let path = parts.get(0).unwrap();
                                            let mut value = parts.get(1).unwrap().to_string();
                                            if value.starts_with("'") && value.ends_with("'") {
                                                value = value.strip_suffix("'").unwrap().strip_prefix("'").unwrap().to_string();
                                            };
                                            if value.starts_with("\"") && value.ends_with("\"") {
                                                value = value.strip_suffix("\"").unwrap().strip_prefix("\"").unwrap().to_string();
                                            };
                                            serde_json::to_string(&serde_json::json!({ "path": path, "value": value })).unwrap()
                                        })
                                        .collect::<Vec<String>>()
                                        .join(",");
                                    let _ = Otoroshi::upsert_one_resource_with_content_type(final_resource_name.to_string(), id.to_string(), format!("[{}]", serie), "application/json+oto-patch".to_string(), cli_opts.clone()).await;
                                    ()
                                }
                            }
                        }
                    }
                }
            }, 
            ResourcesSubCommand::Patch { resource, id, merge, file, data, stdin } => {
                // TODO: handle json patch
                let final_resource_name: String = if resource.ends_with("s") {
                    resource.to_string()
                } else {
                    format!("{}s", resource)
                };
                match Otoroshi::get_one_resource(final_resource_name.to_string(), id.to_string(), cli_opts.clone()).await {
                    None => {
                        cli_stderr_printline!("error while fetching resource {}/{}", final_resource_name, id);
                        std::process::exit(-1)
                    },
                    Some(res) => {
                        if data.is_empty() {
                            let content: String = match file {
                                None => {
                                    if *stdin {
                                        std::io::read_to_string(std::io::stdin()).unwrap()
                                    } else {
                                        match merge {
                                            None => edit::edit("".to_string()).unwrap_or("".to_string()),
                                            Some(merge) => merge.to_string()
                                        }
                                    }
                                },
                                Some(file) => crate::utils::file::FileHelper::get_content_string(file).await
                            };
                            let is_yaml = !content.trim().starts_with("{");
                            if is_yaml {
                                let input = serde_yaml::from_str::<serde_json::Value>(&content).unwrap();
                                let mut doc = res.body;
                                doc.merge(&input);
                                let edited = serde_json::to_string(&doc).unwrap();
                                let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), serde_json::to_string(&edited).unwrap(), cli_opts.clone()).await;
                            } else {
                                let input = serde_json::from_str::<serde_json::Value>(&content).unwrap();
                                let mut doc = res.body;
                                doc.merge(&input);
                                let edited = serde_json::to_string(&doc).unwrap();
                                let _ = Otoroshi::upsert_one_resource(final_resource_name.to_string(), id.to_string(), edited, cli_opts.clone()).await;
                                ()
                            }
                        } else {
                            let serie: String = data.into_iter()
                                .filter(|str| str.contains("="))
                                .map(|str| {
                                    let parts: Vec<String> = str.split("=").map(|s| s.to_string()).collect();
                                    let path = parts.get(0).unwrap();
                                    let mut value = parts.get(1).unwrap().to_string();
                                    if value.starts_with("'") && value.ends_with("'") {
                                        value = value.strip_suffix("'").unwrap().strip_prefix("'").unwrap().to_string();
                                    };
                                    if value.starts_with("\"") && value.ends_with("\"") {
                                        value = value.strip_suffix("\"").unwrap().strip_prefix("\"").unwrap().to_string();
                                    };
                                    serde_json::to_string(&serde_json::json!({ "path": path, "value": value })).unwrap()
                                })
                                .collect::<Vec<String>>()
                                .join(",");
                            let _ = Otoroshi::upsert_one_resource_with_content_type(final_resource_name.to_string(), id.to_string(), format!("[{}]", serie), "application/json+oto-patch".to_string(), cli_opts.clone()).await;
                            ()
                        }
                    }
                }
            }, 
            ResourcesSubCommand::Apply { file, directory, recursive, watch  } => {
                match file {
                    None => {
                        match directory {
                            None => cli_stdout_printline!("you need to provide a file or directory path"),
                            Some(directory) => {
                                let files = Self::find_files(directory, recursive.unwrap_or(false));
                                Self::sync_files(files, cli_opts.clone()).await;
                                if watch.unwrap_or(false) {
                                    Self::run_watch(directory.as_os_str().to_string_lossy().to_string(), true, recursive.unwrap_or(false), cli_opts.clone());
                                }
                                ()
                            }
                        }
                    },
                    Some(file) => {
                        if file.starts_with("http://") || file.starts_with("https://") {
                            if file.starts_with("http://") {
                                Self::sync_url_http(file.to_owned(), cli_opts.clone()).await
                            } else if file.starts_with("https://") {
                                Self::sync_url_https(file.to_owned(), cli_opts.clone()).await
                            } else {
                                ()
                            }
                        } else {
                            Self::sync_files(vec![PathBuf::from(file.to_owned())], cli_opts.clone()).await;
                            if watch.unwrap_or(false) {
                                Self::run_watch(file.to_string(), false, recursive.unwrap_or(false), cli_opts.clone());
                            }
                            ()
                        }
                    }
                }
            }, 
            ResourcesSubCommand::Import { file, nd_json } => {
                let cconfig =  crate::cli::config::OtoroshiCtlConfig::get_current_config(cli_opts).await.to_connection_config();
                let content = crate::utils::file::FileHelper::get_content_string(file).await;
                let content_type = Some(nd_json.filter(|i| *i).map(|_| "application/x-ndjson".to_string()).unwrap_or("application/json".to_string()));
                let res = Otoroshi::otoroshi_call(Method::POST, "/api/otoroshi.json", None, Some(hyper::Body::from(content)), content_type, cconfig.clone()).await;
                if res.status != 200 {
                    cli_stderr_printline!("import error ! {:?}", res.body_bytes);
                    std::process::exit(-1)
                }
            }, 
            ResourcesSubCommand::Export { file, directory, split_files, kube, nd_json } => {
                match file {
                    Some(file) => {
                        if !file.exists() {
                            std::fs::File::create(file).unwrap();
                        } 
                        let accept = nd_json.filter(|i| *i).map(|_| "application/x-ndjson".to_string());
                        match Otoroshi::get_export_json(accept, cli_opts.clone()).await {
                            None => {
                                cli_stderr_printline!("error while fetching export");
                                std::process::exit(-1)
                            },
                            Some(bytes) => std::fs::write(file, bytes).unwrap(),
                        }
                    },
                    None => match directory {
                        None => {
                            cli_stderr_printline!("you have to specify a file or directory");
                            std::process::exit(-1)
                        },
                        Some(directory) => {
                            if !directory.exists() {
                                std::fs::create_dir_all(directory).unwrap();
                            }
                            for resource in Otoroshi::get_exposed_resources(cli_opts.clone()).await.unwrap().resources.into_iter() {
                                let name = resource.plural_name.to_string();
                                let name2 = resource.plural_name.to_string();
                                let res = Otoroshi::get_resources(name.to_string(), 1, 99999, Vec::new(), cli_opts.clone()).await.unwrap();
                                if split_files.unwrap_or(false) {
                                    if !res.body.is_empty() {
                                        std::fs::create_dir_all(directory.join(name)).unwrap();
                                    }
                                    for entity in res.body {
                                        let entity_id = EntityHelper::extract_json_entity_id(&entity).unwrap();
                                        let entity_name = EntityHelper::extract_json_entity_name(&entity).unwrap();
                                        if cli_opts.clone().ouput.filter(|v| *v == "yaml").is_some() && kube.unwrap_or(true) {
                                            let kube_entity = KubeEntity::new(resource.clone().kind, entity_name, entity);
                                            std::fs::write(
                                                directory.join(name2.to_string()).join(format!("{}.yaml", entity_id.to_string())), 
                                                serde_yaml::to_string(&kube_entity).unwrap()
                                            ).unwrap();
                                        } else if cli_opts.clone().ouput.filter(|v| *v == "yaml").is_some() && !kube.unwrap_or(true) {
                                            let mut kind: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                                            kind.insert("kind".to_string(), serde_json::Value::String(resource.clone().kind));
                                            let mut entity_with_kind = entity.clone();
                                            entity_with_kind.merge(&serde_json::Value::Object(kind));
                                            std::fs::write(
                                                directory.join(name2.to_string()).join(format!("{}.yaml", entity_id.to_string())), 
                                                serde_yaml::to_string(&entity_with_kind).unwrap()
                                            ).unwrap();
                                        } else {
                                            let mut kind: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                                            kind.insert("kind".to_string(), serde_json::Value::String(resource.clone().kind));
                                            let mut entity_with_kind = entity.clone();
                                            entity_with_kind.merge(&serde_json::Value::Object(kind));
                                            std::fs::write(
                                                directory.join(name2.to_string()).join(format!("{}.json", entity_id.to_string())), 
                                                serde_json::to_string_pretty(&entity_with_kind).unwrap()
                                            ).unwrap();
                                        }
                                    }
                                } else {
                                    let kinded_docs: Vec<serde_json::Value> = res.body.into_iter().map(|mut i| {
                                        let mut kind: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                                        kind.insert("kind".to_string(), serde_json::Value::String(resource.clone().kind));
                                        i.merge(&serde_json::Value::Object(kind));
                                        i
                                    }).collect();
                                    let doc = serde_json::Value::Array(kinded_docs);
                                    std::fs::write(
                                        directory.join(format!("{}.json", name.to_string())), 
                                        serde_json::to_string_pretty(&doc).unwrap()
                                    ).unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
