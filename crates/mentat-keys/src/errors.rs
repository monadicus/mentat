use thiserror::Error;

/// Possible errors that can be encountered when working with a keypair.
#[derive(Debug, Error)]
pub enum KeysError {
    #[error("could not decode privkey")]
    ErrPrivKeyUndecodable,
    #[error("invalid privkey length")]
    ErrPrivKeyLengthInvalid,
    #[error("privkey cannot be 0")]
    ErrPrivKeyZero,
    #[error("pubkey is not on the curve")]
    ErrPubKeyNotOnCurve,

    #[error("not a supported CurveType")]
    ErrCurveTypeNotSupported,

    #[error("sign: unexpected payload.SignatureType while signing")]
    ErrSignUnsupportedPayloadSignatureType,
    #[error("sign: unexpected Signature type while signing")]
    ErrSignUnsupportedSignatureType,

    #[error("verify: unexpected payload.SignatureType while verifying")]
    ErrVerifyUnsupportedPayloadSignatureType,
    #[error("verify: unexpected Signature type while verifying")]
    ErrVerifyUnsupportedSignatureType,
    #[error("verify: verify returned false")]
    ErrVerifyFailed,

    #[error("payment not found in signingPayload")]
    ErrPaymentNotFound,
    // #[error("sign: unable to sign")]
    // ErrSignFailed,
    // #[error("keygen: error generating pair for {0} curve type")]
    // ErrKeyGenFailed(String),
    // #[error(transparent)]
    // AsserterError(#[from] AsserterError),
    #[error("{0}")]
    String(String),
}

impl From<String> for KeysError {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

/// The result type for any Keys module errors.
pub type KeysResult<T, E = KeysError> = std::result::Result<T, E>;
