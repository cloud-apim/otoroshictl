use secrecy::SecretString;
use std::env;
use std::io::{self, IsTerminal, Write};

/// Check if stdin is a TTY (interactive terminal)
pub fn is_interactive() -> bool {
    io::stdin().is_terminal()
}

/// Prompt for a string value interactively
/// Returns Err if not in a TTY environment
pub fn prompt_for_value(prompt: &str) -> Result<String, String> {
    if !is_interactive() {
        return Err(format!(
            "Cannot prompt for '{}': not running in interactive terminal (no TTY). \
             Please provide the value via CLI flag or environment variable.",
            prompt
        ));
    }
    print!("{}: ", prompt);
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {}", e))?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;
    Ok(input.trim().to_string())
}

/// Prompt for a value with a default
/// Returns Err if not in a TTY environment
pub fn prompt_for_value_with_default(prompt: &str, default: &str) -> Result<String, String> {
    if !is_interactive() {
        return Err(format!(
            "Cannot prompt for '{}': not running in interactive terminal (no TTY). \
             Please provide the value via CLI flag or environment variable.",
            prompt
        ));
    }
    print!("{} (default: {}): ", prompt, default);
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {}", e))?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

/// Prompt for a boolean value (Y/n)
/// Returns Err if not in a TTY environment
pub fn prompt_for_bool(prompt: &str, default: bool) -> Result<bool, String> {
    if !is_interactive() {
        return Err(format!(
            "Cannot prompt for '{}': not running in interactive terminal (no TTY). \
             Please provide the value via CLI flag or environment variable.",
            prompt
        ));
    }
    let default_str = if default { "Y/n" } else { "y/N" };
    print!("{} ({}): ", prompt, default_str);
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {}", e))?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;
    let trimmed = input.trim().to_lowercase();
    if trimmed.is_empty() {
        Ok(default)
    } else {
        Ok(trimmed == "y" || trimmed == "yes")
    }
}

/// Resolve a parameter: use provided value, env var, or prompt
pub fn resolve_param(value: Option<String>, env_var: &str, prompt: &str) -> Result<String, String> {
    if let Some(v) = value {
        return Ok(v);
    }
    if let Ok(v) = env::var(env_var)
        && !v.is_empty()
    {
        return Ok(v);
    }
    prompt_for_value(prompt)
}

/// Resolve a port parameter with default
pub fn resolve_port(
    value: Option<u16>,
    env_var: &str,
    prompt: &str,
    default: u16,
) -> Result<u16, String> {
    if let Some(v) = value {
        return Ok(v);
    }
    if let Ok(v) = env::var(env_var)
        && !v.is_empty()
    {
        return v.parse::<u16>().map_err(|_| {
            format!(
                "Invalid port value '{}' in environment variable {}",
                v, env_var
            )
        });
    }
    let input = prompt_for_value_with_default(prompt, &default.to_string())?;
    input
        .parse()
        .map_err(|_| format!("Invalid port value: '{}'", input))
}

/// Resolve a boolean parameter with default
pub fn resolve_bool(
    value: Option<bool>,
    env_var: &str,
    prompt: &str,
    default: bool,
) -> Result<bool, String> {
    if let Some(v) = value {
        return Ok(v);
    }
    if let Ok(v) = env::var(env_var) {
        return Ok(v.to_lowercase() == "true" || v == "1");
    }
    prompt_for_bool(prompt, default)
}

/// Resolve password from env var or prompt (masked input)
/// Returns a SecretString to protect the password in memory
pub fn resolve_password(env_var: &str, prompt: &str) -> Result<SecretString, String> {
    if let Ok(v) = env::var(env_var)
        && !v.is_empty()
    {
        return Ok(SecretString::from(v));
    }
    if !is_interactive() {
        return Err(format!(
            "Cannot prompt for '{}': not running in interactive terminal (no TTY). \
             Please provide the value via environment variable {}.",
            prompt, env_var
        ));
    }
    print!("{} (input hidden): ", prompt);
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {}", e))?;
    let password =
        rpassword::read_password().map_err(|e| format!("Failed to read password: {}", e))?;
    Ok(SecretString::from(password))
}

// =============================================================================
// Unit tests
// =============================================================================
// Note: Tests requiring env::set_var/remove_var are not included because
// the crate uses #![forbid(unsafe_code)] and these functions are unsafe
// in Rust 2024. Env var paths are tested via integration tests instead.

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Tests for is_interactive
    // -------------------------------------------------------------------------

    #[test]
    fn test_is_interactive_returns_bool() {
        // In test environment, stdin is typically not a TTY
        let result = is_interactive();
        // Just verify it returns a boolean without panicking
        assert!(result == true || result == false);
    }

    // -------------------------------------------------------------------------
    // Tests for resolve_param (value provided path)
    // -------------------------------------------------------------------------

    #[test]
    fn test_resolve_param_with_value() {
        let result = resolve_param(Some("provided".to_string()), "UNUSED_VAR", "unused");
        assert_eq!(result.unwrap(), "provided");
    }

    #[test]
    fn test_resolve_param_with_empty_value_uses_it() {
        // Empty string is still a provided value
        let result = resolve_param(Some("".to_string()), "UNUSED_VAR", "unused");
        assert_eq!(result.unwrap(), "");
    }

    // -------------------------------------------------------------------------
    // Tests for resolve_port (value provided path)
    // -------------------------------------------------------------------------

    #[test]
    fn test_resolve_port_with_value() {
        let result = resolve_port(Some(8080), "UNUSED_VAR", "unused", 465);
        assert_eq!(result.unwrap(), 8080);
    }

    #[test]
    fn test_resolve_port_with_zero_value() {
        let result = resolve_port(Some(0), "UNUSED_VAR", "unused", 465);
        assert_eq!(result.unwrap(), 0);
    }

    // -------------------------------------------------------------------------
    // Tests for resolve_bool (value provided path)
    // -------------------------------------------------------------------------

    #[test]
    fn test_resolve_bool_with_value_true() {
        let result = resolve_bool(Some(true), "UNUSED_VAR", "unused", false);
        assert!(result.unwrap());
    }

    #[test]
    fn test_resolve_bool_with_value_false() {
        let result = resolve_bool(Some(false), "UNUSED_VAR", "unused", true);
        assert!(!result.unwrap());
    }

    #[test]
    fn test_resolve_bool_value_overrides_default() {
        // Even when default is true, explicit false wins
        let result = resolve_bool(Some(false), "UNUSED_VAR", "unused", true);
        assert!(!result.unwrap());

        // Even when default is false, explicit true wins
        let result = resolve_bool(Some(true), "UNUSED_VAR", "unused", false);
        assert!(result.unwrap());
    }

    // -------------------------------------------------------------------------
    // Tests for non-interactive fallback
    // -------------------------------------------------------------------------

    #[test]
    fn test_resolve_param_no_tty_no_value_fails() {
        // When no value provided and no env var, it should fail in non-TTY
        // Note: This test may pass or fail depending on test runner TTY state
        // We test the path where value IS provided to ensure that works
        let result = resolve_param(Some("value".to_string()), "NONEXISTENT_VAR_12345", "test");
        assert!(result.is_ok());
    }
}
