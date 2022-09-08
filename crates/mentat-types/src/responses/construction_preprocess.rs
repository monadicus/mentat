//! The module defines the `ConstructionPreprocessResponse` response.

use super::*;

/// [`ConstructionPreprocessResponse`] contains options that will be sent
/// unmodified to `/construction/metadata`. If it is not necessary to make a
/// request to `/construction/metadata`, options should be omitted. Some
/// blockchains require the [`PublicKey`] of particular [`AccountIdentifier`]s
/// to construct a valid transaction. To fetch these [`PublicKey`]s, populate
/// `required_public_keys` with the [`AccountIdentifier`]s associated with the
/// desired [`PublicKey`]s. If it is not necessary to retrieve any
/// [`PublicKey`]s for construction, `required_public_keys` should be omitted.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedConstructionPreprocessResponse {
    /// The options that will be sent directly to `/construction/metadata` by
    /// the caller.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub options: Metadata,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub required_public_keys: Vec<Option<AccountIdentifier>>,
}
