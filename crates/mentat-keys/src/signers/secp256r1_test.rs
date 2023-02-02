use mentat_test_utils::TestCase;
use mentat_types::{AccountIdentifier, CurveType, SignatureType, SigningPayload};

use super::*;

struct TestSecp256r1Payload {
    payload: SigningPayload,
    sig_type: SignatureType,
    sig_len: usize,
}

#[test]
fn test_sign_secp256r1() {
    let signer = mock_signer(CurveType::Secp256r1);

    let tests = vec![
        TestCase {
            name: "secp256r1 ecdsa success",
            payload: TestSecp256r1Payload {
                payload: mock_payload(hash("hello123"), SignatureType::Ecdsa),
                sig_type: SignatureType::Ecdsa,
                sig_len: 64,
            },
            criteria: None,
        },
        TestCase {
            name: "secp256r1 ecdsa unsupported signature type",
            payload: TestSecp256r1Payload {
                payload: mock_payload(hash("hello1234"), SignatureType::EcdsaRecovery),
                sig_type: SignatureType::EcdsaRecovery,
                sig_len: 65,
            },
            criteria: Some(KeysError::ErrSignUnsupportedSignatureType),
        },
        TestCase {
            name: "secp256r1 Ed25519 unsupported signature type",
            payload: TestSecp256r1Payload {
                payload: mock_payload(hash("hello123"), SignatureType::Ed25519),
                sig_type: SignatureType::Ed25519,
                sig_len: 64,
            },
            criteria: Some(KeysError::ErrSignUnsupportedSignatureType),
        },
        TestCase {
            name: "secp256r1 schnorr1 unsupported signature type",
            payload: TestSecp256r1Payload {
                payload: mock_payload(hash("hello1234"), SignatureType::Schnorr1),
                sig_type: SignatureType::Schnorr1,
                sig_len: 64,
            },
            criteria: Some(KeysError::ErrSignUnsupportedSignatureType),
        },
    ];

    TestCase::run_err_match(tests, |p| {
        let sig = signer.sign(p.payload, p.sig_type)?;
        assert_eq!(sig.bytes.len(), p.sig_len);
        assert_eq!(signer.public_key(), sig.public_key);

        Ok::<_, KeysError>(sig)
    })
}

#[test]
fn test_verify_secp256r1() {
    let signer = mock_signer(CurveType::Secp256r1);

    let signature_esdca = signer
        .sign(
            SigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".to_string(),
                    ..Default::default()
                }),
                bytes: hash("hello"),
                signature_type: SignatureType::Ecdsa,
                ..Default::default()
            },
            SignatureType::Ecdsa,
        )
        .unwrap();

    let mut simple_bytes = vec![0; 33];
    let hello = "hello".as_bytes();
    simple_bytes[..hello.len()].copy_from_slice(hello);

    let tests = vec![
        TestCase {
            name: "verify failed 1",
            payload: mock_signature(
                SignatureType::Ecdsa,
                signer.public_key(),
                "hello".as_bytes().to_vec(),
                signature_esdca.bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyFailed),
        },
        TestCase {
            name: "incorrect payload signature type Ed25519",
            payload: mock_signature(
                SignatureType::Ed25519,
                signer.public_key(),
                hash("hello"),
                simple_bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyUnsupportedSignatureType),
        },
        TestCase {
            name: "verify failed 2",
            payload: mock_signature(
                SignatureType::Ecdsa,
                signer.public_key(),
                hash("hello"),
                simple_bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyFailed),
        },
        TestCase {
            name: "incorrect payload signature type ecdsa recovery",
            payload: mock_signature(
                SignatureType::EcdsaRecovery,
                signer.public_key(),
                hash("hello"),
                simple_bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyUnsupportedSignatureType),
        },
        TestCase {
            name: "incorrect payload signature type schnorr1",
            payload: mock_signature(
                SignatureType::Schnorr1,
                signer.public_key(),
                hash("hello"),
                simple_bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyUnsupportedSignatureType),
        },
        TestCase {
            name: "good ecdsa signature",
            payload: mock_signature(
                SignatureType::Ecdsa,
                signer.public_key(),
                hash("hello"),
                signature_esdca.bytes,
            ),
            criteria: None,
        },
    ];

    TestCase::run_err_match(tests, |p| signer.verify(p.into()))
}
