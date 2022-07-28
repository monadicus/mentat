//! The module defines the `BlockTransactionResponse` response.

use super::*;

/// [`CallResponse`] contains the result of a `/call` invocation.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct CallResponse {
    /// Result contains the result of the `/call` invocation. This result will
    /// not be inspected or interpreted by Rosetta tooling and is left to
    /// the caller to decode.
    pub result: Value,
    /// Idempotent indicates that if `/call` is invoked with the same
    /// [`crate::requests::CallRequest`] again, at any point in time, it will
    /// return the same `CallResponse`. Integrators may cache the
    /// `CallResponse` if this is set to true to avoid making unnecessary
    /// calls to the Rosetta implementation. For this reason, implementers
    /// should be very conservative about returning true here or they could
    /// cause issues for the caller.
    pub idempotent: bool,
}
