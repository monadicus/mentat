use crate::job::Action;
use mentat_types::Metadata;
use serde_json::Value;
use std::fmt::Write;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum WorkerError {
    /// returned when a populated value is not valid JSON.
    #[error("populated input is not valid JSON")]
    InvalidJSON,

    /// returned when a variable is not present in a Job's state.
    #[error("variable not found")]
    VariableNotFound,

    /// returned when there are no more scenarios to process in a Job.
    #[error("job complete")]
    JobComplete,

    /// returned when the input for an Action cannot be parsed.
    #[error("invalid input")]
    InvalidInput,

    /// returned when an Action has an unsupported type.
    #[error("invalid action type")]
    InvalidActionType,

    /// returned when Action execution fails with a valid input.
    #[error("action execution failed")]
    ActionFailed,

    /// returned when a new account should be created using the `create_account` workflow.
    #[error("create account")]
    CreateAccount,

    /// returned when there is no available balance that can satisfy a FindBalance request. If there are no pending broadcasts, this usually means that we need to request funds.
    #[error("unsatisfiable balance")]
    Unsatisfiable,

    /// returned when the input operation is not supported.
    #[error("the input operation is not supported")]
    InputOperationIsNotSupported,

    #[error("{0}")]
    String(String),
}

impl From<String> for WorkerError {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for WorkerError {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}

/// The worker module result type.
pub type WorkerResult<T, E = WorkerError> = Result<T, E>;

// WorkerErrorInfo is returned by worker execution.
pub struct WorkerErrorInfo {
    workflow: String,
    job: Option<String>,
    scenario: String,
    scenario_index: String,
    action_index: String,
    action: Option<Action>,
    processed_input: String,
    output: String,
    state: Option<Metadata>,
    err: WorkerError,
}

/// Log prints the error to the console in a human readable format.
impl WorkerErrorInfo {
    pub fn log(&self) {
        let mut message = format!("\x1b[33EXECUTION FAILED!\nMessage: {}\n\n", self.err);

        // job identifier is only assigned if persisted once
        if let Some(j) = &self.job {
            writeln!(message, "Job: {j}").unwrap();
        }

        write!(
            message,
            "Workflow: {}\nScenario: {}\nScenario Index: {}\n\n",
            self.workflow, self.scenario, self.scenario_index
        )
        .unwrap();

        if let Some(a) = &self.action {
            writeln!(
                message,
                "Action Index: {}\nAction: {a:?}",
                self.action_index
            )
            .unwrap();

            write!(
                message,
                "Processed Input: {}\nOutput: {}\n\n",
                self.processed_input, self.output
            )
            .unwrap();
        }

        if let Some(m) = &self.state {
            writeln!(message, "State: {m:?}").unwrap()
        }

        writeln!(message, "\x1b[0m").unwrap();

        println!("{message}");
    }
}
