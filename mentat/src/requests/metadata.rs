//! The module defines the `MetadataRequest` request.

use indexmap::IndexMap;

#[cfg(feature = "client")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::*;
#[cfg(feature = "client")]
use crate::identifiers::JSMetadata;

/// A `MetadataRequest` is utilized in any request where the only argument is
/// optional metadata.
#[cfg(feature = "client")]
#[wasm_bindgen]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MetadataRequest {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    #[wasm_bindgen(skip)]
    pub metadata: IndexMap<String, Value>,
}

#[cfg(feature = "client")]
#[wasm_bindgen]
impl MetadataRequest {
    #[wasm_bindgen(catch, constructor)]
    pub fn new(metadata: Option<JSMetadata>) -> Self {
        Self {
            metadata: if let Some(data) = metadata {
                data.into_serde().unwrap()
            } else {
                Default::default()
            },
        }
    }

    #[wasm_bindgen(getter = metadata, typescript_type = "JSMetadata")]
    pub fn metadata(&self) -> JsValue {
        JsValue::from_serde(&self.metadata).unwrap()
    }

    #[wasm_bindgen(setter = metadata, typescript_type = "JSMetadata")]
    pub fn set_test3(&mut self, field: JSMetadata) {
        self.metadata = field.into_serde().unwrap()
    }
}
