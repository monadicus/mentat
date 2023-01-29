use mentat_test_utils::TestCase;
use mentat_types::{CurveType, SignatureType, SigningPayload};

use super::{mock_payload, mock_signer};
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
                sig_len: 65,
            },
            criteria: Some(KeysError::ErrSignUnsupportedSignatureType),
        },
        TestCase {
            name: "secp256k1 schnorr1 success",
            payload: TestSecp256k1Payload {
                payload: mock_payload(hash("hello1234"), SignatureType::Schnorr1),
                sig_type: SignatureType::Schnorr1,
                sig_len: 65,
            },
            criteria: None,
        },
    ];

    // TODO if not an error also check
    // assert.Equal(t, len(signature.Bytes), test.sigLen)
    // assert.Equal(t, signerSecp256k1.PublicKey(), signature.PublicKey)
    TestCase::run_err_match(tests, |p| signer.sign(p.payload, p.sig_type))
}
