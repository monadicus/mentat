//! The module defines the `ConstructionPayloadsResponse` response.

use super::*;

/// `ConstructionTransactionResponse` is returned by `/construction/payloads`.
/// It contains an unsigned transaction blob (that is usually needed to
/// construct the a network transaction from a collection of signatures) and an
/// array of payloads that must be signed by the caller.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConstructionPayloadsResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    pub unsigned_transaction: String,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payloads: Option<Vec<Option<SigningPayload>>>,
}
