use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const METADATA_TYPE: &'static str = r#"
export type JSMetadata = {
    [key in string]: any
};
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JSMetadata")]
    pub type JSMetadata;
}
