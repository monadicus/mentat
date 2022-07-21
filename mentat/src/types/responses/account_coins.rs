//! The module defines the `AccountCoinsResponse` response.

use indexmap::IndexMap;

use super::*;

/// `AccountCoinsResponse` is returned on the `/account/coins` endpoint and
/// includes all unspent [`Coin`]s owned by an [`AccountIdentifier`].
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountCoinsResponse {
    /// The `block_identifier` uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<BlockIdentifier>,
    /// If a blockchain is UTXO-based, all unspent `Coin`s owned by an
    /// `account_identifier` should be returned alongside the balance. It is
    /// highly recommended to populate this field so that users of the Rosetta
    /// API implementation don't need to maintain their own indexer to track
    /// their UTXOs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coins: Option<Vec<Option<Coin>>>,
    /// Account-based blockchains that utilize a nonce or sequence number should
    /// include that number in the `metadata`. This number could be unique to
    /// the identifier or global across the account address. Account-based
    /// blockchains that utilize a nonce or sequence number should include that
    /// number in the `metadata`. This number could be unique to the identifier
    /// or global across the account address.
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
