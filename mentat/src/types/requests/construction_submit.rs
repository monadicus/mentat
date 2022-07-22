//! The module defines the `ConstructionSubmitRequest` request.

use super::*;

/// The transaction submission request includes a signed transaction.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct ConstructionSubmitRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub signed_transaction: String,
}
