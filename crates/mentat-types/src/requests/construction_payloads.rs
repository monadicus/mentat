//! The module defines the ConstructionPayloadsRequest request.

use super::*;

/// [`ConstructionPayloadsRequest`] is the request to `/construction/payloads`.
/// It contains the network, a slice of operations, and arbitrary metadata that
/// was returned by the call to `/construction/metadata`. Optionally, the
/// request can also include an array of [`PublicKey`]s associated with the
/// [`AccountIdentifier`]s returned in
/// [`crate::responses::ConstructionPreprocessResponse`].
#[derive(Clone, Debug, Deserialize, Serialize, Default, Unchecked)]
#[serde(default)]
pub struct UncheckedConstructionPayloadsRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub operations: Vec<Option<UncheckedOperation>>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    #[unchecked(retain)]
    pub metadata: Metadata,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub public_keys: Vec<Option<UncheckedPublicKey>>,
}
