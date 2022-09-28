//! Errors for the validations in this module.

use thiserror::Error;

/// Account Balance Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum AccountBalanceError {
    #[error("currency is used multiple times")]
    CurrencyUsedMultipleTimes,
    #[error("request block hash does not match response block hash")]
    ReturnedBlockHashMismatch,
    #[error("request block index does not match response block index")]
    ReturnedBlockIndexMismatch,
}

/// Block Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum BlockError {
    #[error("Amount.Value is missing")]
    AmountValueMissing,
    #[error("Amount.Value is not an integer")]
    AmountIsNotInt,
    #[error("Amount.Currency is nil")]
    AmountCurrencyIsNil,
    #[error("Amount.Currency.Symbol is empty")]
    AmountCurrencySymbolEmpty,
    #[error("Amount.Currency.Decimals must be >= 0")]
    AmountCurrencyHasNegDecimals,
    #[error("Operation.OperationIdentifier.Index is invalid")]
    OperationIdentifierIndexIsNil,
    #[error("Operation.OperationIdentifier.Index is out of order")]
    OperationIdentifierIndexOutOfOrder,
    #[error("Operation.OperationIdentifier.NetworkIndex is invalid")]
    OperationIdentifierNetworkIndexInvalid,
    #[error("Account is nil")]
    AccountIsNil,
    #[error("Account.Address is missing")]
    AccountAddrMissing,
    #[error("Account.SubAccount.Address is missing")]
    AccountSubAccountAddrMissing,
    #[error("Operation.Status is missing")]
    OperationStatusMissing,
    #[error("Operation.Status is invalid")]
    OperationStatusInvalid,
    #[error("Operation.Type is invalid")]
    OperationTypeInvalid,
    #[error("Operation is nil")]
    OperationIsNil,
    #[error("Operation.Status must be empty for construction")]
    OperationStatusNotEmptyForConstruction,
    #[error("related operation has index greater than operation")]
    RelatedOperationIndexOutOfOrder,
    #[error("found duplicate related operation index")]
    RelatedOperationIndexDuplicate,
    #[error("related operations key is missing")]
    RelatedOperationMissing,
    #[error("fee operation shouldn't have related_operations")]
    RelatedOperationInFeeNotAllowed,
    #[error("BlockIdentifier is nil")]
    BlockIdentifierIsNil,
    #[error("BlockIdentifier.Hash is missing")]
    BlockIdentifierHashMissing,
    #[error("BlockIdentifier.Index is negative")]
    BlockIdentifierIndexIsNeg,
    #[error("PartialBlockIdentifier is nil")]
    PartialBlockIdentifierIsNil,
    #[error("neither PartialBlockIdentifier.Hash nor PartialBlockIdentifier.Index is set")]
    PartialBlockIdentifierFieldsNotSet,
    #[error("TransactionIdentifier is nil")]
    TxIdentifierIsNil,
    #[error("TransactionIdentifier.Hash is missing")]
    TxIdentifierHashMissing,
    #[error("operations cannot be empty for construction")]
    NoOperationsForConstruction,
    #[error("Transaction is nil")]
    TxIsNil,
    #[error("timestamp is before 01/01/2000")]
    TimestampBeforeMin,
    #[error("timestamp is after 01/01/2040")]
    TimestampAfterMax,
    #[error("Block is nil")]
    BlockIsNil,
    #[error("BlockIdentifier.Hash == ParentBlockIdentifier.Hash")]
    BlockHashEqualsParentBlockHash,
    #[error("BlockIdentifier.Index <= ParentBlockIdentifier.Index")]
    BlockIndexPrecedesParentBlockIndex,
    #[error("invalid direction (must be 'forward' or 'backward')")]
    InvalidDirection,
    #[error("duplicate related transaction")]
    DuplicateRelatedTransaction,
    #[error("payment amount doesn't balance")]
    PaymentAmountNotBalancing,
    #[error("fee amount doesn't balance")]
    FeeAmountNotBalancing,
    #[error("payment count doesn't match")]
    PaymentCountMismatch,
    #[error("fee count doesn't match")]
    FeeCountMismatch,
    #[error("fee amount is not negative")]
    FeeAmountNotNegative,
}

