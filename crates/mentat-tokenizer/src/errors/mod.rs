#[macro_use]
pub mod create_error;

mod error_msg;
pub use error_msg::*;

mod lexer_error;
pub use lexer_error::*;
mod context_error;
pub use context_error::*;

mod suggestion;
use error_stack::Report;
pub use suggestion::*;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("{0:?}")]
    LexerError(Report<LexerError>),
    #[error("{0:?}")]
    ContextError(Report<ContextError>),
}

impl From<Report<LexerError>> for Error {
    fn from(r: Report<LexerError>) -> Self {
        Self::LexerError(r)
    }
}

impl From<Report<ContextError>> for Error {
    fn from(r: Report<ContextError>) -> Self {
        Self::ContextError(r)
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
