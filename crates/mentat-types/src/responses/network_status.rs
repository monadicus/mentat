//! The module defines the `NetworkStatusResponse` response.

use super::*;

/// [`NetworkStatusResponse`] contains basic information about the node's view
/// of a blockchain network. It is assumed that any [`BlockIdentifier.Index`]
/// less than or equal to CurrentBlockIdentifier. Index can be queried. If a
/// Rosetta implementation prunes historical state, it should populate the
/// optional `oldest_block_identifier` field with the oldest block available to
/// query. If this is not populated, it is assumed that the
/// `genesis_block_identifier` is the oldest queryable block. If a Rosetta
/// implementation performs some pre-sync before it is possible to query blocks,
/// `sync_status` should be populated so that clients can still monitor
/// healthiness. Without this field, it may appear that the implementation is
/// stuck syncing and needs to be terminated.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable, PartialEq, Eq)]
#[serde(default)]
pub struct NullableNetworkStatusResponse {
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_block_identifier: Option<NullableBlockIdentifier>,
    /// The timestamp of the block in milliseconds since the Unix Epoch. The
    /// timestamp is stored in milliseconds because some blockchains produce
    /// blocks more often than once a second.
    #[nullable(usize)]
    pub current_block_timestamp: isize,
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genesis_block_identifier: Option<NullableBlockIdentifier>,
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nullable(retain)]
    pub oldest_block_identifier: Option<NullableBlockIdentifier>,
    /// `SyncStatus` is used to provide additional context about an
    /// implementation's sync status. This object is often used by
    /// implementations to indicate healthiness when block data cannot be
    /// queried until some sync phase completes or cannot be determined by
    /// comparing the timestamp of the most recent block with the current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[nullable(retain)]
    pub sync_status: Option<NullableSyncStatus>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub peers: Vec<Option<Peer>>,
}
