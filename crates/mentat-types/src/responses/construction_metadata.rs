//! The module defines the `ConstructionMetadataResponse` response.

use super::*;

/// The [`ConstructionMetadataResponse`] returns network-specific metadata used
/// for transaction construction. Optionally, the implementer can return the
/// suggested fee associated with the transaction being constructed. The caller
/// may use this info to adjust the intent of the transaction or to create a
/// transaction with a different account that can pay the suggested fee.
/// Suggested fee is an array in case fee payment must occur in multiple
/// currencies.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedConstructionMetadataResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// The optional suggested fees for the response.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub suggested_fee: Vec<Option<UncheckedAmount>>,
}
