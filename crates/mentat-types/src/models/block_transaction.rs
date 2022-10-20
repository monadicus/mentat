//! The module defines the `BlockTransaction` model.

use super::*;

/// [`BlockTransaction`] contains a populated [`Transaction`] and the
/// [`BlockIdentifier`] that contains it.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default, deny_unknown_fields)]
pub struct UncheckedBlockTransaction {
    /// The [`BlockIdentifier`] uniquely identifies a block in a particular
    /// network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<UncheckedBlockIdentifier>,
    /// [`Transaction`]s contain an array of [`Operation`]s that are
    /// attributable to the same [`TransactionIdentifier`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<UncheckedTransaction>,
}
