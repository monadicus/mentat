use super::*;

/// BlockEvent represents the addition or removal of a BlockIdentifier from
/// storage. Streaming BlockEvents allows lightweight clients to update their
/// own state without needing to implement their own syncing logic.
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockEvent {
    /// sequence is the unique identifier of a BlockEvent within the context of
    /// a NetworkIdentifier.
    pub sequence:         u64,
    /// The block_identifier uniquely identifies a block in a particular
    /// network.
    pub block_identifier: BlockIdentifier,
    /// BlockEventType determines if a BlockEvent represents the addition or
    /// removal of a block.
    #[serde(rename = "type")]
    pub type_:            BlockEventType,
}
