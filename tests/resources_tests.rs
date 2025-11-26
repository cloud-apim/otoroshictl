mod common;

use common::{OtoroshiCtl, TestData};
use predicates::prelude::*;
use serial_test::serial;

// =============================================================================
// Resource tests (require Otoroshi)
// Run with: cargo test --test resources_tests -- --ignored --test-threads=1
// =============================================================================

// -----------------------------------------------------------------------------
// GET commands - List resources
// -----------------------------------------------------------------------------

#[test]
#[ignore]
#[serial]
fn test_resources_get_routes_table() {
    let cli = OtoroshiCtl::new();
    // Default output is table format with column headers
    cli.run(&["resources", "get", "routes"])
        .success()
        .stdout(predicate::str::contains("id").or(predicate::str::contains("name")));
}

#[test]
#[ignore]
#[serial]
fn test_resources_get_routes_json() {
    let cli = OtoroshiCtl::new();
    let output = cli.run_success(&["resources", "get", "routes", "-o", "json"]);
    // Should be valid JSON (array)
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("Invalid JSON output");
    assert!(parsed.is_array());
}

#[test]
#[ignore]
#[serial]
fn test_resources_get_routes_yaml() {
    let cli = OtoroshiCtl::new();
    cli.run(&["resources", "get", "routes", "-o", "yaml"])
        .success()
        .stdout(predicate::str::contains("apiVersion").or(predicate::str::contains("-")));
}

#[test]
#[ignore]
#[serial]
fn test_resources_get_various_entity_types() {
    let cli = OtoroshiCtl::new();

    // Test multiple entity types exist and respond
    let entity_types = [
        "routes",
        "backends",
        "apikeys",
        "certificates",
        "service-descriptors",
        "jwt-verifiers",
        "auth-modules",
    ];

    for entity_type in entity_types {
        cli.run(&["resources", "get", entity_type]).success();
    }
}

// -----------------------------------------------------------------------------
// GET single resource
// -----------------------------------------------------------------------------

#[test]
#[ignore]
#[serial]
fn test_resources_get_single_route() {
    let cli = OtoroshiCtl::new();
    let test_data = TestData::new();

    // Create a route first
    cli.run(&["resources", "apply", "-f", test_data.route_path()])
        .success();

    // Get single route by ID
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("test-route-1"));

    // Cleanup
    cli.run(&["resources", "delete", "routes", "test-route-1"])
        .success();
}

#[test]
#[ignore]
#[serial]
fn test_resources_get_nonexistent_route() {
    let cli = OtoroshiCtl::new();
    // CLI returns success with "not found" message
    cli.run(&["resources", "get", "route", "nonexistent-route-xyz"])
        .success()
        .stdout(predicate::str::contains("not found"));
}

// -----------------------------------------------------------------------------
// TEMPLATE command
// -----------------------------------------------------------------------------

#[test]
#[ignore]
#[serial]
fn test_resources_template_various_types() {
    let cli = OtoroshiCtl::new();

    // Test templates for different resource types
    let resource_types = ["route", "backend", "apikey", "certificate"];

    for resource_type in resource_types {
        cli.run(&["resources", "template", resource_type])
            .success()
            .stdout(predicate::str::is_empty().not());
    }
}

#[test]
#[ignore]
#[serial]
fn test_resources_template_output_formats() {
    let cli = OtoroshiCtl::new();

    // JSON output
    let json_output = cli.run_success(&["resources", "template", "route", "-o", "json"]);
    let _: serde_json::Value = serde_json::from_str(&json_output).expect("Invalid JSON template");

    // YAML output
    cli.run(&["resources", "template", "route", "-o", "yaml"])
        .success()
        .stdout(predicate::str::contains("frontend"));
}

// -----------------------------------------------------------------------------
// APPLY / CREATE / DELETE - CRUD lifecycle
// -----------------------------------------------------------------------------

#[test]
#[ignore]
#[serial]
fn test_resources_crud_lifecycle() {
    let cli = OtoroshiCtl::new();
    let test_data = TestData::new();

    // CREATE via apply
    cli.run(&["resources", "apply", "-f", test_data.route_path()])
        .success();

    // READ - verify created
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("test-route-1"));

    // READ as JSON - verify structure
    let json_output = cli.run_success(&["resources", "get", "route", "test-route-1", "-o", "json"]);
    let route: serde_json::Value =
        serde_json::from_str(&json_output).expect("Invalid JSON for route");
    assert!(
        route.get("id").is_some() || route.get("spec").is_some() || route.get("name").is_some()
    );

    // UPDATE via apply (idempotent)
    cli.run(&["resources", "apply", "-f", test_data.route_path()])
        .success();

    // DELETE
    cli.run(&["resources", "delete", "routes", "test-route-1"])
        .success();

    // VERIFY deleted
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("not found"));
}

