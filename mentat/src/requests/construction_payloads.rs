//! The module defines the ConstructionPayloadsRequest model.

use indexmap::IndexMap;

use super::*;

/// `ConstructionPayloadsRequest` is the request to `/construction/payloads`. It
/// contains the network, a slice of operations, and arbitrary metadata that was
/// returned by the call to `/construction/metadata`. Optionally, the request
/// can also include an array of [`PublicKey`]s associated with the
/// [`AccountIdentifiers`] returned in [`ConstructionPreprocessResponse`].
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConstructionPayloadsRequest {
    /// The `network_identifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// Undocumented in rosetta
    pub operations: Vec<Operation>,
    /// Additional metadata related to the `/construction/payloads` request.
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
    /// Undocumented in rosetta
    pub public_keys: Option<Vec<PublicKey>>,
}
