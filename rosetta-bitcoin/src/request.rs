use mentat::serde::Serialize;

pub fn trim_hash(hash: &str) -> &str {
    if let Some(h) = hash.strip_prefix("0x") {
        h
    } else {
        hash
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "mentat::serde")]
pub struct BitcoinJrpc<P: Serialize> {
    jsonrpc: String,
    id: String,
    method: String,
    params: Vec<P>,
}

impl<P: Serialize> BitcoinJrpc<P> {
    pub fn new(method: &str, params: Vec<P>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: method.to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "mentat::serde")]
pub struct ScanObjectsDescriptor {
    pub desc: String,
    pub range: u64,
}
