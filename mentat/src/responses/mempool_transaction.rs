//! The module defines the `MempoolTransactionResponse` response.

use indexmap::IndexMap;

use super::*;

/// A `MempoolTransactionResponse` contains an estimate of a mempool
/// transaction. It may not be possible to know the full impact of a transaction
/// in the mempool (ex: fee paid).
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MempoolTransactionResponse {
    /// `Transactions` contain an array of [`Operation`]s that are attributable
    /// to the same [`TransactionIdentifier`].
    pub transaction: Transaction,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
