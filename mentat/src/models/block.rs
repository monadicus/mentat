//! The module defines the Block model.

use indexmap::IndexMap;

use super::*;

/// Blocks contain an array of Transactions that occurred at a particular
/// BlockIdentifier. A hard requirement for blocks returned by Rosetta
/// implementations is that they MUST be inalterable: once a client has
/// requested and received a block identified by a specific BlockIdentifier,
/// all future calls for that same BlockIdentifier must return the same block
/// contents.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Block {
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    pub block_identifier: BlockIdentifier,
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    pub parent_block_identifier: BlockIdentifier,
    /// The timestamp of the block in milliseconds since the Unix Epoch. The
    /// timestamp is stored in milliseconds because some blockchains produce
    /// blocks more often than once a second.
    pub timestamp: u64,
    /// The list of [`Transaction`]s related to the block.
    pub transactions: Vec<Transaction>,
    /// Additional metadata related to the Block.
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
