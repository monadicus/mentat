//! The module defines the `Allow` model.

use super::*;

/// `Allow` specifies supported [`Operation`] status, [`Operation`] types, and
/// all possible error statuses. This `Allow` object is used by clients to
/// validate the correctness of a Rosetta Server implementation. It is expected
/// that these clients will error if they receive some response that contains
/// any of the above information that is not specified here.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Allow {
    /// All `OperationStatus` this implementation supports. Any status that is
    /// returned during parsing that is not listed here will cause client
    /// validation to error.
    pub operation_statuses: Vec<OperationStatus>,
    /// All Operation Type this implementation supports. Any type that is
    /// returned during parsing that is not listed here will cause client
    /// validation to error.
    pub operation_types: Vec<String>,
    /// All `ApiError` that this implementation could return. Any error that
    /// is returned during parsing that is not listed here will cause client
    /// validation to error.
    pub errors: Vec<MentatError>,
    /// Any Rosetta implementation that supports querying the balance of an
    /// account at any height in the past should set this to true.
    pub historical_balance_lookup: bool,
    /// If populated, `timestamp_start_index` indicates the first block index
    /// where block timestamps are considered valid (i.e. all blocks less than
    /// `timestamp_start_index` could have invalid timestamps). This is useful
    /// when the genesis block (or blocks) of a network have timestamp 0. If not
    /// populated, block timestamps are assumed to be valid for all available
    /// blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_start_index: Option<u64>,
    /// All methods that are supported by the `/call` endpoint. Communicating
    /// which parameters should be provided to `/call` is the responsibility of
    /// the implementer (this is en lieu of defining an entire type system and
    /// requiring the implementer to define that in `Allow`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_methods: Option<Vec<String>>,
    /// `BalanceExemption`s is an array of `BalanceExemption` indicating which
    /// account balances could change without a corresponding Operation.
    /// `BalanceExemption`s should be used sparingly as they may introduce
    /// significant complexity for integrators that attempt to reconcile all
    /// account balance changes. If your implementation relies on any
    /// `BalanceExemption`s, you MUST implement historical balance lookup (the
    /// ability to query an account balance at any [`BlockIdentifier`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_exemptions: Option<Vec<BalanceExemption>>,
    /// Any Rosetta implementation that can update an [`AccountIdentifier`]'s
    /// unspent coins based on the contents of the mempool should populate this
    /// field as true. If false, requests to `/account/coins` that set
    /// `include_mempool` as true will be automatically rejected.
    pub mempool_coins: bool,
}
