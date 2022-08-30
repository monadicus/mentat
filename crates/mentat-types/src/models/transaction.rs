//! The module defines the `Transaction` model.

use super::*;

/// [`Transaction`]s contain an array of [`Operation`]s that are attributable to
/// the same [`TransactionIdentifier`].
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedTransaction {
    /// The [`TransactionIdentifier`] uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_identifier: Option<TransactionIdentifier>,
    /// The list of [`Operation`] on the transaction.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub operations: Vec<Option<UncheckedOperation>>,
    /// A optional list of `RelatedTransaction` related to this transaction.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub related_transactions: Vec<Option<UncheckedRelatedTransaction>>,
    /// `Transaction`s that are related to other transactions (like a
    /// cross-shard transaction) should include the transaction_identifier of
    /// these transactions in the metadata.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}

impl EstimateSize for Transaction {
    fn estimated_size(&self) -> usize {
        size_of_val(self)
            + self.transaction_identifier.estimated_size()
            + estimated_vec_size(&self.operations)
            + estimated_vec_size(&self.related_transactions)
            + estimated_metadata_size(&self.metadata)
    }
}
