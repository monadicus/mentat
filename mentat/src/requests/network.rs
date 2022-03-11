use indexmap::IndexMap;

use super::*;

/// A NetworkRequest is utilized to retrieve some data specific exclusively to a NetworkIdentifier.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NetworkRequest {
    /// The network_identifier specifies which network a particular object is associated with.
    pub network_identifier: NetworkIdentifier,
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
