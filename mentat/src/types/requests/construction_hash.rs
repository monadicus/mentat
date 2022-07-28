//! The module defines the `ConstructionHashRequest` request.

use mentat_macros::Nullable;

use super::*;

/// [`ConstructionHashRequest`] is the input to the `/construction/hash`
/// endpoint.
#[derive(Clone, Debug, Deserialize, Serialize, Default, Nullable)]
#[serde(default)]
pub struct NullableConstructionHashRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub signed_transaction: String,
}
