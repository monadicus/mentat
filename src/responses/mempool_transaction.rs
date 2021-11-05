use indexmap::IndexMap;

use super::*;

/// A MempoolTransactionResponse contains an estimate of a mempool transaction. It may not be possible to know the full impact of a transaction in the mempool (ex: fee paid).
#[derive(Serialize, Deserialize)]
pub struct MempoolTransactionResponse {
    /// Transactions contain an array of Operations that are attributable to the same TransactionIdentifier.
    pub transaction: Transaction,
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}