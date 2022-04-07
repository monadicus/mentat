use indexmap::IndexMap;

use super::*;

/// ConstructionDeriveResponse is returned by the /construction/derive endpoint.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ConstructionDeriveResponse {
    /// [DEPRECATED by account_identifier in v1.4.4] Address in network-specific
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The account_identifier uniquely identifies an account within a network.
    /// All fields in the account_identifier are utilized to determine this
    /// uniqueness (including the metadata field, if populated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<AccountIdentifier>,
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
