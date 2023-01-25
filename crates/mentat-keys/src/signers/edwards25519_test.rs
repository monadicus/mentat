use mentat_test_utils::TestCase;
use mentat_types::{
    AccountIdentifier,
    CurveType,
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
};

use crate::{errors::KeysError, types::KeyPair, Signer, SignerInterface};

fn mock_signer() -> Signer {
    let key_pair = KeyPair::generate(CurveType::Edwards25519).unwrap();

    key_pair.signer().unwrap()
}

fn mock_payload(msg: Vec<u8>, signature_type: SignatureType) -> SigningPayload {
    SigningPayload {
        account_identifier: Some(AccountIdentifier {
            address: "test".to_string(),
            ..Default::default()
        }),
        bytes: msg,
        signature_type,
        ..Default::default()
    }
}

#[test]
fn test_sign_edwards25519() {
    let signer = mock_signer();

    let tests = vec![
        TestCase {
            name: "correct payload signature type",
            payload: mock_payload(vec![0; 32], SignatureType::Ed25519),
            criteria: None,
        },
        TestCase {
            name: "implicit payload signature type",
            payload: mock_payload(vec![0; 32], SignatureType::EmptyString),
            criteria: None,
        },
        TestCase {
            name: "incorrect payload signature type 1",
            payload: mock_payload(vec![0; 33], SignatureType::Ecdsa),
            criteria: Some(KeysError::ErrSignUnsupportedPayloadSignatureType),
        },
        TestCase {
            name: "incorrect payload signature type 2",
            payload: mock_payload(vec![0; 34], SignatureType::EcdsaRecovery),
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
    let signing_payload = SigningPayload {
        account_identifier: Some(AccountIdentifier {
            address: "test".into(),
            ..Default::default()
        }),
        bytes: msg,
        signature_type,
        ..Default::default()
    };

    Signature {
        signing_payload,
        public_key,
        signature_type,
        bytes: sig,
    }
}

#[test]
fn test_verify_edwards_25519() {
    let signer = mock_signer();

    let mut simple_bytes = vec![0; 32];
    let hello = "hello".as_bytes();
    simple_bytes[..hello.len()].copy_from_slice(hello);

    let payload = SigningPayload {
        account_identifier: Some(AccountIdentifier {
            address: "test".to_string(),
            ..Default::default()
        }),
        bytes: simple_bytes.clone(),
        signature_type: SignatureType::Ed25519,
        ..Default::default()
    };
    let test_sig = signer.sign(payload, SignatureType::Ed25519).unwrap();

    let tests = vec![
        TestCase {
            name: "incorrect payload signature type 1",
            payload: mock_signature(
                SignatureType::Ecdsa,
                signer.public_key(),
                simple_bytes.clone(),
                simple_bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyUnsupportedPayloadSignatureType),
        },
        TestCase {
            name: "incorrect payload signature type 2",
            payload: mock_signature(
                SignatureType::EcdsaRecovery,
                signer.public_key(),
                simple_bytes.clone(),
                simple_bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyUnsupportedPayloadSignatureType),
        },
        TestCase {
            name: "incorrect payload signature msg",
            payload: mock_signature(
                SignatureType::Ed25519,
                signer.public_key(),
                {
                    let mut simple_bytes = vec![0; 40];
                    let hello = "hello".as_bytes();
                    simple_bytes[..hello.len()].copy_from_slice(hello);
                    simple_bytes
                },
                test_sig.bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyFailed),
        },
        TestCase {
            name: "correct payload signature",
            payload: mock_signature(
                SignatureType::Ed25519,
                signer.public_key(),
                simple_bytes.clone(),
                test_sig.bytes,
            ),
            criteria: None,
        },
    ];

    TestCase::run_err_match(tests, |p| signer.verify(p.into()))
}
