// pub TODO: these all need to be unchecked structs

use std::{fmt, time::Duration};

use mentat_types::{
    AccountIdentifier, Amount, CoinIdentifier, Currency, CurveType, Metadata, NetworkIdentifier,
    Operation, SubAccountIdentifier, UncheckedAmount, UncheckedCurrency,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tmp::KeyPair;

/// the expected concurrency of the create account and request funds scenario.
pub const RESERVED_WORKFLOW_CONCURRENCY: usize = 1;

/// ReservedVariable is a reserved variable
/// field in a Job's state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReservedVariable {
    /// Network is the NetworkIdentifier to use
    /// for broadcast.
    Network,
    /// Operations are the Vec<Operation> to use
    /// as intent.
    Operations,
    /// PreprocessMetadata is the metadata to provide to /construction/preprocess.
    PreprocessMetadata,
    /// Transaction is the Transaction confirmed
    /// on-chain.
    Transaction,
    /// ConfirmationDepth is the amount of blocks we wait to confirm
    /// a transaction. We allow setting this on a per scenario basis because
    /// certain transactions may only be considered complete
    /// after some time (pub ex: staking transaction).
    ConfirmationDepth,
    /// DryRun is a boolean that indicates whether we should perform the
    /// entire transaction construction process or just /construction/preprocess
    /// and /construction/metadata to determine the suggested transaction
    /// fee. If this variable is not populated, we assume that it is NOT
    /// a dry run.
    DryRun,
    /// SuggestedFee is the Vec<Amount> returned from
    /// an implementation's /construction/metadata endpoint (if implemented).
    SuggestedFee,
}

impl fmt::Display for ReservedVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReservedVariable::Network => write!(f, "network"),
            ReservedVariable::Operations => write!(f, "operations"),
            ReservedVariable::PreprocessMetadata => write!(f, "preprocess_metadata"),
            ReservedVariable::Transaction => write!(f, "transaction"),
            ReservedVariable::ConfirmationDepth => write!(f, "confirmation_depth"),
            ReservedVariable::DryRun => write!(f, "dry_run"),
            ReservedVariable::SuggestedFee => write!(f, "suggested_fee"),
        }
    }
}

/// ActionType is a type of Action that can be processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    /// GenerateKey creates a new *keys.KeyPair.
    GenerateKey,
    /// SaveAccount saves a generated *keys.KeyPair
    /// and AccountIdentifier to key storage.
    SaveAccount,
    /// Derive calls `/construction/derive` with a *keys.PublicKey.
    Derive,
    /// SetVariable allows for setting the value of any
    /// variable (as opposed to calculating it using
    /// other actions).
    SetVariable,
    /// FindBalance finds some unlocked account that optionally
    /// has a minimum balance of funds.
    /// FindCoin finds some coin above a certain amount. It is
    /// possible to specify coins which should not be considered (by identifier).
    FindBalance,
    /// PrintMessage can be used to print any message
    /// to the terminal. This is usually used to indicate
    /// to the caller that they should deposit money to an
    /// address. It is left generic to allow the user to
    /// include whatever content they would like in the
    /// message (especially useful in on-chain origination).
    /// This can also be used to log information during
    /// execution.
    PrintMessage,
    /// Math is used to perform addition or subtraction of variables. It is
    /// most commonly used to determine how much to send to a change output
    /// on UTXO blockchains.
    Math,
    /// RandomString generates a string according to some provided regex.
    /// It is used to generate account names for blockchains that require
    /// on-chain origination.
    RandomString,
    /// RandomNumber generates a random number in some range [min, max).
    /// It is used to generate random transaction amounts.
    RandomNumber,
    /// FindCurrencyAmount finds a Amount for a certain currency
    /// in an array of Vec<Amount>. This is typically used when parsing
    /// the suggested fee response from /construction/metadata.
    FindCurrencyAmount,
    /// Assert ensures that a provided number is >= 0 and causes
    /// execution to exit if this is not true. This is useful when
    /// ensuring that an account has sufficient balance to pay the
    /// suggested fee to broadcast a transaction.
    Assert,
    /// LoadEnv loads some value from an environment variable. This
    /// is very useful injecting an API token for algorithmic fauceting
    /// when running CI.
    LoadEnv,
    /// HTTPRequest makes an HTTP request at some URL. This is useful
    /// for making a request to a faucet to automate Construction API
    /// testing.
    HttpRequest,
    /// SetBlob stores an arbitrary blob at some key (any valid JSON is
    /// accepted as a key). If a value at a key already exists,
    /// it will be overwritten.
    //
    /// SetBlob is often used when there is some metadata created
    /// during a workflow execution that needs to be accessed
    /// in another workflow (i.e. a mapping between different generated
    /// addresses).
    SetBlob,
    /// GetBlob attempts to retrieve some previously saved blob.
    /// If the blob is not accessible, it will return an error.
    GetBlob,
}

