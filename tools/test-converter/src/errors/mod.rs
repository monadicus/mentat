mod input_file_error;
pub use input_file_error::*;
use mentat_tokenizer::{Error as TokenizeError, Report};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("{0:?}")]
    RulesError(Report<RulesFileError>),
    #[error("{0:?}")]
    TokenizerError(TokenizeError),
}

impl From<Report<RulesFileError>> for Error {
    fn from(r: Report<RulesFileError>) -> Self {
        Self::RulesError(r)
    }
}

impl From<TokenizeError> for Error {
    fn from(r: TokenizeError) -> Self {
        Self::TokenizerError(r)
    }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;
