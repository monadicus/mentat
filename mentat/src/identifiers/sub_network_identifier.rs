//! The module defines the `SubNetworkIdentifier`.

use indexmap::IndexMap;

use super::*;

/// In blockchains with sharded state, the `SubNetworkIdentifier` is required to
/// query some object on a specific shard. This identifier is optional for all
/// non-sharded blockchains.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubNetworkIdentifier {
    /// The network string
    pub network: String,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
