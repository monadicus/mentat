use mentat_types::{
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
    UncheckedSignature,
    UncheckedSignatureType,
};
use secp256k1::{Message, Secp256k1, SecretKey};

use super::SignerInterface;
use crate::{
    errors::{KeysError, KeysResult},
    types::{KeyPair, UncheckedKeyPair},
};

/// `SignerSecp256k1` is initialized from a `UncheckedKeyPair`.
pub struct SignerSecp256k1 {
    pub key_pair: UncheckedKeyPair,
}

impl SignerInterface for SignerSecp256k1 {
    fn public_key(&self) -> PublicKey {
        let kp: KeyPair = self.key_pair.clone().into();
        kp.public_key
    }

    fn sign(&self, payload: SigningPayload, sig_type: SignatureType) -> KeysResult<Signature> {
        let valid_key_pair = self
            .key_pair
            .clone()
            .is_valid()
            .map_err(|err| format!("key pair is invalid: {err}"))?;

        if !(payload.signature_type == sig_type
            || payload.signature_type == SignatureType::EmptyString)
        {
            Err(format!(
                "signing payload signature type {} is invalid: {}",
                payload.signature_type,
                KeysError::ErrSignUnsupportedPayloadSignatureType
            ))?;
        }
        let secp = Secp256k1::new();
        let msg = Message::from_slice(&payload.bytes).expect("TODO");
        let private_key = SecretKey::from_slice(&self.key_pair.private_key).unwrap();

        // TODO sign doesn't error
        let sig = match sig_type {
            SignatureType::Ecdsa => secp
                .sign_ecdsa(&msg, &private_key)
                .serialize_compact()
                .to_vec(),
            SignatureType::EcdsaRecovery => secp
                .sign_ecdsa_low_r(&msg, &private_key)
                .serialize_compact()
                .to_vec(),
            SignatureType::Schnorr1 => {
                let key_pair = private_key.keypair(&secp);
                secp.sign_schnorr_no_aux_rand(&msg, &key_pair)
                    .as_ref()
                    .to_vec()
            }
            _ => {
                return Err(KeysError::from(format!(
                    "signature type {sig_type} is invalid: {}",
                    KeysError::ErrSignUnsupportedSignatureType
                )));
            }
        };

        Ok(Signature {
            signature_type: payload.signature_type,
            signing_payload: payload,
            public_key: valid_key_pair.public_key,
            bytes: sig,
        })
    }

    fn verify(&self, signature: UncheckedSignature) -> KeysResult<()> {
        if signature.signature_type != UncheckedSignatureType::ED25519.into() {
            Err(format!(
                "expected signing payload signature type {} but got {}: {}",
                SignatureType::Ed25519,
                signature.signature_type,
                KeysError::ErrVerifyUnsupportedPayloadSignatureType
            ))?;
        }

        mentat_asserter::signatures(&[Some(&signature)])
            .map_err(|err| format!("signature is invalid: {err}"))?;
        let signature: Signature = signature.into();

        let secp = Secp256k1::new();
        let msg = Message::from_slice(&signature.signing_payload.bytes).expect("TODO");
        let private_key = SecretKey::from_slice(&self.key_pair.private_key).unwrap();

        if !match signature.signature_type {
            SignatureType::Ecdsa | SignatureType::EcdsaRecovery => {
                let pub_key = private_key.public_key(&secp);
                let sig =
                    secp256k1::ecdsa::Signature::from_compact(&signature.bytes).expect("TODO");
                secp.verify_ecdsa(&msg, &sig, &pub_key).is_ok()
            }
            SignatureType::Schnorr1 => {
                let (pub_key, _) = private_key.x_only_public_key(&secp);
                let sig =
                    secp256k1::schnorr::Signature::from_slice(&signature.bytes).expect("TODO");
                secp.verify_schnorr(&sig, &msg, &pub_key).is_ok()
            }
            _ => {
                return Err(KeysError::from(format!(
                    "signature type {} is invalid: {}",
                    signature.signature_type,
                    KeysError::ErrSignUnsupportedSignatureType
                )));
            }
        } {
            Err(KeysError::ErrVerifyFailed)?;
        }

        Ok(())
    }
}
