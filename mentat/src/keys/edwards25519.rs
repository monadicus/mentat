use ed25519_dalek::{Keypair, Signature, Signer, Verifier, SECRET_KEY_LENGTH, SIGNATURE_LENGTH};

pub struct Ed25519Keys {
    keypair: Keypair,
}

impl Keys for Ed25519Keys {
    fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError> {
        if bytes.len() != SECRET_KEY_LENGTH {
            return Err(KeysError::InvalidPrivateKeyBytes);
        }

        let keypair = Keypair::from_bytes(bytes).map_err(|_| KeysError::InvalidPrivateKeyBytes)?;
        Ok(Self { keypair })
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, KeysError> {
        self.keypair
            .sign(message)
            .map_err(|e| KeysError::SignatureFailed(format!("{:?}", e)))
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, KeysError> {
        if signature.len() != SIGNATURE_LENGTH {
            return Err(KeysError::InvalidSignatureBytes);
        }

        let sig = Signature::from_bytes(signature).map_err(|_| KeysError::InvalidSignatureBytes)?;

        self.keypair
            .verify(message, sig)
            .map_err(|_| KeysError::InvalidSignature)
    }
}
