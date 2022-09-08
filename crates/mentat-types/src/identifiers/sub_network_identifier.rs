//! The module defines the `SubNetworkIdentifier`.

use super::*;

/// In blockchains with sharded state, the `SubNetworkIdentifier` is required to
/// query some object on a specific shard. This identifier is optional for all
/// non-sharded blockchains.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct SubNetworkIdentifier {
    /// The network string
    #[serde(
        serialize_with = "string_to_uppercase",
        deserialize_with = "string_as_uppercase"
    )]
    pub network: String,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: Metadata,
}

impl From<&str> for SubNetworkIdentifier {
    fn from(network: &str) -> Self {
        Self {
            network: network.to_uppercase(),
            ..Default::default()
        }
    }
}

impl Sortable for SubNetworkIdentifier {
    fn sort(&self) -> Self {
        let mut new = Self {
            network: self.network.to_uppercase(),
            metadata: self.metadata.clone(),
        };
        new.metadata.sort_keys();
        new
    }
}

impl EstimateSize for SubNetworkIdentifier {
    fn estimated_size(&self) -> usize {
        size_of_val(self)
            + size_of_val(self.network.as_str())
            + estimated_metadata_size(&self.metadata)
    }
}
