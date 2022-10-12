#[cfg(test)]
mod errors_test;

use thiserror::Error;

/// Badger Storage Errors
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum BadgerStorageError {
    #[error("unable to close database")]
    DBCloseFailed,
    #[error("unable to get value for key")]
    ScanGetValueFailed,
    #[error("worker failed")]
    ScanWorkerFailed,
    #[error("decompressed dictionary output does not match")]
    DecompressOutputMismatch,
    #[error("max entries reached")]
    MaxEntries,
    #[error("unable to scan")]
    ScanFailed,
    #[error("found 0 entries for namespace")]
    NoEntriesFoundInNamespace,
}

/// Broadcast Storage Errors
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum BroadcastStorageError {
    #[error("already broadcasting transaction")]
    BroadcastAlreadyExists,
    #[error("unable to commit broadcast update")]
    BroadcastCommitUpdateFailed,
    #[error("unexpected transaction hash returned by broadcast")]
    BroadcastIdentifierMismatch,
}

/// Coin Storage Errors
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum CoinStorageError {
    #[error("duplicate coin found")]
    DuplicateCoinFound,
    #[error("unable to parse amount for coin")]
    CoinParseFailed,
    #[error("coin not found")]
    CoinNotFound,
}

/// Compressor Errors
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum CompressorError {
    #[error("unable to close writer")]
    WriterCloseFailed,
    #[error("unable to decode object")]
    ObjectDecodeFailed,
    #[error("unable to copy block")]
    CopyBlockFailed,
    #[error("unable to decode raw bytes")]
    RawDecodeFailed,
}

/// Job Storage Errors
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum JobStorageError {
    #[error("identifier not found")]
    JobIdentifierNotFound,
    #[error("unable to update terminal job")]
    JobUpdateOldFailed,
    #[error("job does not exist")]
    JobDoesNotExist,
}

/// Key Storage Errors
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum KeyStorageError {
    /// ErrAddrExists is returned when key storage already
    /// contains an address.
    #[error("key already exists")]
    AddrExists,
    #[error("address not found")]
    AddrNotFound,
    #[error("unable to parse key pair")]
    ParseKeyPairFailed,
    #[error("cannot determine signature type for payload")]
    DetermineSigTypeFailed,
    #[error("no addresses available")]
    NoAddrAvailable,
}

/// Balance Storage Errors
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum BalanceStorageError {
    /// ErrNegativeBalance is returned when an account
    /// balance goes negative as the result of an operation.
    #[error("negative balance")]
    NegativeBalance,

    /// ErrInvalidLiveBalance is returned when an account's
    /// live balance varies in a way that is inconsistent
    /// with any balance exemption.
    #[error("invalid live balance")]
    InvalidLiveBalance,

    /// ErrBalancePruned is returned when the caller attempts
    /// to retrieve a pruned balance.
    #[error("balance pruned")]
    BalancePruned,

    /// ErrBlockNil is returned when the block to lookup
    /// a balance at is nil.
    #[error("block nil")]
    BlockNil,

    /// ErrAccountMissing is returned when a fetched
    /// account does not exist.
    #[error("account missing")]
    AccountMissing,

    /// ErrInvalidChangeValue is returned when the change value
    /// cannot be parsed.
    #[error("invalid change value")]
    InvalidChangeValue,

    /// ErrInvalidValue is returned when the value we are trying
    /// to save cannot be parsed.
    #[error("invalid value")]
    InvalidValue,

    #[error("balance storage helper or handler is missing")]
    HelperHandlerMissing,

    #[error("invalid currency")]
    InvalidCurrency,
}

