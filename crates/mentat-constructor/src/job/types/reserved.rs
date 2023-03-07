use super::*;

pub static RESERVED_WORKFLOW_CONCURRENCY: u8 = 1;

/// `ReservedVariable` is a reserved variable field in a `Job`'s state.
pub enum ReservedVariable {
    /// ConfirmationDepth is the amount of blocks we wait to confirm a
    /// transaction. We allow setting this on a per scenario basis because
    /// certain transactions may only be considered complete after some time
    /// (ex: staking transaction).
    // "confirmation_depth"
    ConfirmationDepth,
    /// DryRun is a `bool` that indicates whether we should perform the entire
    /// transaction construction process or just /construction/preprocess
    /// and /construction/metadata to determine the suggested transaction
    /// fee. If this variable is not populated, we assume that it is NOT a
    /// dry run.
    // "dry_run"
    DryRun,
    /// Network is the `NetworkIdentifier` to use for broadcast.
    // "network"
    Network,
    /// Operations are the `Vec<Operation>` to use as intent.
    // "operations"
    Operations,
    /// PreprocessMetadata is the metadata to provide to
    /// /construction/preprocess.
    // "preprocess_metadata"
    PreprocessMetadata,
    /// SuggestedFee is the `Vec<Amount>` returned from an implementation's
    /// /construction/metadata endpoint (if implemented).
    // "suggested_fee"
    SuggestedFee,
    /// Transaction is the `Transaction` confirmed on-chain.
    // "transaction"
    Transaction,
}

/// `ReservedWorkflow` is a `Workflow` reserved for special circumstances.
/// All ReservedWorkflows must exist when running the constructor.
pub enum ReservedWorkflow {
    /// CreateAccount is where another account (already with funds) creates a
    /// new account. It is possible to configure how many accounts should be
    /// created. CreateAccount must be executed with a concurrency of 1.
    CreateAccount,
    /// RequestFunds is where the user funds an account. This flow is invoked
    /// when there are no pending broadcasts and it is not possible to make
    /// progress on any Flows or start new ones. RequestFunds must be executed
    /// with a concurrency of 1.
    RequestFunds,
    /// ReturnFunds is invoked on shutdown so funds can be returned to a single
    /// address (like a faucet). This is useful for CI testing.
    ReturnFunds,
}
