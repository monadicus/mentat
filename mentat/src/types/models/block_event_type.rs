//! The module defines the `BlockEventType` model.

use super::*;

/// `BlockEventType` determines if a [`BlockEvent`] represents the addition or
/// removal of a block.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BlockEventType(pub String);

impl BlockEventType {
    /// A block was added to the canonical chain.
    pub const BLOCK_ADDED: &'static str = "block_added";
    /// A block was removed from the canonical chain in a reorg.
    pub const BLOCK_REMOVED: &'static str = "block_removed";

    pub fn valid(&self) -> bool {
        match self.0.as_str() {
            Self::BLOCK_ADDED | Self::BLOCK_REMOVED => true,
            _ => false,
        }
    }
}
