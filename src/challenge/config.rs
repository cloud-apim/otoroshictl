//! Configuration for the Otoroshi Challenge Proxy.

use base64::Engine;
use http::Method;
use http::header::HeaderName;
use std::net::SocketAddr;
use std::path::Path;
use std::time::Duration;

use crate::challenge::error::ConfigError;
use crate::otoroshi::protocol::{Algorithm, ConsumerInfoVerifier};

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

/// Clever Cloud environment variable holding a single health check path.
pub const CC_HEALTH_CHECK_PATH_ENV: &str = "CC_HEALTH_CHECK_PATH";

/// Prefix of the Clever Cloud environment variables holding indexed health check paths
/// (`CC_HEALTH_CHECK_PATH_0`, `CC_HEALTH_CHECK_PATH_1`, ...).
pub const CC_HEALTH_CHECK_PATH_INDEXED_ENV_PREFIX: &str = "CC_HEALTH_CHECK_PATH_";

/// Normalize a request path for exclusion matching.
///
/// Trims whitespace, drops any query string, ensures a leading `/` and removes a trailing `/`
/// (except for the root path). Returns `None` for empty values.
pub fn normalize_path(path: &str) -> Option<String> {
    let trimmed = path.trim();
    let without_query = trimmed.split(['?', '#']).next().unwrap_or("");
    if without_query.is_empty() {
        return None;
    }
    let mut normalized = if without_query.starts_with('/') {
        without_query.to_string()
    } else {
        format!("/{}", without_query)
    };
    while normalized.len() > 1 && normalized.ends_with('/') {
        normalized.pop();
    }
    Some(normalized)
}

/// Read the health check paths declared through Clever Cloud environment variables.
///
/// Supports both `CC_HEALTH_CHECK_PATH` (single path) and the indexed form
/// `CC_HEALTH_CHECK_PATH_0`, `CC_HEALTH_CHECK_PATH_1`, ... (stops at the first gap).
/// See <https://www.clever.cloud/developers/doc/deploy/applications/dotnet/#enable-health-check-during-deployment>.
pub fn clever_cloud_health_check_paths() -> Vec<String> {
    clever_cloud_health_check_paths_from(|name| std::env::var(name).ok())
}

/// Same as [`clever_cloud_health_check_paths`] but with an injectable environment lookup.
pub fn clever_cloud_health_check_paths_from<F>(lookup: F) -> Vec<String>
where
    F: Fn(&str) -> Option<String>,
{
    let mut paths = Vec::new();
    if let Some(value) = lookup(CC_HEALTH_CHECK_PATH_ENV)
        && !value.trim().is_empty()
    {
        paths.push(value);
    }
    let mut index = 0usize;
    while let Some(value) = lookup(&format!(
        "{}{}",
        CC_HEALTH_CHECK_PATH_INDEXED_ENV_PREFIX, index
    )) {
        if !value.trim().is_empty() {
            paths.push(value);
        }
        index += 1;
    }
    paths
}

/// Protocol version for Otoroshi challenge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    /// V1: Simple echo of the state header value.
    V1,
    /// V2: JWT-based challenge/response with HMAC-SHA512.
    V2,
}

/// Configuration for decoding Otoroshi Consumer Info JWTs.
#[derive(Debug, Clone)]
pub struct ConsumerInfoConfig {
    /// Header name to read the Consumer Info JWT from.
    pub in_header: HeaderName,
    /// Header name to write the decoded JSON into.
    pub out_header: HeaderName,
    /// JWT verifier for the Consumer Info token.
    pub verifier: ConsumerInfoVerifier,
    /// If true, reject requests where the Consumer Info header is absent or invalid.
    pub strict: bool,
}

