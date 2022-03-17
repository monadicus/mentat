use mentat::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "mentat::serde")]
pub struct SnarkosJrpc<P: Serialize> {
    jsonrpc: String,
    id: String,
    method: String,
    params: Vec<P>,
}

impl<P: Serialize> SnarkosJrpc<P> {
    pub fn new(method: &str, params: Vec<P>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: method.to_string(),
            params,
        }
    }
}
