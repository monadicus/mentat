//! The module defines the `NetworkListResponse` response.

use wasm_bindgen::prelude::wasm_bindgen;

use super::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export type JSSubNetworkIdentifier = {
    network: string,
    metadata: JSMetadata,
};

export type JSNetworkIdentifier = {
    blockchain: string,
    network: string,
    sub_network_identifier?: JSSubNetworkIdentifier,
};

export type JSNetworkListResponse = {
    network_identifiers: JSNetworkIdentifier[],
};


"#;

/// A `NetworkListResponse` contains all [`NetworkIdentifier`]s that the node
/// can serve information for.
#[wasm_bindgen(typescript_type = "JSNetworkListResponse")]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NetworkListResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    #[wasm_bindgen(skip)]
    pub network_identifiers: Vec<NetworkIdentifier>,
}
