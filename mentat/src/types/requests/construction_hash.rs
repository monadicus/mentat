//! The module defines the `ConstructionHashRequest` request.

use super::*;

/// `ConstructionHashRequest` is the input to the `/construction/hash` endpoint.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ConstructionHashRequest {
    /// The `NetworkIdentifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    #[allow(clippy::missing_docs_in_private_items)]
    pub signed_transaction: String,
}
