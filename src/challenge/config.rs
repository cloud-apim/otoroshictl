//! Configuration for the Otoroshi Challenge Proxy.

use base64::Engine;
use http::header::HeaderName;
use std::net::SocketAddr;
use std::time::Duration;

use crate::challenge::error::ConfigError;

// Re-export shared constants from protocol module
pub use crate::otoroshi::protocol::{
    DEFAULT_STATE_HEADER, DEFAULT_STATE_RESP_HEADER, DEFAULT_TOKEN_EXPIRY_SECONDS,
};

/// Default port for the proxy to listen on.
pub const DEFAULT_LISTEN_PORT: u16 = 8080;

/// Default port for the backend application.
pub const DEFAULT_BACKEND_PORT: u16 = 9000;

/// Default backend host.
pub const DEFAULT_BACKEND_HOST: &str = "127.0.0.1";

/// Default timeout for backend requests in seconds.
pub const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;

/// Alias for backward compatibility.
pub const DEFAULT_TOKEN_TTL_SECS: i64 = DEFAULT_TOKEN_EXPIRY_SECONDS;

/// Protocol version for Otoroshi challenge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    /// V1: Simple echo of the state header value.
    V1,
    /// V2: JWT-based challenge/response with HMAC-SHA512.
    V2,
}

/// Proxy configuration built from CLI arguments.
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// Socket address to listen on.
    pub listen_addr: SocketAddr,
    /// Full URL to the backend server.
    pub backend_url: String,
    /// Shared secret for JWT signing/verification as raw bytes (required for V2).
    pub secret: Option<Vec<u8>>,
    /// Header name for incoming challenge token.
    pub state_header: HeaderName,
    /// Header name for outgoing response token.
    pub state_resp_header: HeaderName,
    /// Timeout for backend requests.
    pub request_timeout: Duration,
    /// JWT token TTL in seconds.
    pub token_ttl: i64,
    /// Protocol version (V1 or V2).
    pub version: ProtocolVersion,
}

