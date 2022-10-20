//! The module defines the `AccountBalanceRequest` request.

use super::*;

/// An `AccountBalanceRequest` is utilized to make a balance request on the
/// `/account/balance` endpoint. If the `block_identifier` is populated, a
/// historical balance query should be performed.
#[derive(Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default, deny_unknown_fields)]
pub struct UncheckedAccountBalanceRequest {
    /// The `NetworkIdentifier` specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// The `AccountIdentifier` uniquely identifies an account within a
    /// network. All fields in the `account_identifier` are utilized to
    /// determine this uniqueness (including the `metadata` field, if
    /// populated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<AccountIdentifier>,
    /// When fetching data by [`BlockIdentifier`], it may be possible to only
    /// specify the index or hash. If neither property is specified, it is
    /// assumed that the client is making a request at the current block.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub block_identifier: Option<UncheckedPartialBlockIdentifier>,
    /// In some cases, the caller may not want to retrieve all available
    /// balances for an [`AccountIdentifier`]. If the currencies field is
    /// populated, only balances for the specified currencies will be
    /// returned. If not populated, all available balances will be returned.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub currencies: Vec<Option<UncheckedCurrency>>,
}
