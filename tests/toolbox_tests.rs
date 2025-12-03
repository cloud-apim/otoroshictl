mod common;

use common::OtoroshiCtl;
use predicates::prelude::*;

// =============================================================================
// Toolbox add-mailer tests
// =============================================================================

// -----------------------------------------------------------------------------
// CLI structure tests (without Otoroshi)
// -----------------------------------------------------------------------------

#[test]
fn test_toolbox_help() {
    let cli = OtoroshiCtl::new();
    cli.run(&["toolbox", "--help"])
        .success()
        .stdout(predicate::str::contains("add-mailer"))
        .stdout(predicate::str::contains("mtls"))
        .stdout(predicate::str::contains("open"));
}

#[test]
fn test_toolbox_add_mailer_help() {
    let cli = OtoroshiCtl::new();
    cli.run(&["toolbox", "add-mailer", "--help"])
        .success()
        .stdout(predicate::str::contains("SMTP"))
        .stdout(predicate::str::contains("--host"))
        .stdout(predicate::str::contains("--port"))
        .stdout(predicate::str::contains("--user"))
        .stdout(predicate::str::contains("--smtps"))
        .stdout(predicate::str::contains("--starttls"));
}

#[test]
fn test_toolbox_alias_works() {
    let cli = OtoroshiCtl::new();
    cli.run(&["tb", "add-mailer", "--help"])
        .success()
        .stdout(predicate::str::contains("SMTP"));
}

#[test]
fn test_toolbox_add_mailer_env_vars_documented() {
    let cli = OtoroshiCtl::new();
    cli.run(&["toolbox", "add-mailer", "--help"])
        .success()
        .stdout(predicate::str::contains("OTOROSHI_MAILER_HOST"))
        .stdout(predicate::str::contains("OTOROSHI_MAILER_PORT"))
        .stdout(predicate::str::contains("OTOROSHI_MAILER_USER"));
}

#[test]
fn test_toolbox_add_mailer_default_port_documented() {
    let cli = OtoroshiCtl::new();
    cli.run(&["toolbox", "add-mailer", "--help"])
        .success()
        // Default port 465 is mentioned in the description
        .stdout(predicate::str::contains("465"));
}

// -----------------------------------------------------------------------------
// Integration tests with Otoroshi (marked #[ignore])
// Run with: cargo test --test toolbox_tests -- --ignored
// -----------------------------------------------------------------------------

#[test]
#[ignore]
fn test_add_mailer_success() {
    // Note: This test requires:
    // - A running Otoroshi instance
    // - Environment variables OTOROSHI_MAILER_* configured
    let cli = OtoroshiCtl::new();
    cli.run(&["toolbox", "add-mailer"])
        .success()
        .stdout(predicate::str::contains("Route ID"))
        .stdout(predicate::str::contains("Client ID"))
        .stdout(predicate::str::contains("Bearer Token"));
}

#[test]
#[ignore]
fn test_add_mailer_json_output() {
    let cli = OtoroshiCtl::new();
    let output = cli.run_success(&["toolbox", "add-mailer", "-o", "json"]);

    let json: serde_json::Value = serde_json::from_str(&output).expect("Invalid JSON");
    assert!(json.get("route").is_some());
    assert!(json.get("apikey").is_some());
    assert!(json.get("url").is_some());

    // Verify nested structure
    let route = json.get("route").unwrap();
    assert!(route.get("id").is_some());
    assert!(route.get("name").is_some());

    let apikey = json.get("apikey").unwrap();
    assert!(apikey.get("clientId").is_some());
    assert!(apikey.get("clientSecret").is_some());
    assert!(apikey.get("bearerToken").is_some());
}

#[test]
#[ignore]
fn test_add_mailer_yaml_output() {
    let cli = OtoroshiCtl::new();
    cli.run(&["toolbox", "add-mailer", "-o", "yaml"])
        .success()
        .stdout(predicate::str::contains("route:"))
        .stdout(predicate::str::contains("apikey:"))
        .stdout(predicate::str::contains("url:"));
}

#[test]
#[ignore]
fn test_add_mailer_with_flags() {
    let cli = OtoroshiCtl::new();
    cli.run(&[
        "toolbox",
        "add-mailer",
        "--host",
        "smtp.example.com",
        "--port",
        "465",
        "--user",
        "test@example.com",
        "--smtps",
        "--starttls",
    ])
    .success()
    .stdout(predicate::str::contains("Mailer created successfully"));
}

// =============================================================================
// Toolbox open tests
// =============================================================================

// -----------------------------------------------------------------------------
// CLI structure tests (without Otoroshi)
// -----------------------------------------------------------------------------

#[test]
fn test_toolbox_open_help() {
    let cli = OtoroshiCtl::new();
    cli.run(&["toolbox", "open", "--help"])
        .success()
        .stdout(predicate::str::contains("backoffice"));
}

// -----------------------------------------------------------------------------
// Integration tests with Otoroshi (marked #[ignore])
// Run with: cargo test --test toolbox_tests -- --ignored
// -----------------------------------------------------------------------------

#[test]
#[ignore]
fn test_toolbox_open_requires_backoffice_url() {
    // Test case: Otoroshi < 17.9 without backoffice_url
    let cli = OtoroshiCtl::new();
    cli.run(&["toolbox", "open"])
        .failure()
        .stderr(predicate::str::contains("Otoroshi 17.9"));
}
