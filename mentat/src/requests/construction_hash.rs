//! The module defines the ConstructionHashRequest model.

use super::*;

/// `ConstructionHashRequest` is the input to the `/construction/hash` endpoint.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConstructionHashRequest {
    /// The `network_identifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// Undocumented in rosetta
    pub signed_transaction: String,
}
