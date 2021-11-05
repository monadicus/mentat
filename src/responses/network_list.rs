use super::*;

/// A NetworkListResponse contains all NetworkIdentifiers that the node can serve information for.
#[derive(Serialize, Deserialize)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}