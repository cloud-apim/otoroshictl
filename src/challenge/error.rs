//! Error types for the Otoroshi Challenge Proxy.

use thiserror::Error;

/// Errors that can occur during configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Header name is invalid.
    #[error("Invalid header name for {name}: {source}")]
    InvalidHeader {
        /// Name of the configuration field.
        name: &'static str,
        /// Underlying error.
        #[source]
        source: http::header::InvalidHeaderName,
    },

    /// Secret is not valid base64 when --secret-base64 is used.
    #[error("Invalid base64 encoding for secret: {0}")]
    InvalidBase64Secret(#[from] base64::DecodeError),

    /// Token TTL must be positive.
    #[error("Token TTL must be greater than 0, got {0}")]
    InvalidTokenTtl(i64),

    /// Backend host is invalid.
    #[error("Invalid backend host: {0}")]
    InvalidBackendHost(String),

    /// Port must be non-zero.
    #[error("Port must be greater than 0")]
    InvalidPort,

    /// Failed to read a key file.
    #[error("Failed to read key file '{path}': {source}")]
    KeyFileError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// Failed to extract the public key from the private key.
    #[error("Failed to extract public key: {0}")]
    PublicKeyExtraction(String),
}
