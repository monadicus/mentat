//! The module defines the `Transaction` model.

use indexmap::IndexMap;
use mentat_macros::Nullable;

use super::*;

/// [`Transaction`]s contain an array of [`Operation`]s that are attributable to
/// the same [`TransactionIdentifier`].
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableTransaction {
    /// The [`TransactionIdentifier`] uniquely identifies a transaction in a
    /// particular network and block or in the mempool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_identifier: Option<TransactionIdentifier>,
    /// The list of [`Operation`] on the transaction.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub operations: Vec<Option<NullableOperation>>,
    /// A optional list of `RelatedTransaction` related to this transaction.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub related_transactions: Vec<Option<NullableRelatedTransaction>>,
    /// `Transaction`s that are related to other transactions (like a
    /// cross-shard transaction) should include the tranaction_identifier of
    /// these transactions in the metadata.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: IndexMap<String, Value>,
}
