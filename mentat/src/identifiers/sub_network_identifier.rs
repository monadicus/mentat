use indexmap::IndexMap;

use super::*;

/// In blockchains with sharded state, the SubNetworkIdentifier is required to
/// query some object on a specific shard. This identifier is optional for all
/// non-sharded blockchains.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SubNetworkIdentifier {
    pub network: String,
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
