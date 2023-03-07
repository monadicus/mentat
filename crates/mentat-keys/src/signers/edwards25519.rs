use ed25519_compact::{KeyPair as EdKeyPair, Seed, Signature as EdSignature};
use mentat_types::{
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
    UncheckedSignature,
    UncheckedSignatureType,
};

use super::*;

/// `SignerEdwards25519` is initialized from a `UncheckedKeyPair`.
pub struct SignerEdwards25519 {
    pub key_pair: UncheckedKeyPair,
}

impl SignerInterface for SignerEdwards25519 {
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

        if !(matches!(
            payload.signature_type,
            SignatureType::Ed25519 | SignatureType::EmptyString
        )) {
            Err(format!(
                "expected signing payload signature type {} but got {}: {}",
                SignatureType::Ed25519,
                payload.signature_type,
                KeysError::ErrSignUnsupportedPayloadSignatureType
            ))?;
        }

        if !matches!(sig_type, SignatureType::Ed25519) {
            Err(format!(
                "expected signature type {} but got {sig_type}: {}",
                SignatureType::Ed25519,
                KeysError::ErrSignUnsupportedSignatureType
            ))?;
        }

        // Safe to unwrap here, private_key size has already been checked.
        let private_key =
            EdKeyPair::from_seed(Seed::from_slice(&valid_key_pair.private_key).unwrap());
        let signature = private_key.sk.sign(&payload.bytes, None);

        Ok(Signature {
            signature_type: payload.signature_type,
            signing_payload: payload,
            public_key: valid_key_pair.public_key,
            bytes: signature.to_vec(),
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

        let pk = ed25519_compact::PublicKey::from_slice(&signature.public_key.bytes)
            .map_err(|_| KeysError::ErrVerifyFailed)?;
        let ed_signature =
            EdSignature::from_slice(&signature.bytes).map_err(|_| KeysError::ErrVerifyFailed)?;
        pk.verify(&signature.signing_payload.bytes, &ed_signature)
            .map_err(|_| KeysError::ErrVerifyFailed)?;

        Ok(())
    }
}
