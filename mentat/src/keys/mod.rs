pub mod aleo;
pub mod edwards25519;
mod errors;
pub mod pallas;
pub mod secp256k1;
pub mod secp256r1;
use errors::KeysError;

/// Defines generic operations performed with sets of keys.
pub trait Keys: Sized {
    /// Import a private key from raw bytes.
    fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError>;

    /// Sign a message.
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, KeysError>;

    /// Verify a signature.
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, KeysError>;
}
