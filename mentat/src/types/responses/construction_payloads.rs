//! The module defines the `ConstructionPayloadsResponse` response.

use mentat_macros::Nullable;

use super::*;

/// `ConstructionTransactionResponse` is returned by `/construction/payloads`.
/// It contains an unsigned transaction blob (that is usually needed to
/// construct the a network transaction from a collection of signatures) and an
/// array of payloads that must be signed by the caller.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableConstructionPayloadsResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    pub unsigned_transaction: String,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub payloads: Vec<Option<NullableSigningPayload>>,
}
