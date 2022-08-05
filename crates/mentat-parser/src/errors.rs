//! The errors for the Parser module.

use mentat_asserter::AsserterError;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(clippy::missing_docs_in_private_items)]
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
#[allow(clippy::missing_docs_in_private_items)]
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

/// `ParserError` type.
#[derive(Debug, Error)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum ParserError {
    #[error(transparent)]
    Intent(#[from] IntentError),
    #[error(transparent)]
    MatchOperations(#[from] MatchOperationsError),
    #[error(transparent)]
    Asserter(#[from] AsserterError),
    #[error("{0}")]
    String(String),
}

impl From<String> for ParserError {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for ParserError {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}

/// The parser module result type.
pub type ParserResult<T, E = ParserError> = Result<T, E>;

/// `err` takes an error as an argument and returns
/// whether or not the error is one thrown by the asserter
/// along with the specific source of the error
pub fn err(err: Box<dyn std::error::Error>) -> (bool, &'static str) {
    if err.is::<IntentError>() {
        (true, "account balance error")
    } else if err.is::<MatchOperationsError>() {
        (true, "match error")
    } else {
        (false, "")
    }
}
