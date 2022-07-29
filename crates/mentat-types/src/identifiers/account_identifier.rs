//! The module defines the `AccountIdentifier`.

use indexmap::IndexMap;

use super::*;

/// The [`AccountIdentifier`] uniquely identifies an account within a network.
/// All fields in the `account_identifier` are utilized to determine this
/// uniqueness (including the metadata field, if populated).
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct AccountIdentifier {
    /// The address may be a cryptographic public key (or some encoding of it)
    /// or a provided username.
    pub address: String,
    /// An account may have state specific to a contract address (ERC-20 token)
    /// and/or a stake (delegated balance). The [`SubAccountIdentifier`] should
    /// specify which state (if applicable) an account instantiation refers to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<SubAccountIdentifier>,
    /// Any additional information related to the currency itself. For example,
    /// it would be useful to populate this object with the contract address of
    /// an ERC-20 token.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}

impl From<String> for AccountIdentifier {
    fn from(address: String) -> Self {
        Self {
            address,
            ..Default::default()
        }
    }
}

impl From<(String, String)> for AccountIdentifier {
    fn from((address, subaccount): (String, String)) -> Self {
        Self {
            address,
            sub_account: Some(subaccount.into()),
            ..Default::default()
        }
    }
}

impl From<(String, Option<String>)> for AccountIdentifier {
    fn from((address, subaccount): (String, Option<String>)) -> Self {
        Self {
            address,
            sub_account: subaccount.map(|s| s.into()),
            ..Default::default()
        }
    }
}

impl Sortable for AccountIdentifier {
    fn sort(&self) -> Self {
        let mut new = self.clone();
        new.sub_account = new.sub_account.map(|sub| sub.sort());
        new.metadata.sort_keys();
        new
    }
}
