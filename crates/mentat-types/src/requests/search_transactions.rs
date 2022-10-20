//! The module defines the `SearchTransactionsRequest` request.

use super::*;

/// [`SearchTransactionsRequest`] is used to search for transactions matching a
/// set of provided conditions in canonical blocks.
#[derive(Clone, Debug, Deserialize, Serialize, Default, Unchecked)]
#[serde(default, deny_unknown_fields)]
pub struct UncheckedSearchTransactionsRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// `Operator` is used by query-related endpoints to determine how to apply
    /// conditions. If this field is not populated, the default and value will
    /// be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<UncheckedOperator>,
    /// `max_block` is the largest block index to consider when searching for
    /// transactions. If this field is not populated, the current block is
    /// considered the `max_block`. If you do not specify a `max_block`, it is
    /// possible a newly synced block will interfere with paginated transaction
    /// queries (as the offset could become invalid with newly added rows).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(option_usize)]
    pub max_block: Option<isize>,
    /// offset is the offset into the query result to start returning
    /// transactions. If any search conditions are changed, the query offset
    /// will change and you must restart your search iteration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(option_usize)]
    pub offset: Option<isize>,
    /// limit is the maximum number of transactions to return in one call. The
    /// implementation may return "= limit transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(option_usize)]
    pub limit: Option<isize>,
    /// The `TransactionIdentifier` uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub transaction_identifier: Option<TransactionIdentifier>,
    /// The `AccountIdentifier` uniquely identifies an account within a network.
    /// All fields in the `account_identifier` are utilized to determine this
    /// uniqueness (including the `metadata` field, if populated).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub account_identifier: Option<AccountIdentifier>,
    /// `CoinIdentifier` uniquely identifies a [`Coin`].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub coin_identifier: Option<CoinIdentifier>,
    /// `Currency` is composed of a canonical Symbol and Decimals. This Decimals
    /// value is used to convert an [`Amount`].value from atomic units
    /// (Satoshis) to standard units (Bitcoins).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub currency: Option<UncheckedCurrency>,
    /// status is the network-specific operation type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub status: Option<String>,
    /// type is the network-specific operation type.
    #[serde(rename = "type")]
    #[unchecked(retain)]
    pub type_: Option<String>,
    /// address is [`AccountIdentifier`].address. This is used to get all
    /// transactions related to an [`AccountIdentifier`].address, regardless of
    /// [`SubAccountIdentifier`].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub address: Option<String>,
    /// success is a synthetic condition populated by parsing network-specific
    /// operation statuses (using the mapping provided in `/network/options`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub success: Option<bool>,
}
