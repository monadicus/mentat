use mentat::{
    serde::Serialize,
    serde_json::{json, Value},
};

pub fn trim_hash(hash: &str) -> &str {
    if let Some(h) = hash.strip_prefix("0x") {
        h
    } else {
        hash
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinJrpc {
    jsonrpc: String,
    id: String,
    method: String,
    params: Vec<Value>,
}

impl BitcoinJrpc {
    pub fn new<P: Serialize>(method: &str, params: &[P]) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: method.to_string(),
            params: params.iter().map(|p| json!(p)).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "mentat::serde")]
pub struct ScanObjectsDescriptor {
    pub desc: String,
    pub range: u64,
}
