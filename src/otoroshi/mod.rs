//! Shared Otoroshi protocol handling.
//!
//! This module provides common functionality for the Otoroshi challenge protocol
//! used by both the standalone proxy command and the sidecar.

pub mod protocol;

pub use protocol::{Algorithm, DEFAULT_STATE_HEADER, DEFAULT_STATE_RESP_HEADER, OtoroshiProtocol};
