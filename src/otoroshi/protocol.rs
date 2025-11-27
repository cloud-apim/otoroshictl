//! Otoroshi challenge protocol implementation.
//!
//! Supports both V1 (simple echo) and V2 (JWT challenge/response) protocols
//! with configurable HMAC algorithms (HS256, HS384, HS512).

use std::str::FromStr;

use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Default header name for incoming challenge token.
pub const DEFAULT_STATE_HEADER: &str = "Otoroshi-State";

/// Default header name for outgoing response token.
pub const DEFAULT_STATE_RESP_HEADER: &str = "Otoroshi-State-Resp";

/// Expected issuer in Otoroshi challenge tokens.
pub const OTOROSHI_ISSUER: &str = "Otoroshi";

/// Default JWT token expiry time in seconds.
pub const DEFAULT_TOKEN_EXPIRY_SECONDS: i64 = 30;

/// Supported HMAC algorithms for Otoroshi protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Algorithm {
    HS256,
    HS384,
    #[default]
    HS512,
}

impl FromStr for Algorithm {
    type Err = std::convert::Infallible;

    /// Parse algorithm from string (e.g., "HS256", "HS384", "HS512").
    /// Defaults to HS512 for unknown values.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_uppercase().as_str() {
            "HS256" => Algorithm::HS256,
            "HS384" => Algorithm::HS384,
            _ => Algorithm::HS512,
        })
    }
}

impl Algorithm {
    fn as_jsonwebtoken(self) -> jsonwebtoken::Algorithm {
        match self {
            Algorithm::HS256 => jsonwebtoken::Algorithm::HS256,
            Algorithm::HS384 => jsonwebtoken::Algorithm::HS384,
            Algorithm::HS512 => jsonwebtoken::Algorithm::HS512,
        }
    }
}

/// Errors that can occur during protocol operations.
#[derive(Debug, Error)]
pub enum ProtocolError {
    /// JWT verification failed.
    #[error("JWT verification failed: {0}")]
    VerificationFailed(#[from] jsonwebtoken::errors::Error),

    /// JWT encoding failed.
    #[error("Failed to create response token: {0}")]
    EncodingFailed(jsonwebtoken::errors::Error),
}

/// Claims expected in the Otoroshi challenge JWT.
///
/// Only the `state` field is needed; other JWT claims are ignored.
#[derive(Debug, Deserialize)]
struct ChallengeClaims {
    /// The challenge state value to echo back in the response.
    state: String,
}

/// Claims for the response JWT sent back to Otoroshi.
#[derive(Debug, Serialize)]
struct ResponseClaims {
    /// The echoed state value from the challenge.
    #[serde(rename = "state-resp")]
    state_resp: String,
    /// Audience (required by Otoroshi).
    aud: String,
    /// Issued at timestamp.
    iat: i64,
    /// Not before timestamp.
    nbf: i64,
    /// Expiration timestamp.
    exp: i64,
}

/// Otoroshi protocol handler for V1/V2 challenge-response.
#[derive(Debug, Clone)]
pub struct OtoroshiProtocol {
    /// Algorithm for verifying incoming tokens.
    pub algo_in: Algorithm,
    /// Secret for verifying incoming tokens (raw bytes).
    pub secret_in: Vec<u8>,
    /// Algorithm for signing outgoing tokens.
    pub algo_out: Algorithm,
    /// Secret for signing outgoing tokens (raw bytes).
    pub secret_out: Vec<u8>,
    /// JWT token TTL in seconds.
    pub ttl: i64,
}

impl OtoroshiProtocol {
    /// Create a new protocol handler with the same secret and algorithm for both directions.
    pub fn new(secret: &[u8], algorithm: Algorithm) -> Self {
        Self {
            algo_in: algorithm,
            secret_in: secret.to_vec(),
            algo_out: algorithm,
            secret_out: secret.to_vec(),
            ttl: DEFAULT_TOKEN_EXPIRY_SECONDS,
        }
    }

    /// Create a new protocol handler with configurable TTL.
    pub fn new_with_ttl(secret: &[u8], algorithm: Algorithm, ttl: i64) -> Self {
        Self {
            algo_in: algorithm,
            secret_in: secret.to_vec(),
            algo_out: algorithm,
            secret_out: secret.to_vec(),
            ttl,
        }
    }

