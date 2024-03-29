//! The module defines the `BlockEvent` model.
use super::*;

/// `BlockEvent` represents the addition or removal of a [`BlockIdentifier`]
/// from storage. Streaming `BlockEvent`s allows lightweight clients to update
/// their own state without needing to implement their own syncing logic.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedBlockEvent {
    /// Sequence is the unique identifier of a BlockEvent within the context of
    /// a [`NetworkIdentifier`].
    #[unchecked(usize)]
    pub sequence: isize,
    /// The `BlockIdentifier` uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<UncheckedBlockIdentifier>,
    /// `BlockEventType` determines if a `BlockEvent` represents the addition or
    /// removal of a block.
    #[serde(rename = "type")]
    #[unchecked(option_enum)]
    pub type_: UncheckedBlockEventType,
}
