//! The module defines the `NetworkRequest` request.

use indexmap::IndexMap;

use super::*;

/// A `NetworkRequest` is utilized to retrieve some data specific exclusively to
/// a [`NetworkIdentifier`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkRequest {
    /// The `NetworkIdentifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
