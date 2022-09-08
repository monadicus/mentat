//! Error types for Syncer errors

use mentat_utils::rust_utils::ContextResult;
use thiserror::Error;

/// Error types for Syncer errors
#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum SyncerError {
    /// ErrCannotRemoveGenesisBlock is returned when
    /// a Rosetta implementation indicates that the
    /// genesis block should be orphaned.
    #[error("cannot remove genesis block")]
    CannotRemoveGenesisBlock,
    /// ErrOrphanHead is returned by the Helper when
    /// the current head should be orphaned. In some
    /// cases, it may not be possible to populate a block
    /// if the head of the canonical chain is not yet synced.
    #[error("orphan head")]
    OrphanHead,
    /// ErrBlockResultNil is returned by the syncer
    /// when attempting to process a block and the block
    /// result is nil.
    #[error("block result is nil")]
    BlockResultNil,
    /// ErrGetCurrentHeadBlockFailed is returned by the syncer when
    /// the current head block index is not able to get
    #[error("unable to get current head")]
    GetCurrentHeadBlockFailed,
    /// ErrOutOfOrder is returned when the syncer examines
    /// a block that is out of order. This typically
    /// means the Helper has a bug.
    #[error("out of order")]
    OutOfOrder,
    /// Cancelled is returned when a syncer thread was told to terminate early
    #[error("cancelled")]
    Canceled,
    /// DeadlineExceeded is returned when the syncer ran for longer than allowed
    #[error("deadline exceeded")]
    DeadlineExceeded,
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

impl From<ContextResult> for SyncerError {
    fn from(e: ContextResult) -> Self {
        match e {
            ContextResult::Canceled => SyncerError::Canceled,
            ContextResult::DeadlineExceeded => SyncerError::DeadlineExceeded,
        }
    }
}

/// The syncer module result type.
pub type SyncerResult<T, E = SyncerError> = Result<T, E>;

/// Err takes an error as an argument and returns
/// whether or not the error is one thrown by the syncer package
#[cfg(test)]
pub fn err(err: Box<dyn std::error::Error>) -> bool {
    err.is::<SyncerError>()
}
