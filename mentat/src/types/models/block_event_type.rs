//! The module defines the `BlockEventType` model.

use std::fmt;

use super::*;

/// `BlockEventType` determines if a [`BlockEvent`] represents the addition or
/// removal of a block.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NullableBlockEventType(String);

impl NullableBlockEventType {
    /// A block was added to the canonical chain.
    pub const BLOCK_ADDED: &'static str = "block_added";
    /// A block was removed from the canonical chain in a reorg.
    pub const BLOCK_REMOVED: &'static str = "block_removed";

    /// returns true if the `BlockEventType` is a valid type
    pub fn valid(&self) -> bool {
        matches!(self.0.as_str(), Self::BLOCK_ADDED | Self::BLOCK_REMOVED)
    }
    
    /// returns true if the underlying string is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for NullableBlockEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NullableBlockEventType {
    fn from(bt: String) -> Self {
        Self(bt)
    }
}

impl From<&str> for NullableBlockEventType {
    fn from(bt: &str) -> Self {
        bt.to_string().into()
    }
}

/// `BlockEventType` determines if a [`BlockEvent`] represents the addition or
/// removal of a block.
#[derive(Clone, Debug, Default)]
pub enum BlockEventType {
    #[default]
    /// A block was added to the canonical chain.
    Added,
    /// A block was removed from the canonical chain in a reorg.
    Removed,
}

impl From<NullableBlockEventType> for BlockEventType {
    fn from(other: NullableBlockEventType) -> Self {
        match other.0.as_ref() {
            NullableBlockEventType::BLOCK_ADDED => Self::Added,
            NullableBlockEventType::BLOCK_REMOVED => Self::Removed,
            i => panic!("unsupported BlockEventType: {i}"),
        }
    }
}

impl From<BlockEventType> for NullableBlockEventType {
    fn from(other: BlockEventType) -> Self {
        match other {
            BlockEventType::Added => Self::BLOCK_ADDED.into(),
            BlockEventType::Removed => Self::BLOCK_REMOVED.into(),
        }
    }
}
