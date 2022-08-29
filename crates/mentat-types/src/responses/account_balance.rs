//! The module defines the `AccountBalanceResponse` response.

use super::*;

/// An `AccountBalanceResponse` is returned on the `/account/balance` endpoint.
/// If an account has a balance for each [`AccountIdentifier`] describing it
/// (ex: an ERC-20 token balance on a few smart contracts), an account balance
/// request must be made with each [`AccountIdentifier`]. The coins field was
/// removed and replaced by by `/account/coins` in v1.4.7.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableAccountBalanceResponse {
    /// The `block_identifier` uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<NullableBlockIdentifier>,
    /// A single account may have a balance in multiple currencies.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub balances: Vec<Option<NullableAmount>>,
    /// Account-based blockchains that utilize a nonce or sequence number should
    /// include that number in the metadata. This number could be unique to the
    /// identifier or global across the account address.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
