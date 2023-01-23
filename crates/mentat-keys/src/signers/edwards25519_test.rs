use mentat_test_utils::TestCase;
use mentat_types::{
    AccountIdentifier,
    CurveType,
    SignatureType,
    SigningPayload,
    UncheckedSignatureType,
};

use crate::{
    errors::KeysError,
    types::{KeyPair, UncheckedKeyPair},
};

// TODO this needs to be a signature type and signature type needs to support
// unknown and empty
fn mock_payload(msg: Vec<u8>, signature_type: UncheckedSignatureType) -> SigningPayload {
    SigningPayload {
        account_identifier: Some(AccountIdentifier {
            address: "test".to_string(),
            ..Default::default()
        }),
        bytes: msg,
        signature_type: Some(signature_type.into()),
        ..Default::default()
    }
}

struct TestSignEdwards25519Payload {
    payload: SigningPayload,
    err: bool,
}

#[test]
fn test_sign_edwards25519() {
    let key_pair = KeyPair::generate(CurveType::Edwards25519).unwrap();
    let signer = key_pair.signer().unwrap();

    let tests = vec![
        TestCase {
            name: "",
            payload: mock_payload(
                vec![0; 32],
                UncheckedSignatureType::from(UncheckedSignatureType::ED25519),
            ),
            criteria: None,
        },
        TestCase {
            name: "",
            payload: mock_payload(vec![0; 32], UncheckedSignatureType::from("")),
            criteria: Some(KeysError::ErrSignUnsupportedPayloadSignatureType),
        },
    ];

    TestCase::run_err_match(tests, |p| signer.sign(p, SignatureType::Ed25519))
}
