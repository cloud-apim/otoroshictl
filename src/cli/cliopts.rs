use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[clap(name = "otoroshictl")]
#[clap(author = "Mathieu ANCELIN (Cloud APIM)")]
#[clap(version = "1.0.0")]
#[clap(about = "Manage your otoroshi clusters with style. otoroshictl is a CLI that can interact with the admin api of an otoroshi cluster. \n\nYou can also use it to expose local process through the otoroshi remote tunnels feature and as an universal sidecar to create a service mesh based on otoroshi. otoroshictl also provide a nice integration with Cloud APIM. \n\notoroshictl is an open-source tool proudly provided by Cloud APIM (https://www.cloud-apim.com). Cloud APIM is a company that provides managed Otoroshi clusters and Wasmo instances perfectly configured and optimized, ready in seconds. The sources of otoroshictl are available on github at https://github.com/cloud-apim/otoroshictl", long_about = None)]
#[clap(arg_required_else_help(true))]
pub struct CliOpts {
    /// Turn debugging information on
    #[arg(short, long, global = true, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Change the rendering format (can be one of: json, yaml, json_pretty)
    #[arg(short, long, global = true, value_name = "FORMAT")]
    pub ouput: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE or URL", global = true)]
    pub config_file: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Sets the tls flag to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, action = clap::ArgAction::SetTrue)]
    pub otoroshi_cluster_tls: bool,
    /// Sets the hostname to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "HOSTNAME")]
    pub otoroshi_cluster_hostname: Option<String>,
    /// Sets the port to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "PORT")]
    pub otoroshi_cluster_port: Option<u16>,
    /// Sets the tls flag to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, action = clap::ArgAction::SetTrue)]
    pub otoroshi_cluster_routing_tls: Option<bool>,
    /// Sets the hostname to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "HOSTNAME")]
    pub otoroshi_cluster_routing_hostname: Option<String>,
    /// Sets the port to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "PORT")]
    pub otoroshi_cluster_routing_port: Option<u16>,
    /// Sets the client_id to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "CLIENT_ID")]
    pub otoroshi_user_client_id: Option<String>,
    /// Sets the client_secret to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "CLIENT_SECRET")]
    pub otoroshi_user_client_secret: Option<String>,
    /// Sets the health_key to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "HEALTH_KEY")]
    pub otoroshi_user_health_key: Option<String>,
    /// Sets the client cert location to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "FILE")]
    pub otoroshi_cluster_cert_location: Option<String>,
    /// Sets the client cert key location to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "FILE")]
    pub otoroshi_cluster_key_location: Option<String>,
    /// Sets the client cert ca location to connect to a custom otoroshi cluster without using a config file
    #[arg(long, global = true, value_name = "FILE")]
    pub otoroshi_cluster_ca_location: Option<String>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ConfigSubCommand {
    /// Display the current config. file content
    CurrentConfig {},
    /// Edit the current config. file
    EditCurrentConfig {},
    /// Display current config. location
    CurrentLocation {},
    /// Display current context
    CurrentContext {},
    /// Set the current context
    UseContext {
        /// Name of the context
        name: String,
    },
    /// Set the current context
    Use {
        /// Name of the context
        name: String,
    },
    /// Rename a context
    RenameContext {
        /// Name of the context
        old_name: String,
        /// New name of the context
        new_name: String,
    },
    /// Display the list of usable contexts
    List {},
    /// Display the list of clusters
    ListClusters {},
    /// Display the list of users
    ListUsers {},
    /// Display the list of contexts
    ListContexts {},
    /// Create or update a cluster
    SetCluster {
        /// Name of the cluster
        name: String,
        /// hostname of the cluster api
        #[arg(long)]
        hostname: String,
        /// port of the cluster api
        #[arg(long)]
        port: u16,
        /// does the cluster api uses tls
        #[arg(long, action = clap::ArgAction::SetTrue)]
        tls: bool,
        /// hostname used for routing (sidecar only)
        #[arg(long)]
        routing_hostname: Option<String>,
        /// port used for routing (sidecar only)
        #[arg(long)]
        routing_port: Option<u16>,
        /// tls flag used for routing (sidecar only)
        #[arg(long, action = clap::ArgAction::SetTrue)]
        routing_tls: Option<bool>,
    },
    /// Create or update a user
    SetUser {
        /// Name of the cluster
        name: String,
        /// client_id of the cluster api
        #[arg(long)]
        client_id: String,
        /// client_secret of the cluster api
        #[arg(long)]
        client_secret: String,
        /// health_access_key of the cluster for health and metrics
        #[arg(long)]
        health_key: Option<String>,
    },
    /// Create or update a context
    SetContext {
        /// Name of the cluster
        name: String,
        /// cluster name for this context
        #[arg(long)]
        cluster: String,
        /// user name for this context
        #[arg(long)]
        user: String,
    },
    /// Create and set a full config
    Add {
        /// Name of the cluster
        name: String,
        /// client_id of the cluster api
        #[arg(long)]
        client_id: Option<String>,
        /// client_secret of the cluster api
        #[arg(long)]
        client_secret: Option<String>,
        /// health_access_key of the cluster for health and metrics
        #[arg(long)]
        health_key: Option<String>,
        /// hostname of the cluster api
        #[arg(long)]
        hostname: Option<String>,
        /// port of the cluster api
        #[arg(long)]
        port: Option<u16>,
        /// does the cluster api uses tls
        #[arg(long, action = clap::ArgAction::SetTrue)]
        tls: bool,
        /// Change current context for this one
        #[arg(long, action = clap::ArgAction::SetTrue)]
        current: bool,
        /// hostname used for routing (sidecar only)
        #[arg(long)]
        routing_hostname: Option<String>,
        /// port used for routing (sidecar only)
        #[arg(long)]
        routing_port: Option<u16>,
        /// tls flag used for routing (sidecar only)
        #[arg(long, action = clap::ArgAction::SetTrue)]
        routing_tls: Option<bool>,
        /// Clever Cloud API token for authentication
        #[arg(long)]
        clever_token: Option<String>,
        /// Otoroshi ID to import from Clever Cloud
        #[arg(long)]
        clever_otoroshi_id: Option<String>,
    },
    /// Delete a cluster
    DeleteCluster {
        /// Name of the cluster
        name: String,
    },
    /// Delete a user
    DeleteUser {
        /// Name of the user
        name: String,
    },
    /// Delete a context
    DeleteContext {
        /// Name of the context
        name: String,
    },
    /// Delete a full context with the associated cluster and user
    Delete {
        /// Name of the context
        name: Option<String>,
    },
    /// Delete configuration and start with a clean one
    Reset {},
    /// Import a context file with current context file
    Import {
        /// The name of the context you want to import
        #[arg(short, long, value_name = "NAME")]
        name: Option<String>,
        /// Change current context to imported one
        #[arg(long, action = clap::ArgAction::SetTrue)]
        current: bool,
        /// Overwrite
        #[arg(long, action = clap::ArgAction::SetTrue)]
        overwrite: bool,
        /// The file or url containing the json object to merge
        file: Option<String>,
        /// Read from stdin
        #[arg(long, action = clap::ArgAction::SetTrue)]
        stdin: bool,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ResourcesSubCommand {
    /// Generate a template for the current kind
    Template {
        /// Resource name to operate on
        resource: String,
        /// Add kube armor to resources
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        kube: Option<bool>,
    },
    /// Generate crds manifest for kubernetes
    Crds {
        #[arg(short, long, value_name = "FILE")] // ok just file because writing
        file: Option<PathBuf>,
    },
    /// Generate rbac manifest for kubernetes       
    Rbac {
        #[arg(short, long, value_name = "FILE")] // ok just file because writing
        file: Option<PathBuf>,
        /// the namespace used for the ServiceAccount
        #[arg(long)]
        namespace: Option<String>,
        /// the username used for the ServiceAccount, ClusterRoleBinding, ClusterRole
        #[arg(long)]
        username: Option<String>,
    },
    /// Get otoroshi resource from current cluster
    Get {
        /// Optional resource name to operate on
        resource: Option<String>,
        /// Optional resource id to operate on
        id: Option<String>,
        /// Optional comma separated list of columns to display
        #[arg(long, global = true)]
        columns: Vec<String>,
        /// Add kube armor to resources
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        kube: Option<bool>,
        /// The viewed page
        #[arg(long)]
        page: Option<u32>,
        /// The viewed page size
        #[arg(long)]
        page_size: Option<u32>,
        /// Filter the returned elements
        #[arg(short, long)]
        filters: Vec<String>,
    },
    /// Delete otoroshi resources
    Delete {
        /// Optional resource name to operate on
        resource: Option<String>,
        /// the ids to delete
        ids: Vec<String>,
        /// The file to delete
        #[arg(short, long, value_name = "FILE or URL")]
        file: Option<String>,
        /// The directory to delete
        #[arg(short, long, value_name = "DIR")]
        directory: Option<PathBuf>,
        /// Walk through sub directories
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        recursive: Option<bool>,
    },
    /// Update otoroshi resources through json merge or json patch
    Patch {
        /// The resource name to operate on
        resource: String,
        /// The resource id to operate on
        id: String,
        /// The json object to merge
        merge: Option<String>,
        /// The file cxontaining the json object to merge
        #[arg(short, long, value_name = "FILE or URL")]
        file: Option<String>,
        /// Use inline PATH=VALUE tuples as entity input
        #[arg(long, value_name = "PATH=VALUE")]
        data: Vec<String>,
        /// Use stdin as entity input
        #[clap(long, action, default_value = "false")]
        stdin: bool,
    },
    /// Update otoroshi resources
    Edit {
        /// The resource name to operate on
        resource: String,
        /// The resource id to operate on
        id: String,
        /// The file to sync
        #[arg(short, long, value_name = "FILE or URL")]
        file: Option<String>,
        /// Use inline PATH=VALUE tuples as entity input
        #[arg(long, value_name = "PATH=VALUE")]
        data: Vec<String>,
        /// The optional inline entity input
        input: Option<String>,
        /// Use stdin as entity input
        #[clap(long, action, default_value = "false")]
        stdin: bool,
    },
    /// Create otoroshi resources
    Create {
        /// The resource name to operate on
        resource: String,
        /// The file to sync
        #[arg(short, long, value_name = "FILE or URL")]
        file: Option<String>,
        /// Use inline PATH=VALUE tuples as entity input
        #[arg(long, value_name = "PATH=VALUE")]
        data: Vec<String>,
        /// The optional inline entity input
        input: Option<String>,
        /// Use stdin as entity input
        #[clap(long, action, default_value = "false")]
        stdin: bool,
    },
    /// Synchronise otoroshi resources from files or directories
    Apply {
        /// The file to sync
        #[arg(short, long, value_name = "FILE or URL")]
        file: Option<String>,
        /// The directory to sync
        #[arg(short, long, value_name = "DIR")]
        directory: Option<PathBuf>,
        /// Walk through sub directories
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        recursive: Option<bool>,
        /// Keep watching file changes
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        watch: Option<bool>,
    },
    /// Export otoroshi resources to files or directories
    Export {
        #[arg(short, long, value_name = "FILE")] // ok just file because writing
        file: Option<PathBuf>,
        /// The directory to sync
        #[arg(short, long, value_name = "DIR")]
        directory: Option<PathBuf>,
        /// Split the export into one entity per file
        #[arg(long, action = clap::ArgAction::SetTrue)]
        split_files: Option<bool>,
        /// Split the export into one entity per file
        #[arg(long, action = clap::ArgAction::SetTrue)]
        kube: Option<bool>,
        /// Export in ndjson format
        #[arg(long, action = clap::ArgAction::SetTrue)]
        nd_json: Option<bool>,
    },
    /// Import data from an export file
    Import {
        /// The file to import
        #[arg(short, long, value_name = "FILE or URL")]
        file: String,
        /// import from ndjson format
        #[arg(long, action = clap::ArgAction::SetTrue)]
        nd_json: Option<bool>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum SidecarSubCommand {
    /// Display instructions to install/run the sidecar
    Howto {},
    /// Run otoroshi sidecar
    Run {
        /// The sidecar config file
        #[arg(short, long, value_name = "FILE or URL")]
        file: Option<String>,
        // /// The sidecar dns server port
        // #[arg(long, value_name = "PORT")]
        // dns_port: Option<u16>,
    },
    GenerateConfig {
        /// The sidecar config file to generate
        #[arg(short, long, value_name = "FILE")]
        file: Option<String>,
    },
    /// Install transparent proxing of the mesh calls through iptables rules
    Install {
        /// Dry run, do not apply the changes
        #[arg(long, action = clap::ArgAction::SetTrue)]
        dry_run: Option<bool>,
        /// The sidecar config file
        #[arg(short, long, value_name = "FILE or URL")]
        file: Option<String>,
        /// The user that will run the sidecar process using the dedicated user or runuser
        #[arg(short, long, value_name = "USER")]
        user: String,
    },
    /// Uninstall transparent proxing of the mesh calls through iptables rules
    Uninstall {
        /// Dry run, do not apply the changes
        #[arg(long, action = clap::ArgAction::SetTrue)]
        dry_run: Option<bool>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum CloudApimSubCommand {
    /// Login to your cloud-apim account
    Login,
    /// List your deployments
    List,
    /// Logout from your cloud-apim account
    Logout,
    /// Add the cluster to the possible otoroshictl configs
    Link {
        /// Name of the deployment
        name: String,
    },
    /// Add the cluster to the possible otoroshictl configs and set it as the current one
    Use {
        /// Name of the deployment
        name: String,
    },
    /// Restart this otoroshi cluster on cloud-apim
    Restart {
        /// Name of the deployment
        name: String,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Manage all the resources (entities) of the current otoroshi cluster
    Resources {
        #[command(subcommand)]
        command: ResourcesSubCommand,
    },
    /// Manage an otoroshi mesh sidecar
    Sidecar {
        #[command(subcommand)]
        command: SidecarSubCommand,
    },
    /// Manage otoroshi tcp tunnel to access tcp resources through the current otoroshi cluster (not implemented yet)
    TcpTunnel {},
    /// Manage cloud apim clusters
    CloudApim {
        #[command(subcommand)]
        command: CloudApimSubCommand,
    },
    /// Exposes local processes on the current otoroshi cluster through the otoroshi remote tunnel feature
    RemoteTunnel {
        /// the local host forwarded to
        #[clap(long, default_value = "localhost")]
        local_host: String,
        /// the local port forwarded to
        #[clap(long, default_value = "8080")]
        local_port: i32,
        /// local process exposed as tls ?
        #[clap(long, action, default_value = "false")]
        local_tls: bool,
        /// enable expose mode
        #[clap(long, action, default_value = "false")]
        expose: bool,
        /// the exposed domain
        #[clap(long)]
        remote_domain: Option<String>,
        /// the exposed subdomain
        #[clap(long)]
        remote_subdomain: Option<String>,
        /// enable tls want mode
        #[clap(long, action, default_value = "false")]
        tls: bool,
        /// the tunnel id
        #[clap(long, default_value = "cli")]
        tunnel: String,
    },
    /// Display the version of the current otoroshi cluster
    Version {},
    /// Display the informations about the current otoroshi cluster
    Infos {},
    /// Display the managed entities of the current otoroshi cluster
    Entities {},
    /// Display the health status of the current otoroshi cluster
    Health {},
    /// Display metrics of the current otoroshi cluster
    Metrics {
        /// Optional comma separated list of columns to display
        #[arg(long)]
        columns: Vec<String>,
        /// Optional comma separated filters for metrics name
        #[arg(long)]
        filters: Option<String>,
    },
    /// Manage all the otoroshi cluster configurations you want to connect to with otoroshictl
    Config {
        #[command(subcommand)]
        command: ConfigSubCommand,
    },
}

impl CliOpts {
    pub fn build_from_command_line() -> CliOpts {
        CliOpts::parse()
    }
}
