//! The module defines the `TransactionIdentifier`.

use std::mem::size_of_val;

use from_tuple::FromTuple;

use super::*;

/// The [`TransactionIdentifier`] uniquely identifies a transaction in a
/// particular network and block or in the mempool.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct TransactionIdentifier {
    /// Any transactions that are attributable only to a block (ex: a block
    /// event) should use the hash of the block as the identifier.
    pub hash: String,
}

impl EstimateSize for TransactionIdentifier {
    fn estimated_size(&self) -> usize {
        size_of_val(self) + size_of_val(self.hash.as_str())
    }
}
