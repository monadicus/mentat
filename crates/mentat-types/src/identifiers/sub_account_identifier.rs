//! The module defines the `SubAccountIdentifier`.

use std::mem::size_of_val;

use indexmap::IndexMap;

use super::*;

/// An account may have state specific to a contract address (ERC-20 token)
/// and/or a stake (delegated balance). The `sub_account_identifier` should
/// specify which state (if applicable) an account instantiation refers to.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct SubAccountIdentifier {
    /// The `SubAccount` address may be a cryptographic value or some other
    /// identifier (ex: bonded) that uniquely specifies a `SubAccount`.
    pub address: String,
    /// If the `SubAccount` address is not sufficient to uniquely specify a
    /// `SubAccount`, any other identifying information can be stored here. It
    /// is important to note that two `SubAccounts` with identical addresses
    /// but differing metadata will not be considered equal by clients.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}

impl From<String> for SubAccountIdentifier {
    fn from(address: String) -> Self {
        Self {
            address,
            ..Default::default()
        }
    }
}

impl Sortable for SubAccountIdentifier {
    fn sort(&self) -> Self {
        let mut new = self.clone();
        new.metadata.sort_keys();
        new
    }
}

impl EstimateSize for SubAccountIdentifier {
    fn estimated_size(&self) -> usize {
        size_of_val(self)
            + size_of_val(self.address.as_str())
            + estimated_metadata_size(&self.metadata)
    }
}
