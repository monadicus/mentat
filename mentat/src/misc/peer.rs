//! The module defines the Peer.

use indexmap::IndexMap;

use super::*;

/// A Peer is a representation of a node's peer.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Peer {
    /// The id of the peer.
    pub peer_id: String,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