// Coin Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum CoinError {
    #[error("coin cannot be nil")]
    IsNil,
    #[error("duplicate coin identifier detected")]
    Duplicate,
    #[error("coin identifier cannot be nil")]
    IdentifierIsNil,
    #[error("coin identifier cannot be empty")]
    IdentifierNotSet,
    #[error("coin change cannot be nil")]
    ChangeIsNil,
    #[error("not a valid coin action")]
    ActionInvalid,
}

/// Construction Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum ConstructionError {
    #[error("ConstructionPreprocessResponse cannot be nil")]
    ConstructionPreprocessResponseIsNil,
    #[error("ConstructionMetadataResponse cannot be nil")]
    ConstructionMetadataResponseIsNil,
    #[error("Metadata is nil")]
    ConstructionMetadataResponseMetadataMissing,
    #[error("TransactionIdentifierResponse cannot be nil")]
    TxIdentifierResponseIsNil,
    #[error("construction combine response cannot be nil")]
    ConstructionCombineResponseIsNil,
    #[error("signed transaction cannot be empty")]
    SignedTxEmpty,
    #[error("construction derive response cannot be nil")]
    ConstructionDeriveResponseIsNil,
    #[error("construction parse response cannot be nil")]
    ConstructionParseResponseIsNil,
    #[error("operations cannot be empty")]
    ConstructionParseResponseOperationsEmpty,
    #[error("signers cannot be empty on signed transaction")]
    ConstructionParseResponseSignersEmptyOnSignedTx,
    #[error("signers should be empty for unsigned txs")]
    ConstructionParseResponseSignersNonEmptyOnUnsignedTx,
    #[error("construction payloads response cannot be nil")]
    ConstructionPayloadsResponseIsNil,
    #[error("unsigned transaction cannot be empty")]
    ConstructionPayloadsResponseUnsignedTxEmpty,
    #[error("signing payloads cannot be empty")]
    ConstructionPayloadsResponsePayloadsEmpty,
    #[error("PublicKey cannot be nil")]
    PublicKeyIsNil,
    #[error("public key bytes cannot be empty")]
    PublicKeyBytesEmpty,
    #[error("public key bytes 0")]
    PublicKeyBytesZero,
    #[error("not a supported CurveType")]
    CurveTypeNotSupported,
    #[error("signing payload cannot be nil")]
    SigningPayloadIsNil,
    #[error("signing payload bytes cannot be empty")]
    SigningPayloadBytesEmpty,
    #[error("signing payload bytes cannot be 0")]
    SigningPayloadBytesZero,
    #[error("signatures cannot be empty")]
    SignaturesEmpty,
    #[error("requested signature type does not match returned signature type")]
    SignaturesReturnedSigMismatch,
    #[error("signature bytes cannot be empty")]
    SignatureBytesEmpty,
    #[error("signature bytes cannot be 0")]
    SignatureBytesZero,
    #[error("not a supported SignatureType")]
    SignatureTypeNotSupported,
}

