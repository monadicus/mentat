use ed25519_dalek::{Keypair, Signature, Signer, KEYPAIR_LENGTH};

use super::{Keys, KeysError};

/// A pair of keys used for signing and verifying messages in the Ed25519
/// signature scheme.
pub struct Ed25519Keys {
    keypair: Keypair,
}

impl Keys for Ed25519Keys {
    type M = Vec<u8>;
    type S = Signature;

    fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError> {
        if bytes.len() != KEYPAIR_LENGTH {
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

#[cfg(test)]
mod tests {
    use rand_old::{rngs::OsRng, CryptoRng, Rng, RngCore};

    use super::*;

    #[test]
    fn import() {
        let keypair = Keypair::generate(&mut OsRng {});
        let bytes = keypair.to_bytes();
        assert!(Ed25519Keys::import_private_key(&bytes).is_ok());
    }

    #[test]
    fn sign_verify() {
        let keypair = Keypair::generate(&mut OsRng {});
        let bytes = keypair.to_bytes();
        let keys = Ed25519Keys::import_private_key(&bytes).unwrap();
        let message = (0..32)
            .map(|_| rand_old::thread_rng().gen::<u8>())
            .collect::<Vec<u8>>();
        let sig = keys.sign(&message).unwrap();
        assert!(keys.verify(&message, &sig).unwrap());
    }
}
