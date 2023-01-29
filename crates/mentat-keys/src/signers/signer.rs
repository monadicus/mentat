use mentat_types::{PublicKey, Signature, SignatureType, SigningPayload, UncheckedSignature};

use crate::{errors::KeysResult, SignerEdwards25519, SignerPallas, SignerSecp256k1};

/// `SignerInterface` is an interface for different curve signers
pub trait SignerInterface {
    fn public_key(&self) -> PublicKey;
    fn sign(&self, payload: SigningPayload, sig_type: SignatureType) -> KeysResult<Signature>;
    fn verify(&self, signature: UncheckedSignature) -> KeysResult<()>;
}

/// `Signer` enum is an enum of all signer types so we can avoid dynamic
/// dispatch.
pub enum Signer {
    Edwards25519(SignerEdwards25519),
    Pallas(SignerPallas),
    Secp256k1(SignerSecp256k1),
}

impl SignerInterface for Signer {
    fn public_key(&self) -> PublicKey {
        match self {
            Self::Edwards25519(s) => s.public_key(),
            Self::Pallas(s) => s.public_key(),
            Self::Secp256k1(s) => s.public_key(),
        }
    }

    fn sign(&self, payload: SigningPayload, sig_type: SignatureType) -> KeysResult<Signature> {
        match self {
            Self::Edwards25519(s) => s.sign(payload, sig_type),
            Self::Pallas(s) => s.sign(payload, sig_type),
            Self::Secp256k1(s) => s.sign(payload, sig_type),
        }
    }

    fn verify(&self, signature: UncheckedSignature) -> KeysResult<()> {
        match self {
            Self::Edwards25519(s) => s.verify(signature),
            Self::Pallas(s) => s.verify(signature),
            Self::Secp256k1(s) => s.verify(signature),
        }
    }
}
