use indexmap::IndexMap;

use super::*;

/// TransactionIdentifierResponse contains the transaction_identifier of a
/// transaction that was submitted to either /construction/hash or
/// /construction/submit.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TransactionIdentifierResponse {
    /// The transaction_identifier uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    pub transaction_identifier: TransactionIdentifier,
    #[serde(default)]
    pub metadata:               IndexMap<String, Value>,
}
