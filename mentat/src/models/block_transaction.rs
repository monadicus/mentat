//! The module defines the BlockTransaction model.

use super::*;

/// BlockTransaction contains a populated Transaction and the BlockIdentifier
/// that contains it.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BlockTransaction {
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    pub block_identifier: BlockIdentifier,
    /// [`Transaction`]s contain an array of Operations that are attributable to
    /// the same TransactionIdentifier.
    pub transaction: Transaction,
}
