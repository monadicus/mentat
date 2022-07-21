//! The module defines the `ConstructionPreprocessResponse` response.

use indexmap::IndexMap;

use super::*;

/// [`ConstructionPreprocessResponse`] contains options that will be sent
/// unmodified to `/construction/metadata`. If it is not necessary to make a
/// request to `/construction/metadata`, options should be omitted. Some
/// blockchains require the [`PublicKey`] of particular [`AccountIdentifier`]s
/// to construct a valid transaction. To fetch these [`PublicKey`]s, populate
/// `required_public_keys` with the [`AccountIdentifier`]s associated with the
/// desired [`PublicKey`]s. If it is not necessary to retrieve any
/// [`PublicKey`]s for construction, `required_public_keys` should be omitted.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConstructionPreprocessResponse {
    /// The options that will be sent directly to `/construction/metadata` by
    /// the caller.
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub options: IndexMap<String, Value>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_public_keys: Option<Vec<Option<AccountIdentifier>>>,
}
