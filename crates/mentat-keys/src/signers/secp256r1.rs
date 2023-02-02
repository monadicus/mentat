use mentat_types::{
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
    UncheckedSignature,
    UncheckedSignatureType,
};
use p256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature as PSignature,
        SigningKey,
        VerifyingKey,
    },
    elliptic_curve::generic_array::GenericArray,
};

use super::*;

/// `SignerSecp256r1` is initialized from a `UncheckedKeyPair`.
pub struct SignerSecp256r1 {
    pub key_pair: UncheckedKeyPair,
}

// The Ecdsa signature is the couple (R, S), both R and S are 32 bytes
const ECDSA_R_LEN: usize = 32;
const _ECDSA_S_LEN: usize = 32;
const ECDSA_MSG_LEN: usize = 32;

impl SignerInterface for SignerSecp256r1 {
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

        if !matches!(sig_type, SignatureType::Ecdsa) {
            Err(format!(
                "expected signature type {} but got {sig_type}: {}",
                SignatureType::Ecdsa,
                KeysError::ErrSignUnsupportedSignatureType
            ))?;
        }

        let private_key = SigningKey::from_bytes(&self.key_pair.private_key)
            .map_err(|_| KeysError::ErrPubKeyNotOnCurve)?;
        // TODO no error for failed to sign?
        let sig: PSignature = private_key.sign(&payload.bytes);
        Ok(Signature {
            signature_type: payload.signature_type,
            signing_payload: payload,
            public_key: valid_key_pair.public_key,
            bytes: sig.to_bytes().as_slice().to_vec(),
        })
    }

    fn verify(&self, signature: UncheckedSignature) -> KeysResult<()> {
        if signature.signature_type != UncheckedSignatureType::ECDSA.into() {
            Err(format!(
                "expected signing payload signature type {} but got {}: {}",
                SignatureType::Ecdsa,
                signature.signature_type,
                KeysError::ErrVerifyUnsupportedSignatureType
            ))?;
        }
        // dbg!("foo");

        mentat_asserter::signatures(&[Some(&signature)])
            .map_err(|err| format!("signature is invalid: {err}"))?;
        let signature: Signature = signature.into();
        // dbg!("bar");

        if signature.signing_payload.bytes.len() != ECDSA_MSG_LEN {
            return Err(KeysError::ErrVerifyFailed);
        }
        // dbg!("sex");

        let public_key = VerifyingKey::from_sec1_bytes(&signature.public_key.bytes)
            .map_err(|_| KeysError::ErrPubKeyNotOnCurve)?;
        // dbg!("pk");
        let message = signature.signing_payload.bytes;
        let mut signature = signature.bytes;
        signature.resize(64, 0);
        let r = GenericArray::from_slice(&signature[0..ECDSA_R_LEN]);
        // dbg!("r", r);
        let s = GenericArray::from_slice(&signature[ECDSA_R_LEN..]);
        // dbg!("s", s);
        let sig: PSignature =
            PSignature::from_scalars(*r, *s).map_err(|_| KeysError::ErrVerifyFailed)?;
        // dbg!(&sig);
        public_key
            .verify(&message, &sig)
            .map_err(|_| KeysError::ErrVerifyFailed)?;

        Ok(())
    }
}
