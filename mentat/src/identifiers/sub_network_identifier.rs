use indexmap::IndexMap;

use super::*;

/// In blockchains with sharded state, the SubNetworkIdentifier is required to
/// query some object on a specific shard. This identifier is optional for all
/// non-sharded blockchains.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SubNetworkIdentifier {
    pub network: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}

impl From<String> for SubNetworkIdentifier {
    fn from(network: String) -> Self {
        Self {
            network,
            ..Default::default()
        }
    }
}
