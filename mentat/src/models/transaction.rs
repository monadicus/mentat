//! The module defines the Transaction model.

use indexmap::IndexMap;

use super::*;

/// Transactions contain an array of Operations that are attributable to the
/// same TransactionIdentifier.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Transaction {
    /// The [`TransactionIdentifier`] uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    pub transaction_identifier: TransactionIdentifier,
    /// The list of [`Operation`] on the transaction.
    pub operations: Vec<Operation>,
    /// A optional list of [`RelatedTransaction`] related to this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_transactions: Option<Vec<RelatedTransaction>>,
    /// Transactions that are related to other transactions (like a cross-shard
    /// transaction) should include the tranaction_identifier of these
    /// transactions in the metadata.
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
