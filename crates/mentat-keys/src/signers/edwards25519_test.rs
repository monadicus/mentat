use mentat_test_utils::TestCase;
use mentat_types::{
    AccountIdentifier,
    CurveType,
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
    UncheckedSignatureType,
};

use crate::{
    errors::KeysError,
    types::{KeyPair, UncheckedKeyPair},
    Signer,
    SignerInterface,
};

fn mock_signer() -> Signer {
    let key_pair = KeyPair::generate(CurveType::Edwards25519).unwrap();

    key_pair.signer().unwrap()
}

// TODO this needs to be a signature type and signature type needs to support
// unknown and empty
fn mock_payload(msg: Vec<u8>, signature_type: UncheckedSignatureType) -> SigningPayload {
    SigningPayload {
        account_identifier: Some(AccountIdentifier {
            address: "test".to_string(),
            ..Default::default()
        }),
        bytes: msg,
        signature_type: signature_type.into(),
        ..Default::default()
    }
}

#[test]
fn test_sign_edwards25519() {
    let signer = mock_signer();

    let tests = vec![
        TestCase {
            name: "correct payload signature type",
            payload: mock_payload(vec![0; 32], UncheckedSignatureType::ED25519.into()),
            criteria: None,
        },
        TestCase {
            name: "implicit payload signature type",
            payload: mock_payload(vec![0; 32], UncheckedSignatureType::from("")),
            criteria: None,
        },
        TestCase {
            name: "incorrect payload signature type 1",
            payload: mock_payload(vec![0; 33], UncheckedSignatureType::ECDSA.into()),
            criteria: Some(KeysError::ErrSignUnsupportedPayloadSignatureType),
        },
        TestCase {
            name: "incorrect payload signature type 2",
            payload: mock_payload(vec![0; 34], UncheckedSignatureType::ECDSA.into()),
            criteria: Some(KeysError::ErrSignUnsupportedPayloadSignatureType),
        },
    ];

    // TODO if not an error also check
    // assert.NoError(t, err)
    // assert.Len(t, signature.Bytes, 64)
    // assert.Equal(t, signerEdwards25519.PublicKey(), signature.PublicKey)
    TestCase::run_err_match(tests, |p| signer.sign(p, SignatureType::Ed25519))
}

fn mock_signature(
    signature_type: SignatureType,
    public_key: PublicKey,
    msg: Vec<u8>,
    sig: Vec<u8>,
) -> Signature {
    let payload = SigningPayload {
        account_identifier: Some(AccountIdentifier {
            address: "test".into(),
            ..Default::default()
        }),
        bytes: msg,
        signature_type,
        ..Default::default()
    };
    todo!()
}
