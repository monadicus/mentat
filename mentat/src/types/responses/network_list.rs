//! The module defines the `NetworkListResponse` response.

use super::*;

/// A [`NetworkListResponse`] contains all [`NetworkIdentifier`]s that the node
/// can serve information for.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NetworkListResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifiers: Option<Vec<Option<NetworkIdentifier>>>,
}
