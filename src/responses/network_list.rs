use super::*;

/// A NetworkListResponse contains all NetworkIdentifiers that the node can serve information for.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}
