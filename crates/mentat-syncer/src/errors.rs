//! Error types for Syncer errors

use thiserror::Error;

/// Error types for Syncer errors
#[derive(Debug, Error, PartialEq, Eq)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum SyncerError {
    /// ErrCannotRemoveGenesisBlock is returned when
    /// a Rosetta implementation indicates that the
    /// genesis block should be orphaned.
    #[error("cannot remove genesis block")]
    CannotRemoveGenesisBlock,
    /// ErrOutOfOrder is returned when the syncer examines
    /// a block that is out of order. This typically
    /// means the Helper has a bug.
    #[error("out of order")]
    OutOfOrder,
    /// ErrOrphanHead is returned by the Helper when
    /// the current head should be orphaned. In some
    /// cases, it may not be possible to populate a block
    /// if the head of the canonical chain is not yet synced.
    #[error("orphan head")]
    OrphanedHead,
    /// ErrBlockResultNil is returned by the syncer
    /// when attempting to process a block and the block
    /// result is nil.
    #[error("block result is nil")]
    BlockResultNil,
    #[error("unable to get current head")]
    GetCurrentHeadBlockFailed,
    #[error("unable to get network status")]
    GetNetworkStatusFailed,
    #[error("unable to fetch block")]
    FetchBlockFailed,
    #[error("unable to fetch block during re-org")]
    FetchBlockReorgFailed,
    #[error("unable to process block")]
    BlockProcessFailed,
    #[error("unable to process blocks")]
    BlocksProcessMultipleFailed,
    #[error("unable to set start index")]
    SetStartIndexFailed,
    #[error("unable to get next syncable range")]
    NextSyncableRangeFailed,
    #[error("{0}")]
    String(String),
}

impl From<String> for SyncerError {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for SyncerError {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}

/// The syncer module result type.
pub type SyncerResult<T, E = SyncerError> = Result<T, E>;

/// Err takes an error as an argument and returns
/// whether or not the error is one thrown by the syncer package
pub fn err(err: Box<dyn std::error::Error>) -> bool {
    err.is::<SyncerError>()
}
