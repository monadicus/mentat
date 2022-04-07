//! The module defines the `ConstructionDeriveResponse` response.

use indexmap::IndexMap;

use super::*;

/// `ConstructionDeriveResponse` is returned by the `/construction/derive`
/// endpoint.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConstructionDeriveResponse {
    /// [DEPRECATED by `account_identifier` in v1.4.4] Address in
    /// network-specific format.
    pub address: Option<String>,
    /// The `AccountIdentifier` uniquely identifies an account within a network.
    /// All fields in the `account_identifier` are utilized to determine this
    /// uniqueness (including the metadata field, if populated).
    pub account_identifier: Option<AccountIdentifier>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
