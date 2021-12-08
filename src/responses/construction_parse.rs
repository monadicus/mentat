use indexmap::IndexMap;

use super::*;

/// ConstructionParseResponse contains an array of operations that occur in a transaction blob. This should match the array of operations provided to /construction/preprocess and /construction/payloads.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConstructionParseResponse {
    pub operations: Vec<Operation>,
    /// [DEPRECATED by account_identifier_signers in v1.4.4] All signers (addresses) of a particular transaction. If the transaction is unsigned, it should be empty.
    pub signers: Option<Vec<String>>,
    pub account_identifier_signers: Option<Vec<AccountIdentifier>>,
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}