/// MathOperation is some mathematical operation that
/// can be performed on 2 numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathOperation {
    /// Addition is adding LeftValue + RightValue.
    Addition,
    /// Subtraction is LeftValue - RightValue.
    Subtraction,
    /// Multiplication is LeftValue * RightValue.
    Multiplication,
    /// Division is LeftValue / RightValue.
    Division,
}

/// HttpMethod is a type representing
/// allowed HTTP methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
}

impl From<HttpMethod> for Method {
    fn from(v: HttpMethod) -> Self {
        match v {
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
        }
    }
}

/// ReservedWorkflow is a Workflow reserved for special circumstances.
/// All ReservedWorkflows must exist when running the constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReservedWorkflow {
    /// CreateAccount is where another account (already with funds)
    /// creates a new account. It is possible to configure how many
    /// accounts should be created. CreateAccount must be executed
    /// with a concurrency of 1.
    CreateAccount,
    /// RequestFunds is where the user funds an account. This flow
    /// is invoked when there are no pending broadcasts and it is not possible
    /// to make progress on any Flows or start new ones. RequestFunds
    /// must be executed with a concurrency of 1.
    RequestFunds,
    /// ReturnFunds is invoked on shutdown so funds can be
    /// returned to a single address (like a faucet). This
    /// is useful for CI testing.
    ReturnFunds,
}

/// Status is status of a Job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    /// Ready means that a Job is ready to process.
    Ready,
    /// Broadcasting means that the intent of the last
    /// scenario is broadcasting.
    Broadcasting,
    /// Failed means that Broadcasting failed.
    Failed,
    /// Completed means that all scenarios were
    /// completed successfully.
    Completed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Ready => write!(f, "ready"),
            Status::Broadcasting => write!(f, "broadcasting"),
            Status::Failed => write!(f, "failed"),
            Status::Completed => write!(f, "completed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Action is a step of computation that
/// where the result is saved to OutputPath.
pub struct Action {
    pub input: String,
    pub type_: ActionType,
    pub output_path: String,
}

/// GenerateKeyInput is the input for GenerateKey.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateKeyInput {
    pub curve_type: CurveType,
}

// pub TODO: add KeyPair from keys
/// SaveAccountInput is the input for SaveAccount.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveAccountInput {
    pub account_identifier: Option<AccountIdentifier>,
    pub key_pair: Option<KeyPair>,
}

/// RandomStringInput is the input to RandomString.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomStringInput {
    pub regex: String,
    /// Limit is the maximum number of times each star, range, or
    /// plus character could be repeated.
    pub limit: usize,
}

/// MathInput is the input to Math.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathInput {
    pub operation: MathOperation,
    pub left_value: String,
    pub right_value: String,
}

