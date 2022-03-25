use ed25519_dalek::{Keypair, Signature, Signer, SECRET_KEY_LENGTH};

use super::{Keys, KeysError};

pub struct Ed25519Keys {
    keypair: Keypair,
}

impl Keys for Ed25519Keys {
    type M = Vec<u8>;
    type S = Signature;

    fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError> {
        if bytes.len() != SECRET_KEY_LENGTH {
            return Err(KeysError::InvalidPrivateKeyBytes);
        }

        let keypair = Keypair::from_bytes(bytes).map_err(|_| KeysError::InvalidPrivateKeyBytes)?;
        Ok(Self { keypair })
    }

    fn sign(&self, message: &Self::M) -> Result<Self::S, KeysError> {
        Ok(self.keypair.sign(message))
    }

    fn verify(&self, message: &Self::M, signature: &Self::S) -> Result<bool, KeysError> {
        Ok(self.keypair.verify(message, signature).is_ok())
    }
}
