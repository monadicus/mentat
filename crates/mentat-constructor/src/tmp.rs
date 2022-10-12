//! contains types from database and keypair that haven't been implemented yet
// TODO: remove when types are implemented elsewhere

use mentat_types::{CurveType, PublicKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyPair {
    public_key: Option<PublicKey>,
    private_key: Vec<u8>,
}

impl KeyPair {
    // overrides the default JSON serializer and encodes bytes as hex instead of base64.
    fn serialize_json(&self) -> Result<Value, String> {
        todo!()
    }

    // overrides the default JSON deserializer and decodes bytes from hex instead of base64.
    fn deserialize_json(&self, v: &Value) -> Result<(), String> {
        todo!()
    }
}

// GenerateKeypair returns a Keypair of a specified CurveType
pub fn generate_key_pair(curve: &CurveType) -> Result<KeyPair, String> {
    todo!()
}
