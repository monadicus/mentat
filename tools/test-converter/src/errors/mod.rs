#[macro_use]
mod create_error;

mod error_msg;
pub(crate) use error_msg::*;

mod input_file_error;
pub(crate) use input_file_error::*;

mod suggestion;
use error_stack::Report;
pub(crate) use suggestion::*;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub(crate) enum Error {
    #[error("{0:?}")]
    RulesError(Report<RulesFileError>),
}

impl From<Report<RulesFileError>> for Error {
    fn from(r: Report<RulesFileError>) -> Self {
        Self::RulesError(r)
    }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;