/// Proxy configuration built from CLI arguments.
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// Socket address to listen on.
    pub listen_addr: SocketAddr,
    /// Full URL to the backend server.
    pub backend_url: String,
    /// Secret or public key bytes for JWT verification (required for V2).
    pub secret: Option<Vec<u8>>,
    /// Public key PEM bytes for JWT verification (only for asymmetric algorithms).
    pub public_key: Option<Vec<u8>>,
    /// Algorithm for JWT verification.
    pub algorithm: Algorithm,
    /// Secret or private key bytes for signing the response JWT.
    /// If None, uses the same as `secret`.
    pub response_secret: Option<Vec<u8>>,
    /// Algorithm for signing the response JWT.
    pub response_algorithm: Algorithm,
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
    /// Optional Consumer Info JWT processing configuration.
    pub consumer_info: Option<ConsumerInfoConfig>,
    /// Strip Otoroshi-specific headers (state challenge + consumer info) before forwarding.
    pub strip_otoroshi_headers: bool,
    /// Request paths forwarded to the backend without challenge verification when the request
    /// carries no state header (health checks). Only `GET` and `HEAD` requests are concerned.
    /// Paths are normalized with [`normalize_path`] and matched exactly.
    pub excluded_paths: Vec<String>,
}

/// Read a PEM value: if it points to an existing file, read the file; otherwise use as-is.
fn resolve_pem(value: &str) -> Result<Vec<u8>, ConfigError> {
    let path = Path::new(value);
    if path.is_file() {
        std::fs::read(path).map_err(|e| ConfigError::KeyFileError {
            path: value.to_string(),
            source: e,
        })
    } else {
        Ok(value.as_bytes().to_vec())
    }
}

/// Returns true if the PEM bytes represent a private key (PKCS#1 or PKCS#8).
fn is_private_key_pem(pem: &[u8]) -> bool {
    let s = std::str::from_utf8(pem).unwrap_or("");
    s.contains("PRIVATE KEY")
}

/// Extract the public key PEM from a private key PEM for RSA algorithms.
fn extract_rsa_public_key(private_pem: &[u8]) -> Result<Vec<u8>, ConfigError> {
    use rsa::pkcs8::DecodePrivateKey;
    use rsa::pkcs8::EncodePublicKey;

    let pem_str = std::str::from_utf8(private_pem)
        .map_err(|e| ConfigError::PublicKeyExtraction(e.to_string()))?;
    let private_key = rsa::RsaPrivateKey::from_pkcs8_pem(pem_str)
        .map_err(|e| ConfigError::PublicKeyExtraction(format!("RSA private key: {}", e)))?;
    let public_key = private_key.to_public_key();
    let public_pem = public_key
        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| ConfigError::PublicKeyExtraction(format!("RSA public key PEM: {}", e)))?;
    Ok(public_pem.into_bytes())
}

/// Extract the public key PEM from a private key PEM for ES256 (P-256).
fn extract_ec_p256_public_key(private_pem: &[u8]) -> Result<Vec<u8>, ConfigError> {
    use p256::pkcs8::DecodePrivateKey;
    use p256::pkcs8::EncodePublicKey;

    let pem_str = std::str::from_utf8(private_pem)
        .map_err(|e| ConfigError::PublicKeyExtraction(e.to_string()))?;
    let secret_key = p256::SecretKey::from_pkcs8_pem(pem_str)
        .map_err(|e| ConfigError::PublicKeyExtraction(format!("EC P-256 private key: {}", e)))?;
    let public_key = secret_key.public_key();
    let public_pem = public_key
        .to_public_key_pem(p256::pkcs8::LineEnding::LF)
        .map_err(|e| ConfigError::PublicKeyExtraction(format!("EC P-256 public key PEM: {}", e)))?;
    Ok(public_pem.into_bytes())
}

/// Extract the public key PEM from a private key PEM for ES384 (P-384).
fn extract_ec_p384_public_key(private_pem: &[u8]) -> Result<Vec<u8>, ConfigError> {
    use p384::pkcs8::DecodePrivateKey;
    use p384::pkcs8::EncodePublicKey;

    let pem_str = std::str::from_utf8(private_pem)
        .map_err(|e| ConfigError::PublicKeyExtraction(e.to_string()))?;
    let secret_key = p384::SecretKey::from_pkcs8_pem(pem_str)
        .map_err(|e| ConfigError::PublicKeyExtraction(format!("EC P-384 private key: {}", e)))?;
    let public_key = secret_key.public_key();
    let public_pem = public_key
        .to_public_key_pem(p384::pkcs8::LineEnding::LF)
        .map_err(|e| ConfigError::PublicKeyExtraction(format!("EC P-384 public key PEM: {}", e)))?;
    Ok(public_pem.into_bytes())
}

