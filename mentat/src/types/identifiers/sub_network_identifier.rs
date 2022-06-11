//! The module defines the `SubNetworkIdentifier`.

use indexmap::IndexMap;

use super::*;
use crate::types::Sortable;

/// In blockchains with sharded state, the `SubNetworkIdentifier` is required to
/// query some object on a specific shard. This identifier is optional for all
/// non-sharded blockchains.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubNetworkIdentifier {
    /// The network string
    pub network: String,
    #[allow(clippy::missing_docs_in_private_items)]
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

impl Sortable for SubNetworkIdentifier {
    fn sort(&self) -> Self {
        let mut new = self.clone();
        new.metadata.sort_keys();
        new
    }
}
