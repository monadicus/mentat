//! The module defines the `BlockTransaction` model.

use super::*;

/// [`BlockTransaction`] contains a populated [`Transaction`] and the
/// [`BlockIdentifier`] that contains it.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableBlockTransaction {
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<NullableBlockIdentifier>,
    /// [`Transaction`]s contain an array of [`Operation`]s that are
    /// attributable to the same [`TransactionIdentifier`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<NullableTransaction>,
}
