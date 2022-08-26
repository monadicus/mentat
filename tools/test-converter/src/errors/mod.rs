#[macro_use]
mod create_error;

mod error_msg;
pub(crate) use error_msg::*;

mod input_file_error;
pub(crate) use input_file_error::*;
mod lexer_error;
pub(crate) use lexer_error::*;
mod parser_error;
pub(crate) use parser_error::*;

mod suggestion;
use error_stack::Report;
pub(crate) use suggestion::*;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub(crate) enum Error {
    #[error("{0:?}")]
    LexerError(Report<LexerError>),
    #[error("{0:?}")]
    ParserError(Report<ParserError>),
    #[error("{0:?}")]
    RulesError(Report<RulesFileError>),
}

impl From<Report<LexerError>> for Error {
    fn from(r: Report<LexerError>) -> Self {
        Self::LexerError(r)
    }
}

impl From<Report<ParserError>> for Error {
    fn from(r: Report<ParserError>) -> Self {
        Self::ParserError(r)
    }
}

impl From<Report<RulesFileError>> for Error {
    fn from(r: Report<RulesFileError>) -> Self {
        Self::RulesError(r)
    }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;
