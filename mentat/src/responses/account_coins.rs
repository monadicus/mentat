use indexmap::IndexMap;

use super::*;

/// AccountCoinsResponse is returned on the /account/coins endpoint and includes
/// all unspent Coins owned by an AccountIdentifier.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccountCoinsResponse {
    /// The block_identifier uniquely identifies a block in a particular
    /// network.
    pub block_identifier: BlockIdentifier,
    /// If a blockchain is UTXO-based, all unspent Coins owned by an
    /// account_identifier should be returned alongside the balance. It is
    /// highly recommended to populate this field so that users of the Rosetta
    /// API implementation don't need to maintain their own indexer to track
    /// their UTXOs.
    pub coins: Vec<Coin>,
    /// Account-based blockchains that utilize a nonce or sequence number should
    /// include that number in the metadata. This number could be unique to the
    /// identifier or global across the account address. Account-based
    /// blockchains that utilize a nonce or sequence number should include that
    /// number in the metadata. This number could be unique to the identifier or
    /// global across the account address.
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
