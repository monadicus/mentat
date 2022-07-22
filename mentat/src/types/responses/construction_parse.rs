//! The module defines the `ConstructionParseResponse` response.

use indexmap::IndexMap;

use super::*;

/// [`ConstructionParseResponse`] contains an array of operations that occur in
/// a transaction blob. This should match the array of operations provided to
/// `/construction/preprocess` and `/construction/payloads`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ConstructionParseResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub operations: Vec<Option<Operation>>,
    /// [DEPRECATED by `account_identifier_signers` in v1.4.4] All signers
    /// (addresses) of a particular transaction. If the transaction is unsigned,
    /// it should be empty.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub signers: Vec<String>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub account_identifier_signers: Vec<Option<AccountIdentifier>>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
