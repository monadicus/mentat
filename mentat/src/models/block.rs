use indexmap::IndexMap;

use super::*;

/// Blocks contain an array of Transactions that occurred at a particular
/// BlockIdentifier. A hard requirement for blocks returned by Rosetta
/// implementations is that they MUST be inalterable: once a client has
/// requested and received a block identified by a specific BlockIndentifier,
/// all future calls for that same BlockIdentifier must return the same block
/// contents.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Block {
    /// The block_identifier uniquely identifies a block in a particular
    /// network.
    pub block_identifier: BlockIdentifier,
    /// The block_identifier uniquely identifies a block in a particular
    /// network.
    pub parent_block_identifier: BlockIdentifier,
    /// The timestamp of the block in milliseconds since the Unix Epoch. The
    /// timestamp is stored in milliseconds because some blockchains produce
    /// blocks more often than once a second.
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
