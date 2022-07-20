//! The module defines the `ConstructionParseRequest` request.

use super::*;

/// `ConstructionParseRequest` is the input to the `/construction/parse`
/// endpoint. It allows the caller to parse either an unsigned or signed
/// transaction.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ConstructionParseRequest {
    /// The `NetworkIdentifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// Signed is a boolean indicating whether the transaction is signed.
    pub signed: bool,
    /// This must be either the unsigned transaction blob returned by
    /// `/construction/payloads` or the signed transaction blob returned by
    /// `/construction/combine`.
    pub transaction: String,
}