impl ProxyConfig {
    /// Create a new configuration from CLI arguments.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        port: u16,
        backend_host: String,
        backend_port: u16,
        secret: Option<String>,
        secret_base64: bool,
        state_header: String,
        state_resp_header: String,
        timeout_secs: u64,
        token_ttl: i64,
        use_v1: bool,
    ) -> Result<Self, ConfigError> {
        let state_header = HeaderName::from_bytes(state_header.as_bytes()).map_err(|e| {
            ConfigError::InvalidHeader {
                name: "state_header",
                source: e,
            }
        })?;

        let state_resp_header =
            HeaderName::from_bytes(state_resp_header.as_bytes()).map_err(|e| {
                ConfigError::InvalidHeader {
                    name: "state_resp_header",
                    source: e,
                }
            })?;

        let version = if use_v1 {
            ProtocolVersion::V1
        } else {
            ProtocolVersion::V2
        };

        // Validate TTL is positive
        if token_ttl <= 0 {
            return Err(ConfigError::InvalidTokenTtl(token_ttl));
        }

        // Validate ports are non-zero
        if port == 0 || backend_port == 0 {
            return Err(ConfigError::InvalidPort);
        }

        // Validate backend host
        if backend_host.is_empty() {
            return Err(ConfigError::InvalidBackendHost(
                "host cannot be empty".to_string(),
            ));
        }
        if backend_host.chars().any(|c| c.is_whitespace()) {
            return Err(ConfigError::InvalidBackendHost(
                "host cannot contain whitespace".to_string(),
            ));
        }

        // Decode secret from base64 if requested, otherwise use as UTF-8 bytes
        let secret_bytes = match secret {
            Some(s) if secret_base64 => Some(base64::engine::general_purpose::STANDARD.decode(&s)?),
            Some(s) => Some(s.into_bytes()),
            None => None,
        };

        Ok(ProxyConfig {
            listen_addr: SocketAddr::from(([0, 0, 0, 0], port)),
            backend_url: format!("http://{}:{}", backend_host, backend_port),
            secret: secret_bytes,
            state_header,
            state_resp_header,
            request_timeout: Duration::from_secs(timeout_secs),
            token_ttl,
            version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new_with_defaults_v2() {
        let config = ProxyConfig::new(
            DEFAULT_LISTEN_PORT,
            DEFAULT_BACKEND_HOST.to_string(),
            DEFAULT_BACKEND_PORT,
            Some("test-secret".to_string()),
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            DEFAULT_REQUEST_TIMEOUT_SECS,
            DEFAULT_TOKEN_TTL_SECS,
            false,
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.listen_addr.port(), 8080);
        assert_eq!(config.backend_url, "http://127.0.0.1:9000");
        assert_eq!(config.secret, Some(b"test-secret".to_vec()));
        assert_eq!(config.state_header.as_str(), "otoroshi-state");
        assert_eq!(config.state_resp_header.as_str(), "otoroshi-state-resp");
        assert_eq!(config.request_timeout, Duration::from_secs(30));
        assert_eq!(config.token_ttl, 30);
        assert_eq!(config.version, ProtocolVersion::V2);
    }

    #[test]
    fn test_config_v1_mode() {
        let config = ProxyConfig::new(
            8080,
            "127.0.0.1".to_string(),
            9000,
            None,
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            30,
            true,
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.version, ProtocolVersion::V1);
        assert!(config.secret.is_none());
    }

    #[test]
    fn test_config_custom_values() {
        let config = ProxyConfig::new(
            3000,
            "localhost".to_string(),
            8000,
            Some("my-secret".to_string()),
            false,
            "X-Challenge".to_string(),
            "X-Challenge-Resp".to_string(),
            60,
            45,
            false,
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.listen_addr.port(), 3000);
        assert_eq!(config.backend_url, "http://localhost:8000");
        assert_eq!(config.request_timeout, Duration::from_secs(60));
        assert_eq!(config.token_ttl, 45);
    }

    #[test]
    fn test_config_base64_secret() {
        // "hello" in base64 is "aGVsbG8="
        let config = ProxyConfig::new(
            8080,
            "127.0.0.1".to_string(),
            9000,
            Some("aGVsbG8=".to_string()),
            true,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            30,
            false,
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.secret, Some(b"hello".to_vec()));
    }

    #[test]
    fn test_config_invalid_header() {
        let config = ProxyConfig::new(
            8080,
            "127.0.0.1".to_string(),
            9000,
            Some("secret".to_string()),
            false,
            "Invalid Header With Spaces".to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            30,
            false,
        );

        assert!(config.is_err());
    }

    #[test]
    fn test_config_invalid_base64_secret() {
        let config = ProxyConfig::new(
            8080,
            "127.0.0.1".to_string(),
            9000,
            Some("not-valid-base64!!!".to_string()),
            true, // secret_base64 = true
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            30,
            false,
        );

        assert!(config.is_err());
        assert!(matches!(
            config.unwrap_err(),
            ConfigError::InvalidBase64Secret(_)
        ));
    }

    #[test]
    fn test_config_invalid_ttl_zero() {
        let config = ProxyConfig::new(
            8080,
            "127.0.0.1".to_string(),
            9000,
            Some("secret".to_string()),
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            0, // TTL = 0
            false,
        );

        assert!(config.is_err());
        assert!(matches!(
            config.unwrap_err(),
            ConfigError::InvalidTokenTtl(0)
        ));
    }

    #[test]
    fn test_config_invalid_ttl_negative() {
        let config = ProxyConfig::new(
            8080,
            "127.0.0.1".to_string(),
            9000,
            Some("secret".to_string()),
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            -10, // TTL = -10
            false,
        );

        assert!(config.is_err());
        assert!(matches!(
            config.unwrap_err(),
            ConfigError::InvalidTokenTtl(-10)
        ));
    }

    #[test]
    fn test_config_invalid_backend_host_empty() {
        let config = ProxyConfig::new(
            8080,
            "".to_string(), // empty host
            9000,
            Some("secret".to_string()),
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            30,
            false,
        );

        assert!(config.is_err());
        assert!(matches!(
            config.unwrap_err(),
            ConfigError::InvalidBackendHost(_)
        ));
    }

    #[test]
    fn test_config_invalid_backend_host_whitespace() {
        let config = ProxyConfig::new(
            8080,
            "host with spaces".to_string(),
            9000,
            Some("secret".to_string()),
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            30,
            false,
        );

        assert!(config.is_err());
        assert!(matches!(
            config.unwrap_err(),
            ConfigError::InvalidBackendHost(_)
        ));
    }

    #[test]
    fn test_config_invalid_port_zero() {
        let config = ProxyConfig::new(
            0, // port = 0
            "127.0.0.1".to_string(),
            9000,
            Some("secret".to_string()),
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            30,
            false,
        );

        assert!(config.is_err());
        assert!(matches!(config.unwrap_err(), ConfigError::InvalidPort));
    }

    #[test]
    fn test_config_invalid_backend_port_zero() {
        let config = ProxyConfig::new(
            8080,
            "127.0.0.1".to_string(),
            0, // backend_port = 0
            Some("secret".to_string()),
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            30,
            false,
        );

        assert!(config.is_err());
        assert!(matches!(config.unwrap_err(), ConfigError::InvalidPort));
    }
}
