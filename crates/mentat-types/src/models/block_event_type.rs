//! The module defines the `BlockEventType` model.

use std::fmt;

use super::*;

/// `BlockEventType` determines if a [`BlockEvent`] represents the addition or
/// removal of a block.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UncheckedBlockEventType(String);

impl UncheckedBlockEventType {
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

impl fmt::Display for UncheckedBlockEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UncheckedBlockEventType {
    fn from(bt: String) -> Self {
        Self(bt)
    }
}

impl From<&str> for UncheckedBlockEventType {
    fn from(bt: &str) -> Self {
        bt.to_string().into()
    }
}

/// `BlockEventType` determines if a [`BlockEvent`] represents the addition or
/// removal of a block.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BlockEventType {
    #[default]
    /// A block was added to the canonical chain.
    BlockAdded,
    /// A block was removed from the canonical chain in a reorg.
    BlockRemoved,
}

impl From<UncheckedBlockEventType> for BlockEventType {
    fn from(other: UncheckedBlockEventType) -> Self {
        match other.0.as_ref() {
            UncheckedBlockEventType::BLOCK_ADDED => Self::BlockAdded,
            UncheckedBlockEventType::BLOCK_REMOVED => Self::BlockRemoved,
            i => panic!("unsupported BlockEventType: {i}"),
        }
    }
}

impl From<BlockEventType> for UncheckedBlockEventType {
    fn from(other: BlockEventType) -> Self {
        match other {
            BlockEventType::BlockAdded => Self::BLOCK_ADDED.into(),
            BlockEventType::BlockRemoved => Self::BLOCK_REMOVED.into(),
        }
    }
}

impl fmt::Display for BlockEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockEventType::BlockAdded => write!(f, "block_added"),
            BlockEventType::BlockRemoved => write!(f, "block_removed"),
        }
    }
}
