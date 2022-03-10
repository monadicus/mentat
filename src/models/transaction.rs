use indexmap::IndexMap;

use super::*;

/// Transactions contain an array of Operations that are attributable to the same TransactionIdentifier.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    /// The transaction_identifier uniquely identifies a transaction in a particular network and block or in the mempool.
    pub transaction_identifier: TransactionIdentifier,
    pub operations: Vec<Operation>,
    pub related_transactions: Option<Vec<RelatedTransaction>>,
    /// Transactions that are related to other transactions (like a cross-shard transaction) should include the tranaction_identifier of these transactions in the metadata.
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
