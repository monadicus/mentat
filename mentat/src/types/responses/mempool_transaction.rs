//! The module defines the `MempoolTransactionResponse` response.

use indexmap::IndexMap;
use mentat_macros::Nullable;

use super::*;

/// A [`MempoolTransactionResponse`] contains an estimate of a mempool
/// transaction. It may not be possible to know the full impact of a transaction
/// in the mempool (ex: fee paid).
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableMempoolTransactionResponse {
    /// [`Transaction`]s contain an array of [`Operation`]s that are
    /// attributable to the same [`TransactionIdentifier`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<NullableTransaction>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
