//! The module defines the `ConstructionSubmitRequest` request.

use super::*;

/// The transaction submission request includes a signed transaction.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionSubmitRequest {
    /// The `network_identifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    #[allow(clippy::missing_docs_in_private_items)]
    pub signed_transaction: String,
}
