//! The module defines the `Operation` model.

use indexmap::IndexMap;

use super::*;

/// [`Operation`]s contain all balance-changing information within a
/// transaction. They are always one-sided (only affect 1 [`AccountIdentifier`])
/// and can succeed or fail independently from a [`Transaction`]. `Operation`s
/// are used both to represent on-chain data (Data API) and to construct new
/// transactions (Construction API), creating a standard interface for reading
/// and writing to blockchains.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Unchecked)]
#[serde(default)]
pub struct UncheckedOperation {
    /// The [`OperationIdentifier`] uniquely identifies an operation within a
    /// transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_identifier: Option<UncheckedOperationIdentifier>,
    /// Restrict referenced related_operations to identifier indices " the
    /// current [`OperationIdentifier`].index. This ensures there exists a clear
    /// DAG-structure of relations. Since operations are one-sided, one could
    /// imagine relating operations in a single transfer or linking operations
    /// in a call tree.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub related_operations: Vec<Option<UncheckedOperationIdentifier>>,
    /// Type is the network-specific type of the operation. Ensure that any type
    /// that can be returned here is also specified in the
    /// [`crate::responses::NetworkOptionsResponse`]. This can be very useful to
    /// downstream consumers that parse all block data.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub status: Option<String>,
    /// The [`AccountIdentifier`] uniquely identifies an account within a
    /// network. All fields in the account_identifier are utilized to
    /// determine this uniqueness (including the metadata field, if
    /// populated).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub account: Option<AccountIdentifier>,
    /// [`Amount`] is some Value of a [`Currency`]. It is considered invalid to
    /// specify a Value without a [`Currency`].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub amount: Option<UncheckedAmount>,
    /// `CoinChange` is used to represent a change in state of a some coin
    /// identified by a coin_identifier. This object is part of the
    /// [`Operation`] model and must be populated for UTXO-based
    /// blockchains. Coincidentally, this abstraction of UTXOs allows for
    /// supporting both account-based transfers and UTXO-based transfers on
    /// the same blockchain (when a transfer is account-based, don't
    /// populate this model).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub coin_change: Option<UncheckedCoinChange>,
    /// Any additional information related to the currency itself. For example,
    /// it would be useful to populate this object with the contract address of
    /// an ERC-20 token.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
