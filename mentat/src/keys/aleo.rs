use snarkvm_algorithms::SignatureScheme;
use snarkvm_dpc::Network;
use snarkvm_utilities::FromBytes;

use super::{Keys, KeysError};

// TODO: this needs to coincide with the Aleo parameters for each network type.
// I'm assuming these are constant but this needs to be confirmed.
const SRS: &str = "aleo_signature";

pub struct AleoKeys<N: Network> {
    priv_key: <<N as Network>::AccountSignatureScheme as SignatureScheme>::PrivateKey,
    pub_key: <<N as Network>::AccountSignatureScheme as SignatureScheme>::PublicKey,
    scheme: <N as Network>::AccountSignatureScheme,
}

impl<N: Network> Keys for AleoKeys<N> {
    type M = Vec<u8>;
    type S = <<N as Network>::AccountSignatureScheme as SignatureScheme>::Signature;

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

    fn sign(&self, message: &Self::M) -> Result<Self::S, KeysError> {
        let mut rng = rand::thread_rng();
        self.scheme
            .sign(&self.priv_key, message, &mut rng)
            .map_err(|e| KeysError::SignatureFailed(format!("{:?}", e)))
    }

    fn verify(&self, message: &Self::M, signature: &Self::S) -> Result<bool, KeysError> {
        self.scheme
            .verify(&self.pub_key, message, signature)
            .map_err(|_| KeysError::InvalidSignature)
    }
}
