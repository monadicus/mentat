use super::*;

/// A NetworkListResponse contains all NetworkIdentifiers that the node can
/// serve information for.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}
