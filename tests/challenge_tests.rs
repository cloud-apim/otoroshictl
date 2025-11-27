mod common;

use common::OtoroshiCtl;
use predicates::prelude::*;

// =============================================================================
// Challenge tests - CLI functionality for the Otoroshi challenge proxy
// =============================================================================

// -----------------------------------------------------------------------------
// CLI structure tests (without running the proxy)
// -----------------------------------------------------------------------------

#[test]
fn test_challenge_help() {
    let cli = OtoroshiCtl::new();
    cli.run(&["challenge", "--help"])
        .success()
        .stdout(predicate::str::contains("Secure backend access via Otoroshi Communication Protocol"));
}

#[test]
fn test_challenge_proxy_help() {
    let cli = OtoroshiCtl::new();
    cli.run(&["challenge", "proxy", "--help"])
        .success()
        // Core options
        .stdout(predicate::str::contains("--secret"))
        .stdout(predicate::str::contains("--port"))
        .stdout(predicate::str::contains("--backend-port"))
        .stdout(predicate::str::contains("--backend-host"))
        .stdout(predicate::str::contains("--state-header"))
        .stdout(predicate::str::contains("--state-resp-header"))
        .stdout(predicate::str::contains("--timeout"))
        .stdout(predicate::str::contains("--v1"))
        // Additional options
        .stdout(predicate::str::contains("--secret-base64"))
        .stdout(predicate::str::contains("--token-ttl"))
        // Default values
        .stdout(predicate::str::contains("[default: 8080]"))
        .stdout(predicate::str::contains("[default: 9000]"))
        // Environment variables
        .stdout(predicate::str::contains("OTOROSHI_SECRET"))
        // Protocol description
        .stdout(predicate::str::contains("V1 protocol"));
}

#[test]
fn test_challenge_proxy_v2_missing_secret_fails() {
    let cli = OtoroshiCtl::new();
    // V2 mode (default) requires a secret
    cli.run(&["challenge", "proxy"]).failure();
}

#[test]
fn test_challenge_alias_works() {
    let cli = OtoroshiCtl::new();
    // The "ch" alias should work the same as "challenge"
    cli.run(&["ch", "--help"])
        .success()
        .stdout(predicate::str::contains("Secure backend access via Otoroshi Communication Protocol"));
}
