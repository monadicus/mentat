//! The module defines the `NetworkRequest` request.

use indexmap::IndexMap;
use mentat_macros::Nullable;

use super::*;

/// A [`NetworkRequest`] is utilized to retrieve some data specific exclusively
/// to a [`NetworkIdentifier`].
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableNetworkRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub metadata: IndexMap<String, Value>,
}