    /// Create a new protocol handler with separate secrets and algorithms for each direction.
    pub fn new_asymmetric(
        secret_in: &[u8],
        algo_in: Algorithm,
        secret_out: &[u8],
        algo_out: Algorithm,
    ) -> Self {
        Self {
            algo_in,
            secret_in: secret_in.to_vec(),
            algo_out,
            secret_out: secret_out.to_vec(),
            ttl: DEFAULT_TOKEN_EXPIRY_SECONDS,
        }
    }

    /// Process a V1 challenge (simple echo).
    ///
    /// Returns the same state value that was passed in.
    pub fn process_v1(&self, state: &str) -> String {
        state.to_string()
    }

    /// Process a V2 challenge (JWT verification and response generation).
    ///
    /// Verifies the incoming JWT token, extracts the state, and creates a response token.
    pub fn process_v2(&self, token: &str) -> Result<String, ProtocolError> {
        let state = self.verify_challenge(token)?;
        self.create_response_token(&state)
    }

    /// Verify an Otoroshi V2 challenge token.
    ///
    /// Validates the JWT signature and extracts the state claim.
    pub fn verify_challenge(&self, token: &str) -> Result<String, ProtocolError> {
        let mut validation = Validation::new(self.algo_in.as_jsonwebtoken());
        // Require expiration and issuer claims for security
        validation.set_required_spec_claims(&["exp", "iss"]);
        validation.set_issuer(&[OTOROSHI_ISSUER]);
        validation.validate_aud = false;
        // Be lenient with expiration for clock skew (matches Otoroshi's acceptLeeway default)
        validation.leeway = 10;

        let token_data = decode::<ChallengeClaims>(
            token,
            &DecodingKey::from_secret(&self.secret_in),
            &validation,
        )?;

        Ok(token_data.claims.state)
    }

    /// Create a response token for Otoroshi.
    ///
    /// Generates a JWT containing the echoed state value with appropriate claims.
    pub fn create_response_token(&self, state: &str) -> Result<String, ProtocolError> {
        let now = Utc::now().timestamp();

        let claims = ResponseClaims {
            state_resp: state.to_string(),
            aud: OTOROSHI_ISSUER.to_string(),
            iat: now,
            nbf: now,
            exp: now + self.ttl,
        };

        encode(
            &Header::new(self.algo_out.as_jsonwebtoken()),
            &claims,
            &EncodingKey::from_secret(&self.secret_out),
        )
        .map_err(ProtocolError::EncodingFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &[u8] = b"test-secret-key-for-testing";

    // -------------------------------------------------------------------------
    // Algorithm tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_algorithm_from_str_hs256() {
        assert_eq!(Algorithm::from_str("HS256").unwrap(), Algorithm::HS256);
        assert_eq!(Algorithm::from_str("hs256").unwrap(), Algorithm::HS256);
    }

    #[test]
    fn test_algorithm_from_str_hs384() {
        assert_eq!(Algorithm::from_str("HS384").unwrap(), Algorithm::HS384);
        assert_eq!(Algorithm::from_str("hs384").unwrap(), Algorithm::HS384);
    }

    #[test]
    fn test_algorithm_from_str_hs512() {
        assert_eq!(Algorithm::from_str("HS512").unwrap(), Algorithm::HS512);
        assert_eq!(Algorithm::from_str("hs512").unwrap(), Algorithm::HS512);
    }

    #[test]
    fn test_algorithm_from_str_defaults_to_hs512() {
        assert_eq!(Algorithm::from_str("unknown").unwrap(), Algorithm::HS512);
        assert_eq!(Algorithm::from_str("").unwrap(), Algorithm::HS512);
    }

    #[test]
    fn test_algorithm_default() {
        assert_eq!(Algorithm::default(), Algorithm::HS512);
    }

    // -------------------------------------------------------------------------
    // Protocol constructor tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_protocol_new() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS256);
        assert_eq!(protocol.algo_in, Algorithm::HS256);
        assert_eq!(protocol.algo_out, Algorithm::HS256);
        assert_eq!(protocol.secret_in, TEST_SECRET);
        assert_eq!(protocol.secret_out, TEST_SECRET);
        assert_eq!(protocol.ttl, DEFAULT_TOKEN_EXPIRY_SECONDS);
    }

    #[test]
    fn test_protocol_new_with_ttl() {
        let protocol = OtoroshiProtocol::new_with_ttl(TEST_SECRET, Algorithm::HS384, 60);
        assert_eq!(protocol.algo_in, Algorithm::HS384);
        assert_eq!(protocol.ttl, 60);
    }