/// Network Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum NetworkError {
    #[error("NetworkIdentifier is nil")]
    SubNetworkIdentifierInvalid,
    #[error("NetworkIdentifier is nil")]
    NetworkIdentifierIsNil,
    #[error("NetworkIdentifier.Blockchain is missing")]
    NetworkIdentifierBlockchainMissing,
    #[error("NetworkIdentifier.Network is missing")]
    NetworkIdentifierNetworkMissing,
    #[error("Peer.PeerID is missing")]
    PeerIDMissing,
    #[error("version is nil")]
    VersionIsNil,
    #[error("Version.NodeVersion is missing")]
    VersionNodeVersionMissing,
    #[error("Version.MiddlewareVersion is missing")]
    VersionMiddlewareVersionMissing,
    #[error("network status response is nil")]
    NetworkStatusResponseIsNil,
    #[error("no Allow.OperationStatuses found")]
    NoAllowedOperationStatuses,
    #[error("no successful Allow.OperationStatuses found")]
    NoSuccessfulAllowedOperationStatuses,
    #[error("error code used multiple times")]
    ErrorCodeUsedMultipleTimes,
    #[error("error details populated in /network/options")]
    ErrorDetailsPopulated,
    #[error("Allow is nil")]
    AllowIsNil,
    #[error("options is nil")]
    NetworkOptionsResponseIsNil,
    #[error("NetworkListResponse is nil")]
    NetworkListResponseIsNil,
    #[error("NetworkListResponse.Networks contains duplicates")]
    NetworkListResponseNetworksContainsDuplicates,
    #[error("BalanceExemption is nil")]
    BalanceExemptionIsNil,
    #[error("BalanceExemption.Type is invalid")]
    BalanceExemptionTypeInvalid,
    #[error("BalanceExemption missing subject")]
    BalanceExemptionMissingSubject,
    #[error("BalanceExemption.SubAccountAddress is empty")]
    BalanceExemptionSubAccountAddressEmpty,
    #[error("BalanceExemptions only supported when HistoricalBalanceLookup supported")]
    BalanceExemptionNoHistoricalLookup,
    #[error("TimestampStartIndex is invalid")]
    TimestampStartIndexInvalid,
    #[error("SyncStatus.CurrentIndex is negative")]
    SyncStatusCurrentIndexNegative,
    #[error("SyncStatus.TargetIndex is negative")]
    SyncStatusTargetIndexNegative,
    #[error("SyncStatus.Stage is invalid")]
    SyncStatusStageInvalid,
}

/// Server Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum ServerError {
    #[error("no supported networks")]
    NoSupportedNetworks,
    #[error("supported network duplicate")]
    SupportedNetworksDuplicate,
    #[error("requestNetwork is not supported")]
    RequestedNetworkNotSupported,
    #[error("AccountBalanceRequest is nil")]
    AccountBalanceRequestIsNil,
    #[error("historical balance lookup is not supported")]
    AccountBalanceRequestHistoricalBalanceLookupNotSupported,
    #[error("BlockRequest is nil")]
    BlockRequestIsNil,
    #[error("BlockTransactionRequest is nil")]
    BlockTransactionRequestIsNil,
    #[error("ConstructionMetadataRequest is nil")]
    ConstructionMetadataRequestIsNil,
    #[error("ConstructionSubmitRequest is nil")]
    ConstructionSubmitRequestIsNil,
    #[error("ConstructionSubmitRequest.SignedTransaction is empty")]
    ConstructionSubmitRequestSignedTxEmpty,
    #[error("MempoolTransactionRequest is nil")]
    MempoolTransactionRequestIsNil,
    #[error("MetadataRequest is nil")]
    MetadataRequestIsNil,
    #[error("NetworkRequest is nil")]
    NetworkRequestIsNil,
    #[error("ConstructionDeriveRequest is nil")]
    ConstructionDeriveRequestIsNil,
    #[error("ConstructionPreprocessRequest is nil")]
    ConstructionPreprocessRequestIsNil,
    #[error("suggested fee multiplier cannot be less than 0")]
    ConstructionPreprocessRequestSuggestedFeeMultiplierIsNeg,
    #[error("ConstructionPayloadsRequest is nil")]
    ConstructionPayloadsRequestIsNil,
    #[error("ConstructionCombineRequest is nil")]
    ConstructionCombineRequestIsNil,
    #[error("UnsignedTransaction cannot be empty")]
    ConstructionCombineRequestUnsignedTxEmpty,
    #[error("ConstructionHashRequest is nil")]
    ConstructionHashRequestIsNil,
    #[error("SignedTransaction cannot be empty")]
    ConstructionHashRequestSignedTxEmpty,
    #[error("ConstructionParseRequest is nil")]
    ConstructionParseRequestIsNil,
    #[error("Transaction cannot be empty")]
    ConstructionParseRequestEmpty,
    #[error("CallRequest is nil")]
    CallRequestIsNil,
    #[error("call method cannot be empty")]
    CallMethodEmpty,
    #[error("call method is not supported")]
    CallMethodUnsupported,
    #[error("duplicate call method detected")]
    CallMethodDuplicate,
    #[error("AccountCoinsRequest is nil")]
    AccountCoinsRequestIsNil,
    #[error("mempool coins not supported")]
    MempoolCoinsNotSupported,
    #[error("EventsBlocksRequest is nil")]
    EventsBlocksRequestIsNil,
    #[error("offset is negative")]
    OffsetIsNegative,
    #[error("limit is negative")]
    LimitIsNegative,
    #[error("SearchTransactionsRequest is nil")]
    SearchTransactionsRequestIsNil,
    #[error("operator is invalid")]
    OperatorInvalid,
    #[error("max block invalid")]
    MaxBlockInvalid,
    #[error("duplicate currency")]
    DuplicateCurrency,
}

