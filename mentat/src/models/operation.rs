use indexmap::IndexMap;

use super::*;

/// Operations contain all balance-changing information within a transaction.
/// They are always one-sided (only affect 1 AccountIdentifier) and can succeed
/// or fail independently from a Transaction. Operations are used both to
/// represent on-chain data (Data API) and to construct new transactions
/// (Construction API), creating a standard interface for reading and writing to
/// blockchains.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Operation {
    /// The operation_identifier uniquely identifies an operation within a
    /// transaction.
    pub operation_identifier: OperationIdentifier,
    /// Restrict referenced related_operations to identifier indices " the
    /// current operation_identifier.index. This ensures there exists a clear
    /// DAG-structure of relations. Since operations are one-sided, one could
    /// imagine relating operations in a single transfer or linking operations
    /// in a call tree.
    pub related_operations: Option<Vec<OperationIdentifier>>,
    /// Type is the network-specific type of the operation. Ensure that any type
    /// that can be returned here is also specified in the
    /// NetworkOptionsResponse. This can be very useful to downstream consumers
    /// that parse all block data.
    #[serde(rename = "type")]
    pub type_: String,
    /// Status is the network-specific status of the operation. Status is not
    /// defined on the transaction object because blockchains with smart
    /// contracts may have transactions that partially apply (some operations
    /// are successful and some are not). Blockchains with atomic transactions
    /// (all operations succeed or all operations fail) will have the same
    /// status for each operation. On-chain operations (operations retrieved in
    /// the /block and /block/transaction endpoints) MUST have a populated
    /// status field (anything on-chain must have succeeded or failed). However,
    /// operations provided during transaction construction (often times called
    /// "intent" in the documentation) MUST NOT have a populated status field
    /// (operations yet to be included on-chain have not yet succeeded or
    /// failed).
    pub status: Option<String>,
    /// The account_identifier uniquely identifies an account within a network.
    /// All fields in the account_identifier are utilized to determine this
    /// uniqueness (including the metadata field, if populated).
    pub account: Option<AccountIdentifier>,
    /// Amount is some Value of a Currency. It is considered invalid to specify
    /// a Value without a Currency.
    pub amount: Option<Amount>,
    /// CoinChange is used to represent a change in state of a some coin
    /// identified by a coin_identifier. This object is part of the Operation
    /// model and must be populated for UTXO-based blockchains. Coincidentally,
    /// this abstraction of UTXOs allows for supporting both account-based
    /// transfers and UTXO-based transfers on the same blockchain (when a
    /// transfer is account-based, don't populate this model).
    pub coin_change: Option<CoinChange>,
    /// Any additional information related to the currency itself. For example,
    /// it would be useful to populate this object with the contract address of
    /// an ERC-20 token.
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
