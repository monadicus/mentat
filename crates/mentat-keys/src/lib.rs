pub mod aleo;
pub mod edwards25519;
mod errors;
pub mod pallas;
pub mod secp256k1;
pub mod secp256r1;
use errors::KeysError;

/// Defines generic operations performed with sets of keys.
pub trait Keys: Sized {
    type M;
    type S;

    /// Import a private key from raw bytes.
    fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError>;

    /// Sign a message.
    fn sign(&self, message: &Self::M) -> Result<Self::S, KeysError>;

    /// Verify a signature.
    fn verify(&self, message: &Self::M, signature: &Self::S) -> Result<bool, KeysError>;
}

#[cfg(test)]
#[path = ""]
mod tests {
    mod errors_test;
    mod keys_test;
}
