use crate::job::Action;
use mentat_asserter::{
    AccountBalanceError, AsserterError, BlockError, CoinError, ConstructionError, ErrorError,
    EventError, NetworkError, SearchError, ServerError, UtilError,
};
use mentat_utils::utils::AccountBalance;
use serde_json::Value;
use std::fmt;
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

    #[error("asserter error: {0}")]
    AsserterError(AsserterError),

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

impl From<AsserterError> for WorkerError {
    fn from(v: AsserterError) -> Self {
        Self::AsserterError(v)
    }
}

// cant use generic Into<AsserterError> impl because it interferes with the from string impls above
macro_rules! from_asserter_error {
    ($e:ident) => {
        impl From<$e> for WorkerError {
            fn from(v: $e) -> Self {
                Self::AsserterError(v.into())
            }
        }
    };
}
from_asserter_error!(AccountBalanceError);
from_asserter_error!(BlockError);
from_asserter_error!(CoinError);
from_asserter_error!(ConstructionError);
from_asserter_error!(NetworkError);
from_asserter_error!(ServerError);
from_asserter_error!(EventError);
from_asserter_error!(SearchError);
from_asserter_error!(ErrorError);
from_asserter_error!(UtilError);

/// The worker module result type.
pub type WorkerResult<T> = Result<T, WorkerError>;

// WorkerErrorInfo is returned by worker execution.
#[derive(Debug)]
pub struct VerboseWorkerError {
    pub workflow: String,
    pub job: Option<String>,
    pub scenario: String,
    pub scenario_index: usize,
    pub action_index: usize,
    pub action: Option<Action>,
    pub processed_input: Option<Value>,
    pub output: Option<Value>,
    pub state: Option<Value>,
    pub err: WorkerError,
}

impl fmt::Display for VerboseWorkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[33EXECUTION FAILED!\nMessage: {}\n\n", self.err)?;

        // job identifier is only assigned if persisted once
        if let Some(j) = &self.job {
            writeln!(f, "Job: {j}")?;
        }

        write!(
            f,
            "Workflow: {:?}\nScenario: {:?}\nScenario Index: {:?}\n\n",
            self.workflow, self.scenario, self.scenario_index
        )?;

        if let Some(a) = &self.action {
            writeln!(f, "Action Index: {}\nAction: {a:?}", self.action_index)?;

            write!(
                f,
                "Processed Input: {:?}\nOutput: {:?}\n\n",
                self.processed_input, self.output
            )?;
        }

        if let Some(m) = &self.state {
            writeln!(f, "State: {m:?}")?
        }

        writeln!(f, "\x1b[0m")
    }
}

impl Default for VerboseWorkerError {
    fn default() -> Self {
        Self {
            workflow: Default::default(),
            job: Default::default(),
            scenario: Default::default(),
            scenario_index: Default::default(),
            action_index: Default::default(),
            action: Default::default(),
            processed_input: Default::default(),
            output: Default::default(),
            state: Default::default(),
            err: WorkerError::String("Default Error".into()),
        }
    }
}

/// The worker module result type.
pub type VerboseWorkerResult<T> = Result<T, VerboseWorkerError>;
