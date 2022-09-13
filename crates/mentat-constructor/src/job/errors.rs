use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum JobError {
    /// returned when there is no broadcast
    /// to confirm in a job.
    #[error("no broadcast to confirm")]
    NoBroadcastToConfirm,

    /// returned when a variable is not
    /// present in a Job's state.
    #[error("variable not found")]
    VariableNotFound,

    /// returned when a variable
    /// is in the incorrect format (i.e. when we find an int
    /// instead of a string).
    #[error("variable in incorrect format")]
    VariableIncorrectFormat,

    /// returned when it is not possible
    /// to create a broadcast or check if a broadcast should be created
    /// from a job.
    #[error("unable to create broadcast")]
    UnableToCreateBroadcast,

    /// returned when a job is in wrong state
    #[error("job in wrong state")]
    JobInWrongState,

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
pub type JobResult<T> = Result<T, JobError>;
