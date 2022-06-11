//! The module defines the `NetworkListResponse` response.

use super::*;

/// A `NetworkListResponse` contains all [`NetworkIdentifier`]s that the node
/// can serve information for.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NetworkListResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    pub network_identifiers: Vec<NetworkIdentifier>,
}
