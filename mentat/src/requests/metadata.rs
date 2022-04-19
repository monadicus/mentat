//! The module defines the `MetadataRequest` request.

use indexmap::IndexMap;
use wasm_bindgen::prelude::wasm_bindgen;

use super::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export interface JSMetadata = {
    [key in string]: any
};

export type JSMetadataRequest = {
    metadata: JSMetadata,
};

"#;

/// A `MetadataRequest` is utilized in any request where the only argument is
/// optional metadata.
#[wasm_bindgen(typescript_type = "JSMetadataRequest")]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MetadataRequest {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    #[wasm_bindgen(skip)]
    pub metadata: IndexMap<String, Value>,
}
