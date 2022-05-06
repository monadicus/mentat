use p256::ecdsa::{
    signature::{Signer, Verifier},
    Signature,
    SigningKey,
    VerifyingKey,
};

use super::{Keys, KeysError};

/// An ECDSA keypair, created from the Secp256r1 curve.
pub struct Secp256r1Keys {
    priv_key: SigningKey,
    pub_key: VerifyingKey,
}

impl Keys for Secp256r1Keys {
    type M = Vec<u8>;
    type S = Signature;

    fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError> {
        let priv_key =
            SigningKey::from_bytes(bytes).map_err(|_| KeysError::InvalidPrivateKeyBytes)?;
        let pub_key = VerifyingKey::from(&priv_key);
        Ok(Self { priv_key, pub_key })
    }

    fn sign(&self, message: &Self::M) -> Result<Self::S, KeysError> {
        Ok(self.priv_key.sign(message))
    }

    fn verify(&self, message: &Self::M, signature: &Self::S) -> Result<bool, KeysError> {
        Ok(self.pub_key.verify(message, signature).is_ok())
    }
}
