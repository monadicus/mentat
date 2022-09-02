//! The module defines the `MetadataRequest` request.

use super::*;

/// A `MetadataRequest` is utilized in any request where the only argument is
/// optional metadata.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedMetadataRequest {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
