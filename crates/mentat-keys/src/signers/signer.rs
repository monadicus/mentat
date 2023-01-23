use mentat_types::{PublicKey, Signature, SignatureType, SigningPayload, UncheckedSignature};

use crate::errors::KeysResult;

/// `Signer` is an interface for different curve signers
pub trait Signer {
    fn public_key(&self) -> PublicKey;
    fn sign(&self, payload: SigningPayload, sig_type: SignatureType) -> KeysResult<Signature>;
    fn verify(&self, signature: UncheckedSignature) -> KeysResult<()>;
}
