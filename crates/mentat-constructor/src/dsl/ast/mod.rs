use indexmap::IndexMap;
use mentat_tokenizer::Span;
use mentat_types::{NetworkIdentifier, Operation};
use serde_json::Value;

use crate::job::ActionType;

/// `Broadcast` contains information needed to create and broadcast a
/// transaction. `Broadcast` is returned from `Job` processing only IF a
/// broadcast is required.
pub struct Broadcast {
    network: NetworkIdentifier,
    intent: Vec<Operation>,
    metadata: IndexMap<String, Value>,
    confirmation_depth: i64,
    dry_run: bool,
}

/// `Job` is an instantion of a `Workflow`.
pub struct Job {
    /// Identifier is a UUID that is generated when a `Job` is stored in
    /// `JobStorage` for the first time. When executing the first scenario in a
    /// `Job`, this will be empty.
    identifier: String,
    state: String,
    index: i32,
    status: Status,
    /// workflow is the name of the `Workflow` being executed.
    workflow: String,
    /// Scenarios are copied into each context in case a configuration file
    /// changes that could corrupt in-process flows.
    scenarios: Vec<Scenario>,
}

/// `Status` is status of a `Job`.
pub enum Status {
    /// Broadcasting means that the intent of the last scenario is broadcasting.
    Broadcasting,
    /// Completed means that all scenarios were completed successfully.
    Completed,
    /// Failed means that Broadcasting failed.
    Failed,
    /// Ready means that a `Job` is ready to process.
    Ready,
}

/// `Workflow` is a collection of scenarios to run (i.e. transactions to
/// broadcast) with some shared state.
pub struct Workflow {
    name: String,
    /// Concurrency is the number of workflows of a particular kind to execute
    /// at once. For example, you may not want to process concurrent workflows
    /// of some staking operations that take days to play out.
    concurrency: i8,
    scenarios: Vec<Scenario>,
    span: Span,
}

/// `ReservedWorkflow` is a `Workflow` reserved for special circumstances.
/// All ReservedWorkflows must exist when running the constructor.
pub enum ReservedWorkflow {
    // CreateAccount is where another account (already with funds)
    // creates a new account. It is possible to configure how many
    // accounts should be created. CreateAccount must be executed
    // with a concurrency of 1.
    CreateAccount,
    // RequestFunds is where the user funds an account. This flow
    // is invoked when there are no pending broadcasts and it is not possible
    // to make progress on any Flows or start new ones. RequestFunds
    // must be executed with a concurrency of 1.
    RequestFunds,
    // ReturnFunds is invoked on shutdown so funds can be
    // returned to a single address (like a faucet). This
    // is useful for CI testing.
    ReturnFunds,
}

// Scenario is a collection of Actions with a specific
// confirmation depth.
//
// There is a special variable you can set at the end
// of a scenario called "<scenario_name>.operations" to
// indicate that a transaction should be broadcast. It is
// also possible to specify the network where the transaction
// should be broadcast and the metadata to provide in a
// call to /construction/preprocess.
//
// Once a scenario is broadcasted and confirmed,
// the transaction details are placed in a special
// variable called "transaction". This can be used
// in scenarios following the execution of this one.
/// <workflow name>(<concurrency>) { ...scenarios }
pub struct Scenario {
    name: String,
    scenarios: Vec<Action>,
    span: Span,
}

/// <scenario name> {
///   <output path> = <action type>(<input>);
/// }
pub struct Action {
    output_path: String,
    action_type: ActionType,
    input: Value,
    span: Span,
}
