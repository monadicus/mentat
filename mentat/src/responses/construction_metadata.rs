use indexmap::IndexMap;

use super::*;

/// The ConstructionMetadataResponse returns network-specific metadata used for
/// transaction construction. Optionally, the implementer can return the
/// suggested fee associated with the transaction being constructed. The caller
/// may use this info to adjust the intent of the transaction or to create a
/// transaction with a different account that can pay the suggested fee.
/// Suggested fee is an array in case fee payment must occur in multiple
/// currencies.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ConstructionMetadataResponse {
    pub metadata: IndexMap<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_fee: Option<Vec<Amount>>,
}
