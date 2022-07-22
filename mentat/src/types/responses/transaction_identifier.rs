//! The module defines the `TransactionIdentifierResponse` response.

use indexmap::IndexMap;

use super::*;

/// [`TransactionIdentifierResponse`] contains the `transaction_identifier` of a
/// transaction that was submitted to either `/construction/hash` or
/// `/construction/submit`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct TransactionIdentifierResponse {
    /// The [`TransactionIdentifier`] uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_identifier: Option<TransactionIdentifier>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub metadata: IndexMap<String, Value>,
}
