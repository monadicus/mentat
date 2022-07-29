//! The module defines the `BlockRequest` request.

use mentat_macros::Nullable;

use super::*;

/// A [`BlockRequest`] is utilized to make a block request on the `/block`
/// endpoint.
#[derive(Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableBlockRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// When fetching data by [`BlockIdentifier`], it may be possible to only
    /// specify the index or hash. If neither property is specified, it is
    /// assumed that the client is making a request at the current block.
    /// This is represented via a [`PartialBlockIdentifier`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<PartialBlockIdentifier>,
}

impl From<(NetworkIdentifier, PartialBlockIdentifier)> for NullableBlockRequest {
    fn from(
        (network_identifier, block_identifier): (NetworkIdentifier, PartialBlockIdentifier),
    ) -> Self {
        Self {
            network_identifier: Some(network_identifier),
            block_identifier: Some(block_identifier),
        }
    }
}

impl From<(PartialBlockIdentifier, NetworkIdentifier)> for NullableBlockRequest {
    fn from(
        (block_identifier, network_identifier): (PartialBlockIdentifier, NetworkIdentifier),
    ) -> Self {
        Self {
            network_identifier: Some(network_identifier),
            block_identifier: Some(block_identifier),
        }
    }
}
