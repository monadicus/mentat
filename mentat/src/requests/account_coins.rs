//! The module defines the AccountCoinsRequest request.

use super::*;

/// `AccountCoinsRequest` is utilized to make a request on the `/account/coins`
/// endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AccountCoinsRequest {
    /// The `NetworkIdentifier` specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// The `AccountIdentifier` uniquely identifies an account within a
    /// network. All fields in the account_identifier are utilized to
    /// determine this uniqueness (including the `metadata` field, if
    /// populated).
    pub account_identifier: AccountIdentifier,
    /// Include state from the mempool when looking up an account's unspent
    /// coins. Note, using this functionality breaks any guarantee of
    /// idempotency.
    pub include_mempool: bool,
    /// In some cases, the caller may not want to retrieve all available
    /// balances for an [`AccountIdentifier`]. If the `currencies` field is
    /// populated, only balances for the specified currencies will be
    /// returned. If not populated, all available balances will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<Currency>>,
}
