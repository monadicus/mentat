//! The module defines the `MempoolResponse` response.

use super::*;

/// A [`MempoolResponse`] contains all transaction identifiers in the mempool
/// for a particular `network_identifier`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableMempoolResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub transaction_identifiers: Vec<Option<TransactionIdentifier>>,
}
