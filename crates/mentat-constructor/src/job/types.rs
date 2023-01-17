use indexmap::IndexMap;
use mentat_tokenizer::Span;
use mentat_types::{
    AccountIdentifier,
    Amount,
    CoinIdentifier,
    Currency,
    CurveType,
    NetworkIdentifier,
    Operation,
    SubAccountIdentifier,
};
use serde_json::Value;

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

/// `Job` is an instantiation of a `Workflow`.
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

/// `Scenario` is a collection of `Action`s with a specific confirmation depth.
///
/// There is a special variable you can set at the end of a scenario called
/// "<scenario_name>.operations" to indicate that a transaction should be
/// broadcast. It is also possible to specify the network where the transaction
/// should be broadcast and the metadata to provide in a call to
/// /construction/preprocess.
///
/// Once a scenario is broadcasted and confirmed, the transaction details are
/// placed in a special variable called "transaction". This can be used in
/// scenarios following the execution of this one. <workflow
/// name>(<concurrency>) { ...scenarios }
pub struct Scenario {
    name: String,
    scenarios: Vec<Action>,
    span: Span,
}

/// <scenario name> {
///   <output path> = <action type>(<input>);
/// }
/// `Action` is a step of computation that where the result is saved to
/// OutputPath.
pub struct Action {
    output_path: String,
    action_type: ActionType,
    input: Value,
    span: Span,
}

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

/// `ActionType` is a type of Action that can be processed and it's input.
pub enum ActionType {
    /// Assert ensures that a provided number is >= 0 and causes execution to
    /// exit if this is not true. This is useful when ensuring that an account
    /// has sufficient balance to pay the suggested fee to broadcast a
    /// transaction.
    // "assert"
    Assert,
    /// Derive calls `/construction/derive` with a *keys.PublicKey.
    // "derive"
    Derive,
    /// FindBalance finds some unlocked account that optionally has a minimum
    /// balance of funds. FindCoin finds some coin above a certain amount. It is
    /// possible to specify coins which should not be considered (by
    /// identifier).
    // "find_balance"
    FindBalance {
        // account_identifier can be optionally provided to ensure the balance returned
        // is for a particular address (this is used when fetching the balance
        // of the same account in multiple currencies, when requesting funds,
        // or when constructing a multi-input UTXO transfer with the
        // same address).
        account_identifier: Option<AccountIdentifier>,

        // sub_account_identifier can be used to find addresses with particular
        // SubAccount balances. This is particularly useful for
        // orchestrating staking transactions.
        sub_account_identifier: Option<SubAccountIdentifier>,

        // not_address can be populated to ensure a different
        // address is found. This is useful when avoiding a
        // self-transfer.
        not_address: Vec<String>,

        // not_account_identifier can be used to avoid entire
        // *types.AccountIdentifiers and is used when only
        // certain SubAccountIdentifiers of an Address are
        // desired.
        not_account_identifier: Vec<AccountIdentifier>,

        // minimum_balance is the minimum required balance that must be found.
        minimum_balance: Option<Amount>,

        // require_coin indicates if a coin must be found with the minimum balance.
        // This is useful for orchestrating transfers on UTXO-based blockchains.
        require_coin: bool,

        // not_coins indicates that certain coins should not be considered. This is useful
        // for avoiding using the same Coin twice.
        not_coins: Vec<CoinIdentifier>,

        // create_limit is used to determine if we should create a new address using
        // the CreateAccount Workflow. This will only occur if the
        // total number of addresses is under some pre-defined limit.
        // If the value is <= 0, we will not attempt to create.
        create_limit: isize,

        // create_probability is used to determine if a new account should be
        // created with some probability [0, 100). This will override the search
        // for any valid accounts and instead return ErrCreateAccount.
        create_probability: i8,
    },
    /// FindCurrencyAmount finds a `Vec<Amount>` for a certain currency in an
    /// array of `Vec<Amount>`. This is typically used when parsing the
    /// suggested fee response from /construction/metadata.
    // "find_currency_amount"
    FindCurrencyAmount {
        currency: Currency,
        amounts: Vec<Amount>,
    },
    /// GenerateKey creates a new *keys.KeyPair.
    // "generate_key"
    GenerateKey { curve_type: CurveType },
    /// GetBlob attempts to retrieve some previously saved blob. If the blob is
    /// not accessible, it will return an error.
    // "get_blob"
    GetBlob { key: Value },
    /// HTTPRequest makes an HTTP request at some URL. This is useful for making
    /// a request to a faucet to automate Construction API testing.
    // "http_request"
    HttpRequest {
        method: HttpMethod,
        url: String,
        timeout: isize,
        body: String,
    },
    /// LoadEnv loads some value from an environment variable. This is very
    /// useful injecting an API token for algorithmic fauceting when running CI.
    // "load_env"
    LoadEnv,
    /// Math is used to perform addition or subtraction of variables. It is most
    /// commonly used to determine how much to send to a change output on UTXO
    /// blockchains.
    // 1 + {{fee}}
    Math {
        operation: MathOperation,
        lhs: String,
        rhs: String,
    },
    /// PrintMessage can be used to print any message to the terminal. This is
    /// usually used to indicate to the caller that they should deposit money to
    /// an address. It is left generic to allow the user to include whatever
    /// content they would like in the message (especially useful in on-chain
    /// origination). This can also be used to log information during execution.
    // "print_message"
    PrintMessage(String),
    // SaveAccount saves a generated *keys.KeyPair and `AccountIdentifier` to key storage.
    // "save_account"
    SaveAccount {
        account_identifier: AccountIdentifier,
        key_pair: (), // todo KEYS,
    },
    /// SetBlob stores an arbitrary blob at some key (any valid JSON is accepted
    /// as a key). If a value at a key already exists, it will be overwritten.
    /// SetBlob is often used when there is some metadata created during a
    /// workflow execution that needs to be accessed in another workflow (i.e. a
    /// mapping between different generated addresses).
    // "set_blob"
    SetBlob { key: Value, value: Value },
    /// SetVariable allows for setting the value of any variable (as opposed to
    /// calculating it using other actions).
    // Empty string
    SetVariable,
    /// RandomString generates a string according to some provided regex. It is
    /// used to generate account names for blockchains that require on-chain
    /// origination.
    // "random_string"
    RandomString {
        regex: String,
        // Limit is the maximum number of times each star, range, or plus character could be
        // repeated.
        limit: isize,
    },
    /// RandomNumber generates a random number in some range [min, max). It is
    /// used to generate random transaction amounts.
    // "random_number"
    RandomNumber { minimum: String, maximum: String },
}

pub enum HttpMethod {
    Get,
    Post,
}

pub enum MathOperation {
    /// Addition is adding lhs + rhs.
    Addition,
    /// Subtraction is adding lhs - rhs.
    Subtraction,
    /// Multiplication is adding lhs * rhs.
    Multiplication,
    /// Division is adding lhs / rhs.
    Division,
}

/// `FindBalanceOutput` is returned by `Action::FindBalance`.
pub struct FindBalanceOutput {
    /// account_identifier is the account associated with the balance (and
    /// coin).
    account_identifier: AccountIdentifier,

    /// balance found at a particular currency.
    balance: Amount,

    /// coin is populated if RequireCoin is true.
    coin: CoinIdentifier,
}
