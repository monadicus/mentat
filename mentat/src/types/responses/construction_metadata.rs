//! The module defines the `ConstructionMetadataResponse` response.

use indexmap::IndexMap;

use super::*;

/// The [`ConstructionMetadataResponse`] returns network-specific metadata used
/// for transaction construction. Optionally, the implementer can return the
/// suggested fee associated with the transaction being constructed. The caller
/// may use this info to adjust the intent of the transaction or to create a
/// transaction with a different account that can pay the suggested fee.
/// Suggested fee is an array in case fee payment must occur in multiple
/// currencies.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConstructionMetadataResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    pub metadata: IndexMap<String, Value>,
    /// The optional suggested fees for the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_fee: Option<Vec<Option<Amount>>>,
}
