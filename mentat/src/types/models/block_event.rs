//! The module defines the `BlockEvent` model.

use mentat_macros::Nullable;

use super::*;
use crate::types::BlockIdentifier;

/// `BlockEvent` represents the addition or removal of a [`BlockIdentifier`]
/// from storage. Streaming `BlockEvent`s allows lightweight clients to update
/// their own state without needing to implement their own syncing logic.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableBlockEvent {
    /// Sequence is the unique identifier of a BlockEvent within the context of
    /// a [`NetworkIdentifier`].
    pub sequence: i64,
    /// The `BlockIdentifier` uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<BlockIdentifier>,
    /// `BlockEventType` determines if a `BlockEvent` represents the addition or
    /// removal of a block.
    #[serde(rename = "type")]
    pub type_: BlockEventType,
}
