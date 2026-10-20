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
        .stdout(predicate::str::contains(
            "Secure backend access via Otoroshi Communication Protocol",
        ));
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
        // Health check exclusions
        .stdout(predicate::str::contains("--exclude-path"))
        .stdout(predicate::str::contains("CC_HEALTH_CHECK_PATH"))
        // Environment variables
        .stdout(predicate::str::contains("OTOROSHI_CHALLENGE_SECRET"))
        .stdout(predicate::str::contains("OTOROSHI_CHALLENGE_EXCLUDE_PATHS"))
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
        .stdout(predicate::str::contains(
            "Secure backend access via Otoroshi Communication Protocol",
        ));
}

// -----------------------------------------------------------------------------
// Excluded paths: list parsing (flag repetition, comma separation, env var)
// -----------------------------------------------------------------------------

/// Start the proxy (V1, no secret needed) on a free port with the given args/env, capture the
/// startup banner, then kill it.
fn proxy_banner(args: &[&str], envs: &[(&str, &str)], expect: Option<&str>) -> String {
    use std::io::Read;
    use std::process::{Command, Stdio};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    let bin = env!("CARGO_BIN_EXE_otoroshictl");
    let port = {
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        l.local_addr().unwrap().port()
    };
    let port_s = port.to_string();
    let mut cmd = Command::new(bin);
    cmd.args(["challenge", "proxy", "--v1", "--port", &port_s])
        .args(args)
        .env_remove("OTOROSHI_CHALLENGE_EXCLUDE_PATHS")
        .env_remove("CC_HEALTH_CHECK_PATH")
        .env_remove("CC_HEALTH_CHECK_PATH_0")
        .env_remove("CC_HEALTH_CHECK_PATH_1")
        .env_remove("CC_HEALTH_CHECK_PATH_2")
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let mut child = cmd.spawn().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    // Reader thread: accumulates stdout until the pipe closes (when the child is killed).
    let buffer: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let reader_buffer = buffer.clone();
    let reader = std::thread::spawn(move || {
        let mut chunk = [0u8; 4096];
        while let Ok(n) = stdout.read(&mut chunk) {
            if n == 0 {
                break;
            }
            reader_buffer
                .lock()
                .unwrap()
                .push_str(&String::from_utf8_lossy(&chunk[..n]));
        }
    });

    // Wait for the expected line when given (deterministic for positive tests), otherwise wait
    // for the "Forwarding" line plus a grace period (the bypass line, if any, follows it).
    let start = Instant::now();
    let mut seen_forwarding_at: Option<Instant> = None;
    while start.elapsed() < Duration::from_secs(5) {
        let out = buffer.lock().unwrap().clone();
        if let Some(line) = expect
            && out.contains(line)
        {
            break;
        }
        match (out.contains("Forwarding requests to"), seen_forwarding_at) {
            (true, None) => seen_forwarding_at = Some(Instant::now()),
            (true, Some(t)) if expect.is_none() && t.elapsed() > Duration::from_millis(300) => {
                break;
            }
            _ => {}
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let _ = child.kill();
    let _ = child.wait();
    let _ = reader.join();
    buffer.lock().unwrap().clone()
}

#[test]
fn test_exclude_path_repeated_and_comma_separated() {
    let expected = "Challenge bypassed for GET/HEAD on: /a, /b, /c, /d";
    let out = proxy_banner(
        &[
            "--exclude-path",
            "/a",
            "--exclude-path",
            "/b,/c/",
            "--exclude-path",
            "d",
        ],
        &[],
        Some(expected),
    );
    assert!(out.contains(expected), "unexpected banner: {out}");
}

#[test]
fn test_exclude_path_from_env_and_clever_cloud_variables() {
    let expected = "Challenge bypassed for GET/HEAD on: /env1, /env2, /health, /hc0 (from --exclude-path and Clever Cloud health check variables)";
    let out = proxy_banner(
        &[],
        &[
            ("OTOROSHI_CHALLENGE_EXCLUDE_PATHS", "/env1,/env2"),
            ("CC_HEALTH_CHECK_PATH", "/health"),
            ("CC_HEALTH_CHECK_PATH_0", "/hc0"),
            ("CC_HEALTH_CHECK_PATH_1", "/env1"), // duplicate, must appear once
        ],
        Some(expected),
    );
    assert!(out.contains(expected), "unexpected banner: {out}");
}

#[test]
fn test_no_exclude_path_prints_no_bypass_line() {
    let out = proxy_banner(&[], &[], None);
    assert!(out.contains("Forwarding requests to"), "banner: {out}");
    assert!(!out.contains("Challenge bypassed"), "banner: {out}");
}
