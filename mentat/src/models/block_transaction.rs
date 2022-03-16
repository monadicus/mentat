use super::*;

/// BlockTransaction contains a populated Transaction and the BlockIdentifier
/// that contains it.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlockTransaction {
    /// The block_identifier uniquely identifies a block in a particular
    /// network.
    pub block_identifier: BlockIdentifier,
    /// Transactions contain an array of Operations that are attributable to the
    /// same TransactionIdentifier.
    pub transaction: Transaction,
}
