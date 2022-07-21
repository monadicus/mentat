//! The module defines the `MempoolResponse` response.

use super::*;

/// A [`MempoolResponse`] contains all transaction identifiers in the mempool for
/// a particular `network_identifier`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MempoolResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_identifiers: Option<Vec<Option<TransactionIdentifier>>>,
}
