use indexmap::IndexMap;

use super::*;

/// A Peer is a representation of a node's peer.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Peer {
    pub peer_id: String,
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
