//! The module defines the Peer.

use super::*;

/// A [`Peer`] is a representation of a node's peer.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct Peer {
    /// The id of the peer.
    pub peer_id: String,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: Metadata,
}
