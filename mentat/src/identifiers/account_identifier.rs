use indexmap::IndexMap;

use super::*;

/// The account_identifier uniquely identifies an account within a network. All
/// fields in the account_identifier are utilized to determine this uniqueness
/// (including the metadata field, if populated).
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AccountIdentifier {
    /// The address may be a cryptographic public key (or some encoding of it)
    /// or a provided username.
    pub address: String,
    /// An account may have state specific to a contract address (ERC-20 token)
    /// and/or a stake (delegated balance). The sub_account_identifier should
    /// specify which state (if applicable) an account instantiation refers to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<SubAccountIdentifier>,
    /// Any additional information related to the currency itself. For example,
    /// it would be useful to populate this object with the contract address of
    /// an ERC-20 token.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
