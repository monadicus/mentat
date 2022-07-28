//! The module defines the `BlockEventType` model.

use std::fmt;

use super::*;

/// `BlockEventType` determines if a [`BlockEvent`] represents the addition or
/// removal of a block.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BlockEventType(String);

impl BlockEventType {
    /// A block was added to the canonical chain.
    pub const BLOCK_ADDED: &'static str = "block_added";
    /// A block was removed from the canonical chain in a reorg.
    pub const BLOCK_REMOVED: &'static str = "block_removed";

    /// returns true if the `BlockEventType` is a valid type
    pub fn valid(&self) -> bool {
        matches!(self.0.as_str(), Self::BLOCK_ADDED | Self::BLOCK_REMOVED)
    }
}

impl fmt::Display for BlockEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for BlockEventType {
    fn from(bt: String) -> Self {
        Self(bt)
    }
}

impl From<&str> for BlockEventType {
    fn from(bt: &str) -> Self {
        bt.to_string().into()
    }
}
