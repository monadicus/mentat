//! The module defines the `NetworkListResponse` response.

use mentat_macros::Nullable;

use super::*;

/// A [`NetworkListResponse`] contains all [`NetworkIdentifier`]s that the node
/// can serve information for.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableNetworkListResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub network_identifiers: Vec<Option<NetworkIdentifier>>,
}
