/// Possible errors that can be encountered when working with a keypair.
#[derive(Clone, Debug)]
pub enum KeysError {
    InvalidPrivateKeyBytes,
    SignatureFailed(String),
    InvalidSignatureBytes,
    InvalidSignature,
}
