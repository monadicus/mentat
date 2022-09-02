mod input_file_error;
use error_stack::Report;
pub(crate) use input_file_error::*;
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
