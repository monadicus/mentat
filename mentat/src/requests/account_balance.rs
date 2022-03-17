use super::*;

/// An AccountBalanceRequest is utilized to make a balance request on the
/// /account/balance endpoint. If the block_identifier is populated, a
/// historical balance query should be performed.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccountBalanceRequest {
    /// The network_identifier specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// The account_identifier uniquely identifies an account within a network.
    /// All fields in the account_identifier are utilized to determine this
    /// uniqueness (including the metadata field, if populated).
    pub account_identifier: AccountIdentifier,
    /// When fetching data by BlockIdentifier, it may be possible to only
    /// specify the index or hash. If neither property is specified, it is
    /// assumed that the client is making a request at the current block.
    pub block_identifier: Option<PartialBlockIdentifier>,
    /// In some cases, the caller may not want to retrieve all available
    /// balances for an AccountIdentifier. If the currencies field is populated,
    /// only balances for the specified currencies will be returned. If not
    /// populated, all available balances will be returned.
    pub currencies: Option<Vec<Currency>>,
}
