use indexmap::IndexMap;

use super::*;

/// A Peer is a representation of a node's peer.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Peer {
    pub peer_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
