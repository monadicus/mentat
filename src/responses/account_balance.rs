use indexmap::IndexMap;

use super::*;

/// An AccountBalanceResponse is returned on the /account/balance endpoint. If an account has a balance for each AccountIdentifier describing it (ex: an ERC-20 token balance on a few smart contracts), an account balance request must be made with each AccountIdentifier. The coins field was removed and replaced by by /account/coins in v1.4.7.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct AccountBalanceResponse {
    /// The block_identifier uniquely identifies a block in a particular network.
    pub block_identifier: BlockIdentifier,
    /// A single account may have a balance in multiple currencies.
    pub balances: Vec<Amount>,
    /// Account-based blockchains that utilize a nonce or sequence number should include that number in the metadata. This number could be unique to the identifier or global across the account address.
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
