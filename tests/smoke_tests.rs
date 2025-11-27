mod common;

use common::OtoroshiCtl;
use predicates::prelude::*;

// =============================================================================
// Smoke tests - Basic CLI functionality
// =============================================================================

// -----------------------------------------------------------------------------
// CLI structure tests (without Otoroshi)
// -----------------------------------------------------------------------------

#[test]
fn test_help_displays_usage() {
    let cli = OtoroshiCtl::new();
    cli.run(&["--help"])
        .success()
        .stdout(predicate::str::contains("otoroshi"))
        .stdout(predicate::str::contains("Usage"))
        .stdout(predicate::str::contains("Commands"));
}

#[test]
fn test_version_flag() {
    let cli = OtoroshiCtl::new();
    cli.run(&["--version"])
        .success()
        .stdout(predicate::str::contains("otoroshictl"));
}

#[test]
fn test_unknown_command_fails() {
    let cli = OtoroshiCtl::new();
    cli.run(&["unknown-command-xyz"])
        .failure()
        .stderr(predicate::str::contains("error").or(predicate::str::contains("unrecognized")));
}

#[test]
fn test_subcommands_have_help() {
    let cli = OtoroshiCtl::new();

    let subcommands = [
        "config",
        "resources",
        "challenge",
        "version",
        "health",
        "infos",
        "entities",
        "metrics",
    ];

    for cmd in subcommands {
        cli.run(&[cmd, "--help"]).success();
    }
}

// -----------------------------------------------------------------------------
// Connectivity tests with Otoroshi (marked #[ignore])
// Run with: cargo test --test smoke_tests -- --ignored
// -----------------------------------------------------------------------------

#[test]
#[ignore]
fn test_version_from_otoroshi() {
    let cli = OtoroshiCtl::new();
    cli.run(&["version"])
        .success()
        .stdout(predicate::str::contains("version").or(predicate::str::contains("major")));
}

#[test]
#[ignore]
fn test_version_json_output() {
    let cli = OtoroshiCtl::new();
    let output = cli.run_success(&["version", "-o", "json"]);

    // Should be valid JSON with version info
    let json: serde_json::Value = serde_json::from_str(&output).expect("Invalid JSON");
    assert!(json.get("version").is_some() || json.get("major").is_some() || json.is_object());
}

#[test]
#[ignore]
fn test_version_yaml_output() {
    let cli = OtoroshiCtl::new();
    cli.run(&["version", "-o", "yaml"])
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
#[ignore]
fn test_health_endpoint() {
    let cli = OtoroshiCtl::new();
    cli.run(&["health"])
        .success()
        .stdout(predicate::str::contains("health").or(predicate::str::contains("otoroshi")));
}

#[test]
#[ignore]
fn test_health_json_output() {
    let cli = OtoroshiCtl::new();
    let output = cli.run_success(&["health", "-o", "json"]);

    // Should be valid JSON
    let json: serde_json::Value = serde_json::from_str(&output).expect("Invalid JSON");
    assert!(json.is_object());
}

#[test]
#[ignore]
fn test_infos_endpoint() {
    let cli = OtoroshiCtl::new();
    cli.run(&["infos"])
        .success()
        .stdout(predicate::str::contains("cluster_id").or(predicate::str::contains("version")));
}

#[test]
#[ignore]
fn test_infos_json_output() {
    let cli = OtoroshiCtl::new();
    let output = cli.run_success(&["infos", "-o", "json"]);

    let json: serde_json::Value = serde_json::from_str(&output).expect("Invalid JSON");
    assert!(json.is_object() || json.is_array());
}

#[test]
#[ignore]
fn test_entities_list() {
    let cli = OtoroshiCtl::new();
    cli.run(&["entities"])
        .success()
        .stdout(predicate::str::contains("routes"))
        .stdout(predicate::str::contains("kind").or(predicate::str::contains("Route")));
}

#[test]
#[ignore]
fn test_entities_json_output() {
    let cli = OtoroshiCtl::new();
    let output = cli.run_success(&["entities", "-o", "json"]);

    let json: serde_json::Value = serde_json::from_str(&output).expect("Invalid JSON");
    // Can be array or object depending on output format
    assert!(json.is_array() || json.is_object());
}

#[test]
#[ignore]
fn test_metrics_endpoint() {
    let cli = OtoroshiCtl::new();
    cli.run(&["metrics"])
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
#[ignore]
fn test_metrics_json_output() {
    let cli = OtoroshiCtl::new();
    let output = cli.run_success(&["metrics", "-o", "json"]);

    let json: serde_json::Value = serde_json::from_str(&output).expect("Invalid JSON");
    assert!(json.is_object() || json.is_array());
}

// -----------------------------------------------------------------------------
// All output formats for each command
// -----------------------------------------------------------------------------

#[test]
#[ignore]
fn test_all_commands_support_output_formats() {
    let cli = OtoroshiCtl::new();

    let commands: &[&[&str]] = &[
        &["version"],
        &["health"],
        &["infos"],
        &["entities"],
        &["metrics"],
    ];

    let formats = ["json", "yaml"];

    for cmd in commands {
        for format in &formats {
            let mut args = cmd.to_vec();
            args.extend(&["-o", format]);
            cli.run(&args).success();
        }
    }
}
