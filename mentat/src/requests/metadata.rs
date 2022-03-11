use indexmap::IndexMap;

use super::*;

/// A MetadataRequest is utilized in any request where the only argument is optional metadata.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MetadataRequest {
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
