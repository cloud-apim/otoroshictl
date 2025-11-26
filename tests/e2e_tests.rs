mod common;

use common::{OtoroshiCtl, TestData};
use predicates::prelude::*;
use serial_test::serial;

// =============================================================================
// End-to-End tests (require Otoroshi)
// These tests simulate complete user workflows
// Run with: cargo test --test e2e_tests -- --ignored --test-threads=1
// =============================================================================

/// Test the typical GitOps workflow:
/// 1. Check cluster health
/// 2. Get available entities
/// 3. Apply resources from YAML
/// 4. Export state
/// 5. Delete resources
/// 6. Re-import from export
#[test]
#[ignore]
#[serial]
fn test_gitops_workflow() {
    let cli = OtoroshiCtl::new();
    let test_data = TestData::new();
    let export_file = test_data.dir.path().join("gitops-export.json");

    // 1. Check cluster is healthy
    cli.run(&["health"]).success();

    // 2. List available entity types
    let output = cli.run_success(&["entities"]);
    assert!(output.contains("routes") || output.contains("Route"));

    // 3. Apply test resources
    cli.run(&["resources", "apply", "-f", test_data.route_path()])
        .success();

    // 4. Verify resource was created
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("test-route-1"));

    // 5. Export current state
    cli.run(&["resources", "export", "-f", export_file.to_str().unwrap()])
        .success();

    // 6. Delete resources
    cli.run(&["resources", "delete", "routes", "test-route-1"])
        .success();

    // 7. Verify deleted
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("not found"));

    // 8. Re-import from export
    cli.run(&["resources", "import", "-f", export_file.to_str().unwrap()])
        .success();

    // 9. Verify restored
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("test-route-1"));

    // Cleanup
    cli.run(&["resources", "delete", "routes", "test-route-1"])
        .success();
}

/// Test the cluster information workflow:
/// Verify all cluster info commands work together
#[test]
#[ignore]
#[serial]
fn test_cluster_info_workflow() {
    let cli = OtoroshiCtl::new();

    // Version
    let version_output = cli.run_success(&["version"]);
    assert!(!version_output.is_empty());

    // Version as JSON
    let version_json = cli.run_success(&["version", "-o", "json"]);
    let _: serde_json::Value =
        serde_json::from_str(&version_json).expect("Version should return valid JSON");

    // Health
    cli.run(&["health"]).success();

    // Infos
    let infos_output = cli.run_success(&["infos"]);
    assert!(infos_output.contains("cluster_id") || infos_output.contains("version"));

    // Metrics
    cli.run(&["metrics"]).success();

    // Entities
    let entities_output = cli.run_success(&["entities"]);
    assert!(entities_output.contains("routes") || entities_output.contains("Route"));
}

/// Test configuration management with resources
#[test]
#[ignore]
#[serial]
fn test_config_context_with_resources() {
    let cli = OtoroshiCtl::new();

    // Verify we're using the right context
    cli.run(&["config", "current-context"])
        .success()
        .stdout(predicate::str::contains("default"));

    // Verify we can list various resource types with this config
    let resource_types = ["routes", "backends", "apikeys", "certificates"];

    for resource_type in resource_types {
        cli.run(&["resources", "get", resource_type]).success();
    }
}

/// Test template generation:
/// Verify templates are generated correctly for different resource types
#[test]
#[ignore]
#[serial]
fn test_template_generation() {
    let cli = OtoroshiCtl::new();

    // Generate templates for different resource types
    let resource_types = ["route", "backend", "apikey"];

    for resource_type in resource_types {
        // Default output (table/raw)
        cli.run(&["resources", "template", resource_type])
            .success()
            .stdout(predicate::str::is_empty().not());

        // YAML output
        cli.run(&["resources", "template", resource_type, "-o", "yaml"])
            .success()
            .stdout(predicate::str::is_empty().not());

        // JSON output
        let json_output = cli.run_success(&["resources", "template", resource_type, "-o", "json"]);
        let _: serde_json::Value = serde_json::from_str(&json_output)
            .unwrap_or_else(|_| panic!("Template for {} should be valid JSON", resource_type));
    }
}

/// Test multi-resource management
#[test]
#[ignore]
#[serial]
fn test_multi_resource_workflow() {
    let cli = OtoroshiCtl::new();
    let test_data = TestData::new();

    // Apply multiple routes from single file
    cli.run(&["resources", "apply", "-f", test_data.multi_route_path()])
        .success();

    // Verify both routes exist
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("test-route-1"));

    cli.run(&["resources", "get", "route", "test-route-2"])
        .success()
        .stdout(predicate::str::contains("test-route-2"));

    // Delete both at once
    cli.run(&[
        "resources",
        "delete",
        "routes",
        "test-route-1",
        "test-route-2",
    ])
    .success();

    // Verify both deleted
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("not found"));

    cli.run(&["resources", "get", "route", "test-route-2"])
        .success()
        .stdout(predicate::str::contains("not found"));
}
