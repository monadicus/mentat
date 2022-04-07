//! The module defines the ConstructionPayloadsRequest request.

use indexmap::IndexMap;

use super::*;

/// `ConstructionPayloadsRequest` is the request to `/construction/payloads`. It
/// contains the network, a slice of operations, and arbitrary metadata that was
/// returned by the call to `/construction/metadata`. Optionally, the request
/// can also include an array of [`PublicKey`]s associated with the
/// [`AccountIdentifier`]s returned in [`ConstructionPreprocessResponse`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionPayloadsRequest {
    /// The NetworkIdentifier specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    #[allow(clippy::missing_docs_in_private_items)]
    pub operations: Vec<Operation>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub public_keys: Option<Vec<PublicKey>>,
}