    #[test]
    fn test_protocol_new_asymmetric() {
        let secret_in = b"secret-in";
        let secret_out = b"secret-out";
        let protocol = OtoroshiProtocol::new_asymmetric(
            secret_in,
            Algorithm::HS256,
            secret_out,
            Algorithm::HS512,
        );
        assert_eq!(protocol.algo_in, Algorithm::HS256);
        assert_eq!(protocol.algo_out, Algorithm::HS512);
        assert_eq!(protocol.secret_in, secret_in);
        assert_eq!(protocol.secret_out, secret_out);
    }

    // -------------------------------------------------------------------------
    // V1 protocol tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_process_v1_echoes_state() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);
        assert_eq!(protocol.process_v1("test-state"), "test-state");
        assert_eq!(protocol.process_v1(""), "");
        assert_eq!(
            protocol.process_v1("special-chars-!@#$%"),
            "special-chars-!@#$%"
        );
    }

    // -------------------------------------------------------------------------
    // Response token creation tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_create_response_token_success() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);
        let token = protocol.create_response_token("my-state").unwrap();

        // Token should be a valid JWT (three parts separated by dots)
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);
    }

    #[test]
    fn test_create_response_token_can_be_decoded() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);
        let token = protocol.create_response_token("my-state").unwrap();

        // Decode and verify the token contains expected claims
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS512);
        validation.set_required_spec_claims::<&str>(&[]);
        validation.set_audience(&[OTOROSHI_ISSUER]);

        #[derive(Debug, Deserialize)]
        struct TestClaims {
            #[serde(rename = "state-resp")]
            state_resp: String,
            aud: String,
            iat: i64,
            #[allow(dead_code)]
            nbf: i64,
            exp: i64,
        }

        let decoded =
            decode::<TestClaims>(&token, &DecodingKey::from_secret(TEST_SECRET), &validation)
                .unwrap();

        assert_eq!(decoded.claims.state_resp, "my-state");
        assert_eq!(decoded.claims.aud, OTOROSHI_ISSUER);
        assert!(decoded.claims.exp > decoded.claims.iat);
    }

    #[test]
    fn test_create_response_token_respects_ttl() {
        let protocol = OtoroshiProtocol::new_with_ttl(TEST_SECRET, Algorithm::HS512, 120);
        let token = protocol.create_response_token("state").unwrap();

        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS512);
        validation.set_required_spec_claims::<&str>(&[]);
        validation.set_audience(&[OTOROSHI_ISSUER]);

        #[derive(Debug, Deserialize)]
        struct TestClaims {
            iat: i64,
            exp: i64,
        }

        let decoded =
            decode::<TestClaims>(&token, &DecodingKey::from_secret(TEST_SECRET), &validation)
                .unwrap();

        assert_eq!(decoded.claims.exp - decoded.claims.iat, 120);
    }

    // -------------------------------------------------------------------------
    // Challenge verification tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_verify_challenge_valid_token() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);

        // Create a valid challenge token
        #[derive(Serialize)]
        struct Challenge {
            state: String,
            iss: String,
            iat: i64,
            exp: i64,
        }

        let now = Utc::now().timestamp();
        let claims = Challenge {
            state: "challenge-state".to_string(),
            iss: OTOROSHI_ISSUER.to_string(),
            iat: now,
            exp: now + 60,
        };

        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(TEST_SECRET),
        )
        .unwrap();

        let state = protocol.verify_challenge(&token).unwrap();
        assert_eq!(state, "challenge-state");
    }

    #[test]
    fn test_verify_challenge_invalid_signature() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);

        // Create a token with a different secret
        #[derive(Serialize)]
        struct Challenge {
            state: String,
            iss: String,
            exp: i64,
        }

        let claims = Challenge {
            state: "state".to_string(),
            iss: OTOROSHI_ISSUER.to_string(),
            exp: Utc::now().timestamp() + 60,
        };

        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(b"wrong-secret"),
        )
        .unwrap();

        let result = protocol.verify_challenge(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_challenge_malformed_token() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);
        let result = protocol.verify_challenge("not-a-valid-jwt");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_challenge_missing_issuer() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);

        // Token without iss claim should be rejected
        #[derive(Serialize)]
        struct Challenge {
            state: String,
            exp: i64,
        }

        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &Challenge {
                state: "test".to_string(),
                exp: Utc::now().timestamp() + 60,
            },
            &EncodingKey::from_secret(TEST_SECRET),
        )
        .unwrap();

        let result = protocol.verify_challenge(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_challenge_wrong_issuer() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);

        // Token with wrong iss claim should be rejected
        #[derive(Serialize)]
        struct Challenge {
            state: String,
            iss: String,
            exp: i64,
        }

        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &Challenge {
                state: "test".to_string(),
                iss: "NotOtoroshi".to_string(),
                exp: Utc::now().timestamp() + 60,
            },
            &EncodingKey::from_secret(TEST_SECRET),
        )
        .unwrap();

        let result = protocol.verify_challenge(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_challenge_missing_expiration() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);

        // Token without exp claim should be rejected
        #[derive(Serialize)]
        struct Challenge {
            state: String,
            iss: String,
        }

        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &Challenge {
                state: "test".to_string(),
                iss: OTOROSHI_ISSUER.to_string(),
            },
            &EncodingKey::from_secret(TEST_SECRET),
        )
        .unwrap();

        let result = protocol.verify_challenge(&token);
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Full V2 flow tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_process_v2_full_flow() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);

        // Create a challenge token
        #[derive(Serialize)]
        struct Challenge {
            state: String,
            iss: String,
            iat: i64,
            exp: i64,
        }

        let now = Utc::now().timestamp();
        let challenge_token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &Challenge {
                state: "roundtrip-state".to_string(),
                iss: OTOROSHI_ISSUER.to_string(),
                iat: now,
                exp: now + 60,
            },
            &EncodingKey::from_secret(TEST_SECRET),
        )
        .unwrap();

        // Process the challenge
        let response_token = protocol.process_v2(&challenge_token).unwrap();

        // Verify the response contains the correct state
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS512);
        validation.set_required_spec_claims::<&str>(&[]);
        validation.set_audience(&[OTOROSHI_ISSUER]);

        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "state-resp")]
            state_resp: String,
        }

        let decoded = decode::<Response>(
            &response_token,
            &DecodingKey::from_secret(TEST_SECRET),
            &validation,
        )
        .unwrap();

        assert_eq!(decoded.claims.state_resp, "roundtrip-state");
    }

    // -------------------------------------------------------------------------
    // Different algorithm tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_create_and_verify_with_hs256() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS256);
        let token = protocol.create_response_token("state").unwrap();

        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_required_spec_claims::<&str>(&[]);
        validation.set_audience(&[OTOROSHI_ISSUER]);

        let result = decode::<serde_json::Value>(
            &token,
            &DecodingKey::from_secret(TEST_SECRET),
            &validation,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_and_verify_with_hs384() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS384);
        let token = protocol.create_response_token("state").unwrap();

        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS384);
        validation.set_required_spec_claims::<&str>(&[]);
        validation.set_audience(&[OTOROSHI_ISSUER]);

        let result = decode::<serde_json::Value>(
            &token,
            &DecodingKey::from_secret(TEST_SECRET),
            &validation,
        );
        assert!(result.is_ok());
    }

    // -------------------------------------------------------------------------
    // Expiration tests (leeway = 10s, matches Otoroshi default)
    // -------------------------------------------------------------------------

    #[test]
    fn test_verify_challenge_expired_within_leeway() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);

        // Create a token that expired 5 seconds ago (within 10s leeway)
        #[derive(Serialize)]
        struct Challenge {
            state: String,
            iss: String,
            exp: i64,
        }

        let now = Utc::now().timestamp();
        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &Challenge {
                state: "leeway-test".to_string(),
                iss: OTOROSHI_ISSUER.to_string(),
                exp: now - 5, // Expired 5s ago
            },
            &EncodingKey::from_secret(TEST_SECRET),
        )
        .unwrap();

        // Should succeed due to 10s leeway
        let result = protocol.verify_challenge(&token);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "leeway-test");
    }

    #[test]
    fn test_verify_challenge_expired_beyond_leeway() {
        let protocol = OtoroshiProtocol::new(TEST_SECRET, Algorithm::HS512);

        // Create a token that expired 15 seconds ago (beyond 10s leeway)
        #[derive(Serialize)]
        struct Challenge {
            state: String,
            iss: String,
            exp: i64,
        }

        let now = Utc::now().timestamp();
        let token = encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &Challenge {
                state: "expired-test".to_string(),
                iss: OTOROSHI_ISSUER.to_string(),
                exp: now - 15, // Expired 15s ago
            },
            &EncodingKey::from_secret(TEST_SECRET),
        )
        .unwrap();

        // Should fail - beyond leeway
        let result = protocol.verify_challenge(&token);
        assert!(result.is_err());
    }
}
