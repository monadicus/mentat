use snarkvm_algorithms::SignatureScheme;
use snarkvm_dpc::Network;
use snarkvm_utilities::{FromBytes, ToBytes};

use super::{Keys, KeysError};

const SRS: &str = "aleo_signature";

pub struct AleoKeys<N: Network> {
    priv_key: <<N as Network>::AccountSignatureScheme as SignatureScheme>::PrivateKey,
    pub_key: <<N as Network>::AccountSignatureScheme as SignatureScheme>::PublicKey,
    scheme: <N as Network>::AccountSignatureScheme,
}

impl<N: Network> Keys for AleoKeys<N> {
    fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError> {
        let scheme = <N as Network>::AccountSignatureScheme::setup(SRS);
        let priv_key =
            <<N as Network>::AccountSignatureScheme as SignatureScheme>::PrivateKey::from_bytes_le(
                bytes,
            )
            .map_err(|_| KeysError::InvalidPrivateKeyBytes)?;
        let pub_key = scheme.generate_public_key(&priv_key);

        Ok(Self {
            priv_key,
            pub_key,
            scheme,
        })
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, KeysError> {
        let mut rng = rand::thread_rng();
        self.scheme
            .sign(&self.priv_key, &message, &mut rng)
            .map_err(|e| KeysError::SignatureFailed(format!("{:?}", e)))?
            .to_bytes_le()
            .map_err(|_| KeysError::InvalidSignatureBytes)
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, KeysError> {
        let sig =
            <<N as Network>::AccountSignatureScheme as SignatureScheme>::Signature::from_bytes_le(
                signature,
            )
            .map_err(|_| KeysError::InvalidSignatureBytes)?;
        self.scheme
            .verify(&self.pub_key, message, &sig)
            .map_err(|_| KeysError::InvalidSignature)
    }
}