/// Block Storage Errors
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum BlockStorageError {
    /// ErrHeadBlockNotFound is returned when there is no
    /// head block found in BlockStorage.
    #[error("head block not found")]
    HeadBlockNotFound,

    /// ErrBlockNotFound is returned when a block is not
    /// found in BlockStorage.
    #[error("block not found")]
    BlockNotFound,

    /// ErrDuplicateKey is returned when a key
    /// cannot be stored because it is a duplicate.
    #[error("duplicate key")]
    DuplicateKey,

    /// ErrDuplicateTransactionHash is returned when a transaction
    /// hash cannot be stored because it is a duplicate.
    #[error("duplicate transaction hash")]
    DuplicateTransactionHash,

    #[error("last processed block is less than start index")]
    LastProcessedBlockPrecedesStart,

    #[error("could not decode transaction hash contents")]
    TransactionHashContentsDecodeFailed,

    #[error("could not remove transaction")]
    TransactionDeleteFailed,

    #[error("saved blocks at transaction does not contain transaction hash")]
    TransactionHashNotFound,

    #[error("unable to decode block data for transaction")]
    BlockDataDecodeFailed,

    #[error("unable to find transaction")]
    TransactionNotFound,

    #[error("transaction does not exist in block")]
    TransactionDoesNotExistInBlock,

    #[error("oldest index missing")]
    OldestIndexMissing,

    #[error("cannot remove oldest index")]
    CannotRemoveOldest,

    #[error("cannot access pruned data")]
    CannotAccessPrunedData,

    #[error("nothing to prune")]
    NothingToPrune,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum StorageError {
    #[error(transparent)]
    Balance(#[from] BalanceStorageError),
    #[error(transparent)]
    Block(#[from] BlockStorageError),
    #[error(transparent)]
    Coin(#[from] CoinStorageError),
    #[error(transparent)]
    Key(#[from] KeyStorageError),
    #[error(transparent)]
    Badger(#[from] BadgerStorageError),
    #[error(transparent)]
    Compressor(#[from] CompressorError),
    #[error(transparent)]
    Job(#[from] JobStorageError),
    #[error(transparent)]
    Broadcast(#[from] BroadcastStorageError),
    #[error("{0}")]
    StringError(String),
}

impl From<String> for StorageError {
    fn from(s: String) -> Self {
        Self::StringError(s)
    }
}

impl From<&str> for StorageError {
    fn from(s: &str) -> Self {
        Self::StringError(s.into())
    }
}

pub type StorageResult<T> = Result<T, StorageError>;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageErrorType {
    Balance,
    Block,
    Coin,
    Key,
    Badger,
    Compressor,
    Job,
    Broadcast,
    String,
}

#[cfg(test)]
impl StorageErrorType {
    /// takes an error as an argument and returns
    /// whether or not the error is one thrown by the storage
    /// along with the specific source of the error
    pub fn check(err: Box<dyn std::error::Error>) -> Option<StorageErrorType> {
        match err {
            e if e.is::<BalanceStorageError>() => Some(StorageErrorType::Balance),
            e if e.is::<BlockStorageError>() => Some(StorageErrorType::Block),
            e if e.is::<CoinStorageError>() => Some(StorageErrorType::Coin),
            e if e.is::<KeyStorageError>() => Some(StorageErrorType::Key),
            e if e.is::<BadgerStorageError>() => Some(StorageErrorType::Badger),
            e if e.is::<CompressorError>() => Some(StorageErrorType::Compressor),
            e if e.is::<JobStorageError>() => Some(StorageErrorType::Job),
            e if e.is::<BroadcastStorageError>() => Some(StorageErrorType::Broadcast),
            e if e.is::<StorageError>() => match *e.downcast::<StorageError>().unwrap() {
                StorageError::Balance(_) => Some(StorageErrorType::Balance),
                StorageError::Block(_) => Some(StorageErrorType::Block),
                StorageError::Coin(_) => Some(StorageErrorType::Coin),
                StorageError::Key(_) => Some(StorageErrorType::Key),
                StorageError::Badger(_) => Some(StorageErrorType::Badger),
                StorageError::Compressor(_) => Some(StorageErrorType::Compressor),
                StorageError::Job(_) => Some(StorageErrorType::Job),
                StorageError::Broadcast(_) => Some(StorageErrorType::Broadcast),
                StorageError::StringError(_) => Some(StorageErrorType::String),
            },
            _ => None,
        }
    }
}