/// FindBalanceInput is the input to FindBalance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindBalanceInput {
    /// AccountIdentifier can be optionally provided to ensure the balance returned
    /// is for a particular address (this is used when fetching the balance
    /// of the same account in multiple currencies, when requesting funds,
    /// or when constructing a multi-input UTXO transfer with the
    /// same address).
    pub account_identifier: Option<AccountIdentifier>,

    /// SubAccountIdentifier can be used to find addresses with particular
    /// SubAccount balances. This is particularly useful for
    /// orchestrating staking transactions.
    pub sub_account_identifier: Option<SubAccountIdentifier>,

    /// NotAddress can be populated to ensure a different
    /// address is found. This is useful when avoiding a
    /// self-transfer.
    pub not_address: Vec<String>,

    /// NotAccountIdentifier can be used to avoid entire
    /// *types.AccountIdentifiers and is used when only
    /// certain SubAccountIdentifiers of an Address are
    /// desired.
    pub not_account_identifier: Vec<Option<AccountIdentifier>>,

    /// MinimumBalance is the minimum required balance that must be found.
    pub minimum_balance: UncheckedAmount,

    /// RequireCoin indicates if a coin must be found with the minimum balance.
    /// This is useful for orchestrating transfers on UTXO-based blockchains.
    pub require_coin: bool,

    /// NotCoins indicates that certain coins should not be considered. This is useful
    /// for avoiding using the same Coin twice.
    pub not_coins: Vec<CoinIdentifier>,

    /// CreateLimit is used to determine if we should create a new address using
    /// the CreateAccount Workflow. This will only occur if the
    /// total number of addresses is under some pre-defined limit.
    /// If the value is <= 0, we will not attempt to create.
    pub create_limit: isize,

    /// CreateProbability is used to determine if a new account should be
    /// created with some probability [0, 100). This will override the search
    /// for any valid accounts and instead return ErrCreateAccount.
    pub create_probability: u32,
}

/// FindBalanceOutput is returned by FindBalance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindBalanceOutput {
    /// AccountIdentifier is the account associated with the balance
    /// (and coin).
    pub account_identifier: Option<AccountIdentifier>,

    /// Balance found at a particular currency.
    pub balance: Option<Amount>,

    /// Coin is populated if RequireCoin is true.
    pub coin: Option<CoinIdentifier>,
}

/// RandomNumberInput is used to generate a random
/// number in the range [minimum, maximum).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomNumberInput {
    pub minimum: String,
    pub maximum: String,
}

/// FindCurrencyAmountInput is the input
/// to FindCurrencyAmount.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCurrencyAmountInput {
    pub currency: Option<UncheckedCurrency>,
    pub amounts: Vec<Option<UncheckedAmount>>,
}

/// the input to an HTTP Request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequestInput {
    pub method: HttpMethod,
    pub url: String,
    pub timeout: Duration,
    /// If the Method is POST, the Body
    /// can be populated with JSON.
    pub body: String,
}

/// HTTPRequestInput is the input to
/// HTTP Request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBlobInput {
    // TODO this may be wrong
    pub key: String,
    pub value: Value,
}

/// GetBlobInput is the input to
/// GetBlob.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlobInput {
    // TODO this may be wrong
    pub key: String,
}

/// Scenario is a collection of Actions with a specific
/// confirmation depth.
///
/// There is a special variable you can set at the end
/// of a scenario called "<scenario_name>.operations" to
/// indicate that a transaction should be broadcast. It is
/// also possible to specify the network where the transaction
/// should be broadcast and the metadata to provide in a
/// call to /construction/preprocess.
///
/// Once a scenario is broadcasted and confirmed,
/// the transaction details are placed in a special
/// variable called "transaction". This can be used
/// in scenarios following the execution of this one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub actions: Vec<Action>,
}

/// Workflow is a collection of scenarios to run (i.e.
/// transactions to broadcast) with some shared state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    /// Concurrency is the number of workflows of a particular
    /// kind to execute at once. For example, you may not want
    /// to process concurrent workflows of some staking operations
    /// that take days to play out.
    pub concurrency: usize,
    pub scenarios: Vec<Scenario>,
}

/// Job is an instantiation of a Workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    /// Identifier is a UUID that is generated
    /// when a Job is stored in JobStorage for the
    /// first time. When executing the first scenario
    /// in a Job, this will be empty.
    pub identifier: String,
    pub state: Value,
    pub index: usize,
    pub status: Status,
    /// Workflow is the name of the workflow being executed.
    pub workflow: String,
    /// Scenarios are copied into each context in case
    /// a configuration file changes that could corrupt
    /// in-process flows.
    pub scenarios: Vec<Scenario>,
}

/// Broadcast contains information needed to create
/// and broadcast a transaction. Broadcast is returned
/// from Job processing only IF a broadcast is required.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broadcast {
    pub network: Option<NetworkIdentifier>,
    pub intent: Vec<Operation>,
    pub metadata: Metadata,
    pub confirmation_depth: usize,
    pub dry_run: bool,
}
