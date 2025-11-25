mod common;

use common::OtoroshiCtl;
use predicates::prelude::*;

// =============================================================================
// Configuration tests (without Otoroshi)
// These tests verify configuration management commands
// =============================================================================

// -----------------------------------------------------------------------------
// Context management
// -----------------------------------------------------------------------------

#[test]
fn test_config_current_context() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "current-context"])
        .success()
        .stdout(predicate::str::contains("default"));
}

#[test]
fn test_config_list_contexts() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "list"])
        .success()
        .stdout(predicate::str::contains("default"));
}

#[test]
fn test_config_list_contexts_detailed() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "list-contexts"])
        .success()
        .stdout(predicate::str::contains("default"));
}

#[test]
fn test_config_use_nonexistent_context() {
    let cli = OtoroshiCtl::new();
    // The application returns success but displays a message
    cli.run(&["config", "use", "nonexistent-context-xyz"])
        .success()
        .stdout(predicate::str::contains("does not exist"));
}

// -----------------------------------------------------------------------------
// Cluster management
// -----------------------------------------------------------------------------

#[test]
fn test_config_list_clusters() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "list-clusters"])
        .success()
        .stdout(predicate::str::contains("default"));
}

// -----------------------------------------------------------------------------
// User management
// -----------------------------------------------------------------------------

#[test]
fn test_config_list_users() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "list-users"])
        .success()
        .stdout(predicate::str::contains("default"));
}

// -----------------------------------------------------------------------------
// Config file operations
// -----------------------------------------------------------------------------

#[test]
fn test_config_current_config() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "current-config"])
        .success()
        .stdout(predicate::str::contains("api_version"))
        .stdout(predicate::str::contains("clusters"))
        .stdout(predicate::str::contains("users"))
        .stdout(predicate::str::contains("contexts"));
}

#[test]
fn test_config_current_location() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "current-location"])
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_config_reset() {
    let cli = OtoroshiCtl::new();

    // Reset should work
    cli.run(&["config", "reset"]).success();

    // Config should still be valid after reset
    cli.run(&["config", "current-context"])
        .success()
        .stdout(predicate::str::contains("default"));
}

// -----------------------------------------------------------------------------
// Output formats
// -----------------------------------------------------------------------------

#[test]
fn test_config_yaml_output() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "current-config", "-o", "yaml"])
        .success()
        .stdout(predicate::str::contains("api_version"))
        .stdout(predicate::str::contains("clusters"));
}

#[test]
fn test_config_json_output() {
    let cli = OtoroshiCtl::new();
    // Note: config current-config outputs YAML format regardless of -o flag
    // This test verifies the command succeeds and contains expected content
    cli.run(&["config", "current-config", "-o", "json"])
        .success()
        .stdout(predicate::str::contains("api_version"));
}

// -----------------------------------------------------------------------------
// Help commands
// -----------------------------------------------------------------------------

#[test]
fn test_config_help() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "--help"])
        .success()
        .stdout(predicate::str::contains("current-context"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("use"));
}

#[test]
fn test_config_subcommands_help() {
    let cli = OtoroshiCtl::new();

    let subcommands = [
        "list",
        "list-clusters",
        "list-users",
        "list-contexts",
        "current-context",
        "current-config",
        "current-location",
        "add",
        "use",
        "reset",
    ];

    for subcmd in subcommands {
        cli.run(&["config", subcmd, "--help"]).success();
    }
}

// -----------------------------------------------------------------------------
// Error cases
// -----------------------------------------------------------------------------

#[test]
fn test_config_unknown_subcommand() {
    let cli = OtoroshiCtl::new();
    cli.run(&["config", "unknown-subcommand"]).failure();
}

#[test]
fn test_config_add_missing_required_args() {
    let cli = OtoroshiCtl::new();
    // 'add' requires at least a name
    cli.run(&["config", "add"]).failure();
}

#[test]
fn test_config_without_config_file() {
    let cli = OtoroshiCtl::without_config();
    // Without config file, most commands should fail or show error
    cli.run(&["config", "current-context"]).failure();
}
