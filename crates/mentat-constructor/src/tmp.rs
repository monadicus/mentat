//! contains types from database and keypair that haven't been implemented yet
// TODO: remove when types are implemented elsewhere

use mentat_types::{CurveType, PublicKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Transaction is an interface that provides
/// access to a KV store within some transaction
/// context provided by a Database.
///
/// When a Transaction is committed or discarded,
/// all memory utilized is reclaimed. If you want to persist
/// any data retrieved, make sure to make a copy!
pub trait Transaction {
    fn set(&mut self, _: Value, _: Value, _: bool) -> Result<(), ()>;
    fn get(&self, _: &Value) -> Result<(Value, bool), ()>;
    fn delete(&mut self, _: &Value) -> Result<(), ()>;

    fn scan(
        &self,
        // prefix restriction
        _: &Value,
        // seek start
        _: &Value,
        _: fn(&Value, &Value) -> Result<(), ()>,
        // log entries
        _: bool,
        // reverse == true means greatest to least
        _: bool,
    ) -> Result<usize, ()>;

    fn commit(&mut self) -> Result<(), ()>;
    fn discard(&mut self);
}

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
