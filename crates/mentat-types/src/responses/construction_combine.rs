//! The module defines the `BlockTransactionResponse` response.

use super::*;

/// `ConstructionCombineResponse` is returned by `/construction/combine`. The
/// network payload will be sent directly to the `/construction/submit`
/// endpoint.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableConstructionCombineResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    pub signed_transaction: String,
}
