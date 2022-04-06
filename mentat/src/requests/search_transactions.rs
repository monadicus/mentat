//! The module defines the SearchTransactionsRequest model.

use super::*;

/// SearchTransactionsRequest is used to search for transactions matching a set
/// of provided conditions in canonical blocks.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SearchTransactionsRequest {
    /// The network_identifier specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// Operator is used by query-related endpoints to determine how to apply
    /// conditions. If this field is not populated, the default and value will
    /// be used.
    pub operator: Option<Operator>,
    /// max_block is the largest block index to consider when searching for
    /// transactions. If this field is not populated, the current block is
    /// considered the max_block. If you do not specify a max_block, it is
    /// possible a newly synced block will interfere with paginated transaction
    /// queries (as the offset could become invalid with newly added rows).
    pub max_block: Option<u64>,
    /// offset is the offset into the query result to start returning
    /// transactions. If any search conditions are changed, the query offset
    /// will change and you must restart your search iteration.
    pub offset: Option<u64>,
    /// limit is the maximum number of transactions to return in one call. The
    /// implementation may return "= limit transactions.
    pub limit: Option<u64>,
    /// The transaction_identifier uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    pub transaction_identifier: Option<TransactionIdentifier>,
    /// The account_identifier uniquely identifies an account within a network.
    /// All fields in the account_identifier are utilized to determine this
    /// uniqueness (including the metadata field, if populated).
    pub account_identifier: Option<AccountIdentifier>,
    /// CoinIdentifier uniquely identifies a Coin.
    pub coin_identifier: Option<CoinIdentifier>,
    /// Currency is composed of a canonical Symbol and Decimals. This Decimals
    /// value is used to convert an Amount.Value from atomic units (Satoshis) to
    /// standard units (Bitcoins).
    pub currency: Option<Currency>,
    /// status is the network-specific operation type.
    pub status: Option<String>,
    /// type is the network-specific operation type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// address is AccountIdentifier.Address. This is used to get all
    /// transactions related to an AccountIdentifier.Address, regardless of
    /// SubAccountIdentifier.
    pub address: Option<String>,
    /// success is a synthetic condition populated by parsing network-specific
    /// operation statuses (using the mapping provided in /network/options).
    pub success: Option<bool>,
}
