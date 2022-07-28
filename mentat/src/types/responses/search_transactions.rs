//! The module defines the `SearchTransactionsResponse` response.

use super::*;

/// [`SearchTransactionsResponse`] contains an ordered collection of
/// [`BlockTransaction`]s that match the query in
/// [`crate::requests::SearchTransactionsRequest`]. These [`BlockTransaction`]s
/// are sorted from most recent block to oldest block.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SearchTransactionsResponse {
    /// transactions is an array of [`BlockTransaction`]s sorted by most recent
    /// [`BlockIdentifier`] (meaning that transactions in recent blocks appear
    /// first). If there are many transactions for a particular search,
    /// transactions may not contain all matching transactions. It is up to the
    /// caller to paginate these transactions using the `max_block` field.
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub transactions: Vec<Option<BlockTransaction>>,
    /// `total_count` is the number of results for a given search. Callers
    /// typically use this value to concurrently fetch results by offset or to
    /// display a virtual page number associated with results.
    pub total_count: i64,
    /// `next_offset` is the next offset to use when paginating through
    /// transaction results. If this field is not populated, there are no more
    /// transactions to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
}
