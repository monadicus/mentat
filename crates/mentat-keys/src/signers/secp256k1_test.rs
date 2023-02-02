use mentat_test_utils::TestCase;
use mentat_types::{AccountIdentifier, CurveType, SignatureType, SigningPayload};

use super::{mock_payload, mock_signature, mock_signer};
use crate::{errors::KeysError, SignerInterface};

const HASH_LENGTH: usize = 32;

#[derive(Debug, Default)]
pub struct Hash([u8; HASH_LENGTH]);

impl Hash {
    fn set_bytes(&mut self, mut bytes: &[u8]) {
        if bytes.len() > HASH_LENGTH {
            bytes = &bytes[bytes.len() - HASH_LENGTH..];
        }
        self.0[HASH_LENGTH - bytes.len()..].copy_from_slice(bytes)
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }
}

fn hash(message: &str) -> Vec<u8> {
    let mut hash = Hash::default();
    hash.set_bytes(message.as_bytes());
    hash.bytes().to_vec()
}

struct TestSecp256k1Payload {
    payload: SigningPayload,
    sig_type: SignatureType,
    sig_len: usize,
}

#[test]
fn test_sign_secp256k1() {
    let signer = mock_signer(CurveType::Secp256k1);

    let tests = vec![
        TestCase {
            name: "secp256k1 ecdsa success",
            payload: TestSecp256k1Payload {
                payload: mock_payload(hash("hello123"), SignatureType::Ecdsa),
                sig_type: SignatureType::Ecdsa,
                sig_len: 64,
            },
            criteria: None,
        },
        TestCase {
            name: "secp256k1 ecdsa recovery success",
            payload: TestSecp256k1Payload {
                payload: mock_payload(hash("hello1234"), SignatureType::EcdsaRecovery),
                sig_type: SignatureType::EcdsaRecovery,
                sig_len: 65,
            },
            criteria: None,
        },
        TestCase {
            name: "secp256k1 Ed25519 unsupported signature type",
            payload: TestSecp256k1Payload {
                payload: mock_payload(hash("hello123"), SignatureType::Ed25519),
                sig_type: SignatureType::Ed25519,
                sig_len: 64,
            },
            criteria: Some(KeysError::ErrSignUnsupportedSignatureType),
        },
        TestCase {
            name: "secp256k1 schnorr1 success",
            payload: TestSecp256k1Payload {
                payload: mock_payload(hash("hello1234"), SignatureType::Schnorr1),
                sig_type: SignatureType::Schnorr1,
                sig_len: 64,
            },
            criteria: None,
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
fn test_verify_secp256k1() {
    let signer = mock_signer(CurveType::Secp256k1);

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
    let signature_esdca_recovery = signer
        .sign(
            SigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".to_string(),
                    ..Default::default()
                }),
                bytes: hash("hello"),
                signature_type: SignatureType::EcdsaRecovery,
                ..Default::default()
            },
            SignatureType::EcdsaRecovery,
        )
        .unwrap();
    let signature_schnorr1 = signer
        .sign(
            SigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".to_string(),
                    ..Default::default()
                }),
                bytes: hash("hello"),
                signature_type: SignatureType::Schnorr1,
                ..Default::default()
            },
            SignatureType::Schnorr1,
        )
        .unwrap();

    let mut simple_bytes = vec![0; 33];
    let hello = "hello".as_bytes();
    simple_bytes[..hello.len()].copy_from_slice(hello);

    let tests = vec![
        TestCase {
            name: "incorrect payload signature type",
            payload: mock_signature(
                SignatureType::Ed25519,
                signer.public_key(),
                hash("hello"),
                simple_bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyUnsupportedSignatureType),
        },
        TestCase {
            name: "verify failed 1",
            payload: mock_signature(
                SignatureType::Ecdsa,
                signer.public_key(),
                hash("hello"),
                simple_bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyFailed),
        },
        TestCase {
            name: "verify failed 2",
            payload: mock_signature(
                SignatureType::Schnorr1,
                signer.public_key(),
                hash("hello"),
                simple_bytes.clone(),
            ),
            criteria: Some(KeysError::ErrVerifyFailed),
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
        TestCase {
            name: "good ecdsa recovery signature",
            payload: mock_signature(
                SignatureType::EcdsaRecovery,
                signer.public_key(),
                hash("hello"),
                signature_esdca_recovery.bytes,
            ),
            criteria: None,
        },
        TestCase {
            name: "good schnorr1 signature",
            payload: mock_signature(
                SignatureType::Schnorr1,
                signer.public_key(),
                hash("hello"),
                signature_schnorr1.bytes,
            ),
            criteria: None,
        },
    ];

    TestCase::run_err_match(tests, |p| signer.verify(p.into()))
}
