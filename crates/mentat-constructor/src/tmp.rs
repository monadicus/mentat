//! contains types from keypair that haven't been implemented yet
// TODO: remove when keys is finished

use mentat_types::{CurveType, PublicKey};
use serde::{Deserialize, Serialize};

use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyPair {
    public_key: Option<PublicKey>,
    private_key: Vec<u8>,
}

// TODO incomplete
/// GenerateKeypair returns a Keypair of a specified CurveType
pub fn generate_key_pair(curve: CurveType) -> Result<KeyPair, String> {
    assert_eq!(curve, CurveType::Secp256k1);

    let (secret_key, public_key) = Secp256k1::new().generate_keypair(&mut OsRng);

    Ok(KeyPair {
        public_key: Some(PublicKey {
            bytes: public_key.serialize().to_vec(),
            curve_type: Some(curve),
        }),
        private_key: secret_key.secret_bytes().to_vec(),
    })
}
