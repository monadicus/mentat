use thiserror::Error;

/// Possible errors that can be encountered when working with a keypair.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum KeysError {
    #[error("could not decode privkey")]
    ErrPrivKeyUndecodable,
    #[error("invalid privkey length")]
    ErrPrivKeyLengthInvalid,
    #[error("privkey cannot be 0")]
    ErrPrivKeyZero,
    #[error("pubkey is not on the curve")]
    ErrPubKeyNotOnCurve,
    #[error("keygen: error generating pair for {0} curve type")]
    ErrKeyGenFailed(String),
    #[error("not a supported CurveType")]
    ErrCurveTypeNotSupported,
    #[error("sign: unxpected payload.SignatureType while signing")]
    ErrSignUnsupportedPayloadSignatureType,
    #[error("sign: unexpected Signature type while signing")]
    ErrSignUnsupportedSignatureType,
    #[error("sign: unable to sign")]
    ErrSignFailed,
    #[error("verify: unxpected payload.SignatureType while verifying")]
    ErrVerifyUnsupportedPayloadSignatureType,
    #[error("verify: unexpected Signature type while verifying")]
    ErrVerifyUnsupportedSignatureType,
    #[error("verify: verify returned false")]
    ErrVerifyFailed,
}
