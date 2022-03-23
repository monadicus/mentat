#[derive(Clone, Debug)]
pub enum KeysError {
    InvalidPrivateKeyBytes,
    SignatureFailed(String),
    InvalidSignatureBytes,
    InvalidSignature,
}