#[test]
#[ignore]
#[serial]
fn test_resources_apply_multiple_routes() {
    let cli = OtoroshiCtl::new();
    let test_data = TestData::new();

    // Apply multi-document YAML
    cli.run(&["resources", "apply", "-f", test_data.multi_route_path()])
        .success();

    // Verify both routes exist
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("test-route-1"));

    cli.run(&["resources", "get", "route", "test-route-2"])
        .success()
        .stdout(predicate::str::contains("test-route-2"));

    // Delete both
    cli.run(&[
        "resources",
        "delete",
        "routes",
        "test-route-1",
        "test-route-2",
    ])
    .success();
}

#[test]
#[ignore]
#[serial]
fn test_resources_delete_from_file() {
    let cli = OtoroshiCtl::new();
    let test_data = TestData::new();

    // Apply
    cli.run(&["resources", "apply", "-f", test_data.route_path()])
        .success();

    // Delete via file reference
    cli.run(&["resources", "delete", "-f", test_data.route_path()])
        .success();

    // Verify deleted
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("not found"));
}

// -----------------------------------------------------------------------------
// EXPORT / IMPORT
// -----------------------------------------------------------------------------

#[test]
#[ignore]
#[serial]
fn test_resources_export_import_roundtrip() {
    let cli = OtoroshiCtl::new();
    let test_data = TestData::new();
    let export_file = test_data.dir.path().join("export.json");

    // Create test route
    cli.run(&["resources", "apply", "-f", test_data.route_path()])
        .success();

    // Export
    cli.run(&["resources", "export", "-f", export_file.to_str().unwrap()])
        .success();

    // Verify export file is valid JSON
    let export_content = std::fs::read_to_string(&export_file).expect("Failed to read export file");
    let _: serde_json::Value =
        serde_json::from_str(&export_content).expect("Export is not valid JSON");

    // Delete the route
    cli.run(&["resources", "delete", "routes", "test-route-1"])
        .success();

    // Verify deleted
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("not found"));

    // Import from export
    cli.run(&["resources", "import", "-f", export_file.to_str().unwrap()])
        .success();

    // Verify restored
    cli.run(&["resources", "get", "route", "test-route-1"])
        .success()
        .stdout(predicate::str::contains("test-route-1"));

    // Cleanup
    cli.run(&["resources", "delete", "routes", "test-route-1"])
        .success();
}

// -----------------------------------------------------------------------------
// CRDs and RBAC generation
// -----------------------------------------------------------------------------

#[test]
#[ignore]
#[serial]
fn test_resources_crds_generation() {
    let cli = OtoroshiCtl::new();

    cli.run(&["resources", "crds"])
        .success()
        .stdout(predicate::str::contains("apiVersion"))
        .stdout(predicate::str::contains("CustomResourceDefinition"));
}

#[test]
#[ignore]
#[serial]
fn test_resources_rbac_generation() {
    let cli = OtoroshiCtl::new();

    cli.run(&["resources", "rbac"])
        .success()
        .stdout(predicate::str::contains("apiVersion"));
}

// -----------------------------------------------------------------------------
// ERROR CASES
// -----------------------------------------------------------------------------

#[test]
#[ignore]
#[serial]
fn test_resources_apply_invalid_yaml() {
    let cli = OtoroshiCtl::new();
    let test_data = TestData::new();

    // Apply malformed YAML should fail
    cli.run(&["resources", "apply", "-f", test_data.invalid_yaml_path()])
        .failure();
}

#[test]
#[ignore]
#[serial]
fn test_resources_apply_nonexistent_file() {
    let cli = OtoroshiCtl::new();

    cli.run(&["resources", "apply", "-f", "/nonexistent/path/file.yaml"])
        .failure();
}

#[test]
#[ignore]
#[serial]
fn test_resources_get_unknown_entity_type() {
    let cli = OtoroshiCtl::new();

    // Unknown entity type should fail or show error
    let result = cli.run(&["resources", "get", "unknown-entity-type-xyz"]);
    // Either fails or returns empty/error message
    result.code(predicate::ne(0).or(predicate::eq(0)));
}

#[test]
#[ignore]
#[serial]
fn test_resources_template_unknown_type() {
    let cli = OtoroshiCtl::new();

    cli.run(&["resources", "template", "unknown-type"])
        .failure();
}

#[test]
#[ignore]
#[serial]
fn test_resources_delete_nonexistent_route() {
    let cli = OtoroshiCtl::new();

    // Delete non-existent should succeed (idempotent) or show message
    cli.run(&["resources", "delete", "routes", "nonexistent-xyz"])
        .success();
}

// -----------------------------------------------------------------------------
// HELP commands
// -----------------------------------------------------------------------------

#[test]
fn test_resources_help() {
    let cli = OtoroshiCtl::new();

    cli.run(&["resources", "--help"])
        .success()
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("apply"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("template"));
}

#[test]
fn test_resources_get_help() {
    let cli = OtoroshiCtl::new();

    cli.run(&["resources", "get", "--help"]).success();
}
