//! The module defines the `TransactionIdentifierResponse` response.

use indexmap::IndexMap;

use super::*;

/// `TransactionIdentifierResponse` contains the `transaction_identifier` of a
/// transaction that was submitted to either `/construction/hash` or
/// `/construction/submit`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TransactionIdentifierResponse {
    /// The `TransactionIdentifier` uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    pub transaction_identifier: TransactionIdentifier,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
