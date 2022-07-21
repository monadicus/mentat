//! The module defines the ConstructionCombineRequest request.

use super::*;

/// [`ConstructionCombineRequest`] is the input to the `/construction/combine`
/// endpoint. It contains the unsigned transaction blob returned by
/// `/construction/payloads` and all required signatures to create a network
/// transaction.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConstructionCombineRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub unsigned_transaction: String,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<Option<Signature>>>,
}
