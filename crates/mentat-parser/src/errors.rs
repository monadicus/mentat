use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntentError {
    #[error("intended account did not match observed account")]
    ExpectedOperationAccountMismatch,
    #[error("intended amount did not match observed amount")]
    ExpectedOperationAmountMismatch,
    #[error("intended type did not match observed type")]
    ExpectedOperationTypeMismatch,
    #[error("found extra operation")]
    ExpectedOperationsExtraOperation,
    #[error("found unexpected signers")]
    ExpectedSignerUnexpectedSigner,
    #[error("missing expected signer")]
    ExpectedSignerMissing,
}

#[derive(Debug, Error)]
pub enum MatchOperationsError {
    #[error("account is missing")]
    AccountMatchAccountMissing,
    #[error("SubAccountIdentifier.Address is missing")]
    AccountMatchSubAccountMissing,
    #[error("SubAccount is populated")]
    AccountMatchSubAccountPopulated,
    #[error("unexpected SubAccountIdentifier.Address")]
    AccountMatchUnexpectedSubAccountAddr,
    #[error("key is not present in metadata")]
    MetadataMatchKeyNotFound,
    #[error("unexpected value associated with key")]
    MetadataMatchKeyValueMismatch,
    #[error("amount is missing")]
    AmountMatchAmountMissing,
    #[error("amount is populated")]
    AmountMatchAmountPopulated,
    #[error("unexpected amount sign")]
    AmountMatchUnexpectedSign,
    #[error("unexpected currency")]
    AmountMatchUnexpectedCurrency,
    #[error("coin change is nil")]
    CoinActionMatchCoinChangeIsNil,
    #[error("unexpected coin action")]
    CoinActionMatchUnexpectedCoinAction,
    #[error("cannot check equality of 0 operations")]
    EqualAmountsNoOperations,
    #[error("amounts are not equal")]
    EqualAmountsNotEqual,
    #[error("operations have the same sign")]
    OppositeAmountsSameSign,
    #[error("operation absolute values are not equal")]
    OppositeAmountsAbsValMismatch,
    #[error("cannot check equality of <= 1 operations")]
    EqualAddressesTooFewOperations,
    #[error("account is nil")]
    EqualAddressesAccountIsNil,
    #[error("addresses do not match")]
    EqualAddressesAddrMismatch,
    #[error("match index out of range")]
    MatchIndexValidIndexOutOfRange,
    #[error("match index is nil")]
    MatchIndexValidIndexIsNil,
    #[error("unable to match anything to 0 operations")]
    MatchOperationsNoOperations,
    #[error("no descriptions to match")]
    MatchOperationsDescriptionsMissing,
    #[error("unable to find match for operation")]
    MatchOperationsMatchNotFound,
    #[error("could not find match for description")]
    MatchOperationsDescriptionNotMatched,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error(transparent)]
    Intent(#[from] IntentError),
    #[error(transparent)]
    MatchOperations(#[from] MatchOperationsError),
}