/// Event Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum EventError {
    #[error("max sequence invalid")]
    MaxSequenceInvalid,
    #[error("sequence invalid")]
    SequenceInvalid,
    #[error("block event type invalid")]
    BlockEventTypeInvalid,
    #[error("sequence out of order")]
    SequenceOutOfOrder,
}

/// Search Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum SearchError {
    #[error("next offset invalid")]
    NextOffsetInvalid,
    #[error("total count invalid")]
    TotalCountInvalid,
}

/// Error Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum ErrorError {
    #[error("Error is nil")]
    IsNil,
    #[error("Error.Code is negative")]
    CodeIsNeg,
    #[error("Error.Message is missing")]
    MessageMissing,
    #[error("Error.Code unexpected")]
    UnexpectedCode,
    #[error("Error.Message does not match message from /network/options")]
    MessageMismatch,
    #[error("Error.Retriable mismatch")]
    RetriableMismatch,
    #[error("Error.Description is provided but is empty")]
    DescriptionEmpty,
}

/// Util errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum UtilError {
    #[error("string array is empty")]
    StringArrayEmpty,
    #[error("empty string exists in string array")]
    StringArrayEmptyString,
    #[error("duplicate string exists in string array")]
    StringArrayDuplicateString,
    #[error("account array is empty")]
    AccountArrayEmpty,
    #[error("invalid account exists in account array")]
    AccountArrayInvalidAccount,
    #[error("duplicate account exists in account array")]
    AccountArrayDuplicateAccount,
}

/// Asserter Errors
#[derive(Debug, Error, Eq, PartialEq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum AsserterError {
    // AsserterNotInitialized is returned when some call in the asserter
    // package requires the asserter to be initialized first.
    #[error("asserter not initialized")]
    NotInitialized,
    #[error(transparent)]
    AccountBalance(#[from] AccountBalanceError),
    #[error(transparent)]
    Block(#[from] BlockError),
    #[error(transparent)]
    Coin(#[from] CoinError),
    #[error(transparent)]
    Construction(#[from] ConstructionError),
    #[error(transparent)]
    Network(#[from] NetworkError),
    #[error(transparent)]
    Server(#[from] ServerError),
    #[error(transparent)]
    Event(#[from] EventError),
    #[error(transparent)]
    Search(#[from] SearchError),
    #[error(transparent)]
    Error(#[from] ErrorError),
    #[error(transparent)]
    Util(#[from] UtilError),
    #[error("{0}")]
    StringError(String),
}

impl From<String> for AsserterError {
    fn from(s: String) -> Self {
        Self::StringError(s)
    }
}

impl From<&str> for AsserterError {
    fn from(s: &str) -> Self {
        Self::StringError(s.into())
    }
}

/// The result type for any Asserter module errors.
pub type AssertResult<T, E = AsserterError> = std::result::Result<T, E>;

/// `err` takes an error as an argument and returns
/// whether or not the error is one thrown by the asserter
/// along with the specific source of the error
pub fn err(err: Box<dyn std::error::Error>) -> (bool, &'static str) {
    if err.is::<AccountBalanceError>() {
        (true, "account balance error")
    } else if err.is::<BlockError>() {
        (true, "block error")
    } else if err.is::<CoinError>() {
        (true, "coin error")
    } else if err.is::<ConstructionError>() {
        (true, "construction error")
    } else if err.is::<ErrorError>() {
        (true, "error error")
    } else if err.is::<EventError>() {
        (true, "evet error")
    } else if err.is::<NetworkError>() {
        (true, "network error")
    } else if err.is::<SearchError>() {
        (true, "search error")
    } else if err.is::<ServerError>() {
        (true, "server error")
    } else if err.is::<UtilError>() {
        (true, "util error")
    } else {
        (false, "")
    }
}
