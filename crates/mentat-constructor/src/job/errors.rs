use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum JobError {
    /// ErrNoBroadcastToConfirm is returned when there is no broadcast
    /// to confirm in a job.
    #[error("no broadcast to confirm")]
    ErrNoBroadcastToConfirm,
    /// ErrVariableNotFound is returned when a variable is not
    /// present in a Job's state.
    #[error("variable not found")]
    ErrVariableNotFound,
    /// ErrVariableIncorrectFormat is returned when a variable
    /// is in the incorrect format (i.e. when we find an int
    /// instead of a string).
    #[error("variable in incorrect format")]
    ErrVariableIncorrectFormat,
    /// ErrUnableToCreateBroadcast is returned when it is not possible
    /// to create a broadcast or check if a broadcast should be created
    /// from a job.
    #[error("unable to create broadcast")]
    ErrUnableToCreateBroadcast,
    /// ErrJobInWrongState is returned when a job is in wrong state
    #[error("job in wrong state")]
    ErrJobInWrongState,
    #[error("{0}")]
    String(String),
}

impl From<String> for JobError {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for JobError {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}

/// The job module result type.
pub type JobResult<T, E = JobError> = Result<T, E>;
