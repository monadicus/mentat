use mentat_types::CurveType;

use crate::types::KeyPair;

#[test]
fn test_json_encoding() {}

#[test]
fn test_generate_key_pair_secp256k1() {}

#[test]
fn test_generate_key_pair_edwards25519() {}

#[test]
fn test_generate_key_pair_pallas() {}

fn _mock_keypair(private_key: Vec<u8>, curve: CurveType) -> KeyPair {
    let mut key_pair = KeyPair::generate(curve).unwrap();
    key_pair.private_key = private_key;
    key_pair
}

#[test]
fn test_key_pair_validity() {}

#[test]
fn test_import_private_key() {}
