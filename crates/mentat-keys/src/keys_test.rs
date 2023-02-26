use mentat_test_utils::TestCase;
use mentat_types::{decode_from_hex_string, CurveType, UncheckedCurveType};
use serde::{Deserialize, Serialize};

use crate::{
    errors::KeysError,
    keys::PRIV_KEY_BYTES_LEN,
    types::{KeyPair, UncheckedKeyPair},
};

#[test]
fn test_json_encoding() {
    let secp256k1 = KeyPair::generate(CurveType::Secp256k1).unwrap();
    let edwards25519 = KeyPair::generate(CurveType::Edwards25519).unwrap();

    for key_pair in [secp256k1, edwards25519].into_iter() {
        let kpb = serde_json::to_string(&key_pair).unwrap();

        #[derive(Debug, Serialize, Deserialize)]
        struct Simple {
            private_key: String,
        }

        let simple: Simple = serde_json::from_str(&kpb).unwrap();
        let b = decode_from_hex_string(simple.private_key).unwrap();
        assert_eq!(b, key_pair.private_key);

        let kp: KeyPair = serde_json::from_str(&kpb).unwrap();
        assert_eq!(kp, key_pair);
    }
}

#[test]
fn test_generate_key_pair_secp256k1() {
    let kp = KeyPair::generate(CurveType::Secp256k1).unwrap();

    assert_eq!(kp.public_key.curve_type, CurveType::Secp256k1);
    assert_eq!(kp.private_key.len(), PRIV_KEY_BYTES_LEN);
}

#[test]
fn test_generate_key_pair_edwards25519() {
    let kp = KeyPair::generate(CurveType::Edwards25519).unwrap();

    assert_eq!(kp.public_key.curve_type, CurveType::Edwards25519);
    assert_eq!(kp.private_key.len(), PRIV_KEY_BYTES_LEN);
}

#[test]
fn test_generate_key_pair_pallas() {
    let kp = KeyPair::generate(CurveType::Pallas).unwrap();

    assert_eq!(kp.public_key.curve_type, CurveType::Pallas);
    assert_eq!(kp.private_key.len(), PRIV_KEY_BYTES_LEN);
}

fn mock_keypair(private_key: Vec<u8>, curve: CurveType) -> UncheckedKeyPair {
    let mut key_pair = KeyPair::generate(curve).unwrap();
    key_pair.private_key = private_key;
    key_pair.into()
}

#[test]
fn test_key_pair_validity() {
    let mut kp: UncheckedKeyPair = KeyPair::generate(CurveType::Edwards25519).unwrap().into();

    if let Some(ref mut pk) = kp.public_key {
        pk.curve_type = UncheckedCurveType("Blah".to_string());
    }
    let err = kp.is_valid().unwrap_err();
    assert!(err.to_string().contains("not a supported CurveType"));

    let tests = vec![
        TestCase {
            name: "secp256k1 invalid length 1",
            payload: mock_keypair(vec![0; 33], CurveType::Secp256k1),
            criteria: Some(KeysError::ErrPrivKeyLengthInvalid),
        },
        TestCase {
            name: "secp256k1 invalid length 2",
            payload: mock_keypair(vec![0; 31], CurveType::Secp256k1),
            criteria: Some(KeysError::ErrPrivKeyLengthInvalid),
        },
        TestCase {
            name: "secp256k1 invalid length 3",
            payload: mock_keypair(vec![0; 0], CurveType::Secp256k1),
            criteria: Some(KeysError::ErrPrivKeyLengthInvalid),
        },
        TestCase {
            name: "edwards25519 invalid length 1",
            payload: mock_keypair(vec![0; 33], CurveType::Secp256k1),
            criteria: Some(KeysError::ErrPrivKeyLengthInvalid),
        },
        TestCase {
            name: "edwards25519 invalid length 2",
            payload: mock_keypair(vec![0; 31], CurveType::Secp256k1),
            criteria: Some(KeysError::ErrPrivKeyLengthInvalid),
        },
        TestCase {
            name: "edwards25519 invalid length 3",
            payload: mock_keypair(vec![0; 0], CurveType::Secp256k1),
            criteria: Some(KeysError::ErrPrivKeyLengthInvalid),
        },
    ];

    TestCase::run_err_match(tests, |p| p.is_valid())
}

#[test]
fn test_import_private_key() {}
