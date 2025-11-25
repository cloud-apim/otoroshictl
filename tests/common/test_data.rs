use std::path::PathBuf;
use tempfile::TempDir;

/// Test route YAML content
#[allow(dead_code)]
const ROUTE_YAML: &str = r#"apiVersion: proxy.otoroshi.io/v1
kind: Route
metadata:
  name: test-route-1
spec:
  _loc:
    tenant: default
    teams:
      - default
  id: test-route-1
  name: Test Route 1
  description: A test route for integration tests
  tags: []
  metadata: {}
  enabled: true
  debug_flow: false
  export_reporting: false
  capture: false
  groups:
    - default
  bound_listeners: []
  frontend:
    domains:
      - test-route.oto.tools
    strip_path: true
    exact: false
    headers: {}
    query: {}
    methods: []
    cookies: {}
  backend:
    targets:
      - id: target_1
        hostname: request.otoroshi.io
        port: 443
        tls: true
        weight: 1
        backup: false
        predicate:
          type: AlwaysMatch
        protocol: HTTP/1.1
        ip_address: ~
        tls_config:
          certs: []
          trusted_certs: []
          enabled: false
          loose: false
          trust_all: false
    root: /
    rewrite: false
    load_balancing:
      type: RoundRobin
    client:
      retries: 1
      max_errors: 20
      retry_initial_delay: 50
      backoff_factor: 2
      call_timeout: 30000
      call_and_stream_timeout: 120000
      connection_timeout: 10000
      idle_timeout: 60000
      global_timeout: 30000
      sample_interval: 2000
      proxy: {}
      custom_timeouts: []
      cache_connection_settings:
        enabled: false
        queue_size: 2048
    health_check:
      enabled: false
      url: ""
      timeout: 5000
      healthyStatuses: []
      unhealthyStatuses: []
  backend_ref: ~
  plugins:
    - enabled: true
      debug: false
      plugin: cp:otoroshi.next.plugins.OverrideHost
      include: []
      exclude: []
      config: {}
      bound_listeners: []
"#;

/// Second test route for multi-resource tests
#[allow(dead_code)]
const ROUTE_2_YAML: &str = r#"apiVersion: proxy.otoroshi.io/v1
kind: Route
metadata:
  name: test-route-2
spec:
  _loc:
    tenant: default
    teams:
      - default
  id: test-route-2
  name: Test Route 2
  description: Second test route
  tags: []
  metadata: {}
  enabled: true
  debug_flow: false
  export_reporting: false
  capture: false
  groups:
    - default
  bound_listeners: []
  frontend:
    domains:
      - test-route-2.oto.tools
    strip_path: true
    exact: false
    headers: {}
    query: {}
    methods: []
    cookies: {}
  backend:
    targets:
      - id: target_1
        hostname: request.otoroshi.io
        port: 443
        tls: true
        weight: 1
        backup: false
        predicate:
          type: AlwaysMatch
        protocol: HTTP/1.1
        ip_address: ~
        tls_config:
          certs: []
          trusted_certs: []
          enabled: false
          loose: false
          trust_all: false
    root: /
    rewrite: false
    load_balancing:
      type: RoundRobin
  backend_ref: ~
  plugins: []
"#;

/// Invalid YAML for error testing
#[allow(dead_code)]
const INVALID_YAML: &str = r#"
not: valid: yaml: {{
  broken: [
"#;

/// Valid YAML but invalid Otoroshi resource
#[allow(dead_code)]
const INVALID_RESOURCE_YAML: &str = r#"apiVersion: proxy.otoroshi.io/v1
kind: UnknownResource
metadata:
  name: invalid
spec:
  id: invalid
"#;

/// Test data directory with pre-created files
#[allow(dead_code)]
pub struct TestData {
    pub dir: TempDir,
    pub route_file: PathBuf,
    pub route_2_file: PathBuf,
    pub multi_route_file: PathBuf,
    pub invalid_yaml_file: PathBuf,
    pub invalid_resource_file: PathBuf,
}

#[allow(dead_code)]
impl TestData {
    /// Create a new test data directory with all test files
    pub fn new() -> Self {
        let dir = TempDir::new().expect("Failed to create temp dir for test data");

        let route_file = dir.path().join("route1.yaml");
        std::fs::write(&route_file, ROUTE_YAML).expect("Failed to write route1.yaml");

        let route_2_file = dir.path().join("route2.yaml");
        std::fs::write(&route_2_file, ROUTE_2_YAML).expect("Failed to write route2.yaml");

        // Multi-document file with both routes
        let multi_route_file = dir.path().join("routes.yaml");
        let multi_content = format!("{}\n---\n{}", ROUTE_YAML.trim(), ROUTE_2_YAML.trim());
        std::fs::write(&multi_route_file, multi_content).expect("Failed to write routes.yaml");

        let invalid_yaml_file = dir.path().join("invalid.yaml");
        std::fs::write(&invalid_yaml_file, INVALID_YAML).expect("Failed to write invalid.yaml");

        let invalid_resource_file = dir.path().join("invalid_resource.yaml");
        std::fs::write(&invalid_resource_file, INVALID_RESOURCE_YAML)
            .expect("Failed to write invalid_resource.yaml");

        Self {
            dir,
            route_file,
            route_2_file,
            multi_route_file,
            invalid_yaml_file,
            invalid_resource_file,
        }
    }

    /// Get path as string for CLI arguments
    pub fn route_path(&self) -> &str {
        self.route_file.to_str().unwrap()
    }

    pub fn route_2_path(&self) -> &str {
        self.route_2_file.to_str().unwrap()
    }

    pub fn multi_route_path(&self) -> &str {
        self.multi_route_file.to_str().unwrap()
    }

    pub fn invalid_yaml_path(&self) -> &str {
        self.invalid_yaml_file.to_str().unwrap()
    }

    pub fn invalid_resource_path(&self) -> &str {
        self.invalid_resource_file.to_str().unwrap()
    }
}

impl Default for TestData {
    fn default() -> Self {
        Self::new()
    }
}
