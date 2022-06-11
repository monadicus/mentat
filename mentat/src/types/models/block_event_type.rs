//! The module defines the `BlockEventType` model.

use super::*;

/// `BlockEventType` determines if a [`BlockEvent`] represents the addition or
/// removal of a block.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BlockEventType {
    /// A block was added to the canonical chain.
    #[serde(rename = "block_added")]
    BlockAdded,
    /// A block was removed from the canonical chain in a reorg.
    #[serde(rename = "block_removed")]
    BlockRemoved,
}
