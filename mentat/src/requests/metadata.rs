//! The module defines the `MetadataRequest` request.

use indexmap::IndexMap;

use super::*;

/// A `MetadataRequest` is utilized in any request where the only argument is
/// optional metadata.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MetadataRequest {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
