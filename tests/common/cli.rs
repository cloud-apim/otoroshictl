use assert_cmd::Command;
use std::path::PathBuf;
use tempfile::TempDir;

/// Get the path to the compiled binary
fn binary_path() -> PathBuf {
    // Use CARGO_BIN_EXE which is set by cargo test
    PathBuf::from(env!("CARGO_BIN_EXE_otoroshictl"))
}

/// Default test configuration for Otoroshi
const DEFAULT_TEST_CONFIG: &str = r#"api_version: v1
kind: OtoroshiCtlConfig
metadata: {}
cloud_apim: ~
users:
  - name: default
    client_id: admin-api-apikey-id
    client_secret: admin-api-apikey-secret
    health_key: test-health-key
contexts:
  - name: default
    cluster: default
    user: default
    cloud_apim: false
clusters:
  - name: default
    hostname: otoroshi-api.oto.tools
    ip_addresses:
      - 127.0.0.1
    port: 8080
    tls: false
    client_cert: ~
    routing_hostname: ~
    routing_port: ~
    routing_tls: ~
    routing_ip_addresses: ~
current_context: default
"#;

/// Helper to execute otoroshictl commands in tests
pub struct OtoroshiCtl {
    /// Keep TempDir alive to prevent deletion while struct exists
    #[allow(dead_code)]
    config_dir: TempDir,
    config_file: PathBuf,
}

impl OtoroshiCtl {
    /// Create a new instance with a temporary configuration
    pub fn new() -> Self {
        let config_dir = TempDir::new().expect("Failed to create temp dir");
        let config_file = config_dir.path().join("config.yaml");

        std::fs::write(&config_file, DEFAULT_TEST_CONFIG).expect("Failed to write test config");

        Self {
            config_dir,
            config_file,
        }
    }

    /// Create an instance without configuration (to test errors)
    #[allow(dead_code)]
    pub fn without_config() -> Self {
        let config_dir = TempDir::new().expect("Failed to create temp dir");
        let config_file = config_dir.path().join("config.yaml");
        // Do not write a config file

        Self {
            config_dir,
            config_file,
        }
    }

    /// Return a Command configured with the config file
    pub fn cmd(&self) -> Command {
        let mut cmd = Command::new(binary_path());
        cmd.arg("--config-file").arg(&self.config_file);
        cmd
    }

    /// Execute a command and return the assertion
    pub fn run(&self, args: &[&str]) -> assert_cmd::assert::Assert {
        self.cmd().args(args).assert()
    }

    /// Execute a command and return stdout on success
    #[allow(dead_code)]
    pub fn run_success(&self, args: &[&str]) -> String {
        let output = self.cmd().args(args).assert().success();
        String::from_utf8_lossy(&output.get_output().stdout).to_string()
    }
}

impl Default for OtoroshiCtl {
    fn default() -> Self {
        Self::new()
    }
}
