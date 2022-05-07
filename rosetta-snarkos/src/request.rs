use mentat::{
    serde::Serialize,
    serde_json::{json, Value},
};

#[derive(Debug, Serialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkosJrpc {
    jsonrpc: String,
    id: String,
    method: String,
    params: Vec<Value>,
}

impl SnarkosJrpc {
    pub fn new<P: Serialize>(method: &str, params: Vec<P>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: method.to_string(),
            params: params.iter().map(|p| json!(p)).collect(),
        }
    }
}
