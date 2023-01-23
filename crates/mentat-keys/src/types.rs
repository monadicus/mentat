use mentat_macros::Unchecked;
use mentat_types::{bytes_to_hex_str, null_default_bytes_to_hex, PublicKey, UncheckedPublicKey};
use serde::{Deserialize, Serialize};

/// `KeyPair` contains a private key and its associated public key.
#[derive(Clone, Debug, Serialize, Deserialize, Unchecked)]
pub struct UncheckedKeyPair {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<UncheckedPublicKey>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "bytes_to_hex_str",
        deserialize_with = "null_default_bytes_to_hex"
    )]
    // Needs to serialize and deserialize from hex
    pub private_key: Vec<u8>,
}
