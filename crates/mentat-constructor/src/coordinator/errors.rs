use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum ConstructorError {
    /// ErrNoAvailableJobs is returned when it is not possible
    /// to process any jobs. If this is returned, you should wait
    /// and retry.
    #[error("no jobs available")]
    NoAvailableJobs,
    /// ErrReturnFundsComplete is returned when it is not possible
    /// to process any more ReturnFundsWorkflows or when there is no provided
    /// ReturnsFundsWorkflow.
    #[error("return funds complete")]
    ReturnFundsComplete,
    /// ErrDuplicateWorkflows is returned when 2 Workflows with the same name
    /// are provided as an input to NewCoordinator.
    #[error("duplicate workflows")]
    DuplicateWorkflows,
    /// ErrIncorrectConcurrency is returned when CreateAccount or RequestFunds
    /// have a concurrency greater than 1.
    #[error("incorrect concurrency")]
    IncorrectConcurrency,
    /// ErrInvalidConcurrency is returned when the concurrency of a Workflow
    /// is <= 0.
    #[error("invalid concurrency")]
    InvalidConcurrency,
    /// ErrStalled is returned when the caller does not define
    /// a CreateAccount and/or RequestFunds workflow and we run out
    /// of available options (i.e. we can't do anything).
    #[error("processing stalled")]
    Stalled,
    /// ErrNoWorkflows is returned when no workflows are provided
    /// during initialization.
    #[error("no workflows")]
    NoWorkflows,
    /// ErrSignersNotEmpty is returned when signers are not empty in unsigned transaction
    #[error("signers are not empty in unsigned transaction")]
    SignersNotEmpty,
}