/// Extract the public key from a private key PEM based on the algorithm.
fn extract_public_key(algorithm: Algorithm, private_pem: &[u8]) -> Result<Vec<u8>, ConfigError> {
    match algorithm {
        Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
            extract_rsa_public_key(private_pem)
        }
        Algorithm::ES256 => extract_ec_p256_public_key(private_pem),
        Algorithm::ES384 => extract_ec_p384_public_key(private_pem),
        _ => unreachable!("extract_public_key called for symmetric algorithm"),
    }
}

/// Resolve a secret value: decode from base64 if requested, otherwise use as UTF-8 bytes.
fn resolve_secret(secret: &str, is_base64: bool) -> Result<Vec<u8>, ConfigError> {
    if is_base64 {
        base64::engine::general_purpose::STANDARD
            .decode(secret)
            .map_err(ConfigError::InvalidBase64Secret)
    } else {
        Ok(secret.as_bytes().to_vec())
    }
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
        alg: String,
        public_key: Option<String>,
        response_secret: Option<String>,
        response_secret_base64: bool,
        response_alg: Option<String>,
        use_v1: bool,
        consumer_info_enabled: bool,
        consumer_info_header: String,
        consumer_info_out_header: Option<String>,
        consumer_info_alg: String,
        consumer_info_secret: Option<String>,
        consumer_info_secret_base64: bool,
        consumer_info_public_key: Option<String>,
        consumer_info_strict: bool,
        strip_otoroshi_headers: bool,
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

        let algorithm: Algorithm = alg.parse().unwrap_or_default();
        let response_algorithm: Algorithm = response_alg
            .as_deref()
            .map(|s| s.parse().unwrap_or_default())
            .unwrap_or(algorithm);

        // Build secret and public_key bytes based on algorithm type
        let (secret_bytes, public_key_bytes) = if algorithm.is_asymmetric() {
            // For asymmetric verification: secret is PEM private key (to extract public key)
            // or public_key is provided directly
            let private_pem = match &secret {
                Some(s) => Some(resolve_pem(s)?),
                None => None,
            };
            // Public key: provided or extracted from private key
            let pub_pem = match (&public_key, &private_pem) {
                (Some(pk), _) => Some(resolve_pem(pk)?),
                (None, Some(priv_pem)) => Some(extract_public_key(algorithm, priv_pem)?),
                (None, None) => {
                    return Err(ConfigError::PublicKeyExtraction(
                        "public key (--public-key) or private key (--secret) is required for asymmetric algorithms".to_string(),
                    ));
                }
            };
            (private_pem, pub_pem)
        } else {
            // For HMAC: decode secret from base64 if requested, otherwise use as UTF-8 bytes
            let secret_bytes = match &secret {
                Some(s) => Some(resolve_secret(s, secret_base64)?),
                None => None,
            };
            (secret_bytes, None)
        };

        // Build response secret based on response algorithm type
        let response_secret_bytes = match &response_secret {
            Some(rs) => {
                if response_algorithm.is_asymmetric() {
                    // For asymmetric signing: response_secret is the private key PEM
                    Some(resolve_pem(rs)?)
                } else {
                    // For HMAC signing
                    Some(resolve_secret(rs, response_secret_base64)?)
                }
            }
            None => None, // Will use secret_bytes as fallback in server.rs
        };

        // Build the consumer info configuration if enabled
        let consumer_info =
            if consumer_info_enabled {
                let ci_algorithm: Algorithm = consumer_info_alg.parse().unwrap_or_default();

                let ci_secret_bytes = if ci_algorithm.is_asymmetric() {
                    match (&consumer_info_public_key, &consumer_info_secret) {
                        (Some(pk), _) => resolve_pem(pk)?,
                        (None, Some(s)) => {
                            let pem = resolve_pem(s)?;
                            // Accept both a public key PEM and a private key PEM:
                            // if private, extract the public key; otherwise use as-is.
                            if is_private_key_pem(&pem) {
                                extract_public_key(ci_algorithm, &pem)?
                            } else {
                                pem
                            }
                        }
                        (None, None) => return Err(ConfigError::MissingConsumerInfoKey),
                    }
                } else {
                    match &consumer_info_secret {
                        Some(s) => resolve_secret(s, consumer_info_secret_base64)?,
                        None => return Err(ConfigError::MissingConsumerInfoKey),
                    }
                };

                let ci_in_header = HeaderName::from_bytes(consumer_info_header.as_bytes())
                    .map_err(|e| ConfigError::InvalidHeader {
                        name: "consumer_info_header",
                        source: e,
                    })?;

                let ci_out_header_str = consumer_info_out_header.unwrap_or(consumer_info_header);
                let ci_out_header =
                    HeaderName::from_bytes(ci_out_header_str.as_bytes()).map_err(|e| {
                        ConfigError::InvalidHeader {
                            name: "consumer_info_out_header",
                            source: e,
                        }
                    })?;

                Some(ConsumerInfoConfig {
                    in_header: ci_in_header,
                    out_header: ci_out_header,
                    verifier: ConsumerInfoVerifier::new(ci_algorithm, &ci_secret_bytes),
                    strict: consumer_info_strict,
                })
            } else {
                None
            };

        Ok(ProxyConfig {
            listen_addr: SocketAddr::from(([0, 0, 0, 0], port)),
            backend_url: format!("http://{}:{}", backend_host, backend_port),
            secret: secret_bytes,
            public_key: public_key_bytes,
            algorithm,
            response_secret: response_secret_bytes,
            response_algorithm,
            state_header,
            state_resp_header,
            request_timeout: Duration::from_secs(timeout_secs),
            token_ttl,
            version,
            consumer_info,
            strip_otoroshi_headers,
            excluded_paths: Vec::new(),
        })
    }

    /// Set the paths that bypass the challenge verification (health checks).
    ///
    /// Values are normalized with [`normalize_path`], deduplicated and empty values dropped.
    pub fn with_excluded_paths<I, S>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut excluded: Vec<String> = Vec::new();
        for path in paths {
            if let Some(normalized) = normalize_path(path.as_ref())
                && !excluded.contains(&normalized)
            {
                excluded.push(normalized);
            }
        }
        self.excluded_paths = excluded;
        self
    }

    /// Returns true if the request targets an excluded path (health check).
    ///
    /// Only `GET` and `HEAD` requests on an excluded path match. The caller decides whether
    /// to bypass the challenge (the server only does so when no state header is present).
    pub fn is_excluded(&self, method: &Method, path: &str) -> bool {
        if self.excluded_paths.is_empty() {
            return false;
        }
        if method != Method::GET && method != Method::HEAD {
            return false;
        }
        match normalize_path(path) {
            Some(normalized) => self.excluded_paths.contains(&normalized),
            None => false,
        }
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
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
        assert_eq!(config.algorithm, Algorithm::HS512);
        assert_eq!(config.response_algorithm, Algorithm::HS512);
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            true,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
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
            "HS256".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.listen_addr.port(), 3000);
        assert_eq!(config.backend_url, "http://localhost:8000");
        assert_eq!(config.request_timeout, Duration::from_secs(60));
        assert_eq!(config.token_ttl, 45);
        assert_eq!(config.algorithm, Algorithm::HS256);
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.secret, Some(b"hello".to_vec()));
    }

    #[test]
    fn test_config_with_separate_response_secret() {
        let config = ProxyConfig::new(
            8080,
            "127.0.0.1".to_string(),
            9000,
            Some("verify-secret".to_string()),
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            30,
            30,
            "HS512".to_string(),
            None,
            Some("sign-secret".to_string()),
            false,
            Some("HS256".to_string()),
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.secret, Some(b"verify-secret".to_vec()));
        assert_eq!(config.response_secret, Some(b"sign-secret".to_vec()));
        assert_eq!(config.algorithm, Algorithm::HS512);
        assert_eq!(config.response_algorithm, Algorithm::HS256);
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
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
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
        );

        assert!(config.is_err());
        assert!(matches!(config.unwrap_err(), ConfigError::InvalidPort));
    }
    fn base_config() -> ProxyConfig {
        ProxyConfig::new(
            DEFAULT_LISTEN_PORT,
            DEFAULT_BACKEND_HOST.to_string(),
            DEFAULT_BACKEND_PORT,
            Some("test-secret".to_string()),
            false,
            DEFAULT_STATE_HEADER.to_string(),
            DEFAULT_STATE_RESP_HEADER.to_string(),
            DEFAULT_REQUEST_TIMEOUT_SECS,
            DEFAULT_TOKEN_TTL_SECS,
            "HS512".to_string(),
            None,
            None,
            false,
            None,
            false,
            false,
            "Otoroshi-Claims".to_string(),
            None,
            "HS512".to_string(),
            None,
            false,
            None,
            false,
            true,
        )
        .unwrap()
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("/health"), Some("/health".to_string()));
        assert_eq!(normalize_path("health"), Some("/health".to_string()));
        assert_eq!(normalize_path("  /health  "), Some("/health".to_string()));
        assert_eq!(normalize_path("/health/"), Some("/health".to_string()));
        assert_eq!(normalize_path("/health?x=1"), Some("/health".to_string()));
        assert_eq!(normalize_path("/"), Some("/".to_string()));
        assert_eq!(normalize_path("///"), Some("/".to_string()));
        assert_eq!(normalize_path(""), None);
        assert_eq!(normalize_path("   "), None);
        assert_eq!(normalize_path("?only=query"), None);
    }

    #[test]
    fn test_excluded_paths_default_empty() {
        let config = base_config();
        assert!(config.excluded_paths.is_empty());
        assert!(!config.is_excluded(&Method::GET, "/"));
        assert!(!config.is_excluded(&Method::GET, "/health"));
    }

    #[test]
    fn test_with_excluded_paths_normalizes_and_dedups() {
        let config = base_config().with_excluded_paths([
            "/health",
            "health/",
            "  /ready  ",
            "",
            "/ready?probe=1",
        ]);
        assert_eq!(config.excluded_paths, vec!["/health", "/ready"]);
    }

    #[test]
    fn test_is_excluded_matches_get_and_head_only() {
        let config = base_config().with_excluded_paths(["/health"]);
        assert!(config.is_excluded(&Method::GET, "/health"));
        assert!(config.is_excluded(&Method::HEAD, "/health"));
        assert!(config.is_excluded(&Method::GET, "/health/"));
        assert!(config.is_excluded(&Method::GET, "/health?deep=true"));
        assert!(!config.is_excluded(&Method::POST, "/health"));
        assert!(!config.is_excluded(&Method::PUT, "/health"));
        assert!(!config.is_excluded(&Method::DELETE, "/health"));
    }

    #[test]
    fn test_is_excluded_exact_match_only() {
        let config = base_config().with_excluded_paths(["/health"]);
        assert!(!config.is_excluded(&Method::GET, "/healthz"));
        assert!(!config.is_excluded(&Method::GET, "/health/live"));
        assert!(!config.is_excluded(&Method::GET, "/api/health"));
        assert!(!config.is_excluded(&Method::GET, "/"));
    }

    fn env_from<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name| {
            pairs
                .iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| v.to_string())
        }
    }

    #[test]
    fn test_clever_cloud_paths_none() {
        let paths = clever_cloud_health_check_paths_from(env_from(&[]));
        assert!(paths.is_empty());
    }

    #[test]
    fn test_clever_cloud_paths_single() {
        let paths =
            clever_cloud_health_check_paths_from(env_from(&[("CC_HEALTH_CHECK_PATH", "/health")]));
        assert_eq!(paths, vec!["/health"]);
    }

    #[test]
    fn test_clever_cloud_paths_indexed() {
        let paths = clever_cloud_health_check_paths_from(env_from(&[
            ("CC_HEALTH_CHECK_PATH_0", "/health"),
            ("CC_HEALTH_CHECK_PATH_1", "/ready"),
            // gap at 2: index 3 must be ignored
            ("CC_HEALTH_CHECK_PATH_3", "/ignored"),
        ]));
        assert_eq!(paths, vec!["/health", "/ready"]);
    }

    #[test]
    fn test_clever_cloud_paths_single_and_indexed() {
        let paths = clever_cloud_health_check_paths_from(env_from(&[
            ("CC_HEALTH_CHECK_PATH", "/health"),
            ("CC_HEALTH_CHECK_PATH_0", "/ready"),
        ]));
        assert_eq!(paths, vec!["/health", "/ready"]);
    }

    #[test]
    fn test_clever_cloud_paths_skip_blank_values() {
        let paths = clever_cloud_health_check_paths_from(env_from(&[
            ("CC_HEALTH_CHECK_PATH", "   "),
            ("CC_HEALTH_CHECK_PATH_0", ""),
            ("CC_HEALTH_CHECK_PATH_1", "/ready"),
        ]));
        assert_eq!(paths, vec!["/ready"]);
    }
}
