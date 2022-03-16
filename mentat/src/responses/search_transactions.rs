use super::*;

/// SearchTransactionsResponse contains an ordered collection of
/// BlockTransactions that match the query in SearchTransactionsRequest. These
/// BlockTransactions are sorted from most recent block to oldest block.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SearchTransactionsResponse {
    /// transactions is an array of BlockTransactions sorted by most recent
    /// BlockIdentifier (meaning that transactions in recent blocks appear
    /// first). If there are many transactions for a particular search,
    /// transactions may not contain all matching transactions. It is up to the
    /// caller to paginate these transactions using the max_block field.
    pub transactions: Vec<BlockTransaction>,
    /// total_count is the number of results for a given search. Callers
    /// typically use this value to concurrently fetch results by offset or to
    /// display a virtual page number associated with results.
    pub total_count: u64,
    /// next_offset is the next offset to use when paginating through
    /// transaction results. If this field is not populated, there are no more
    /// transactions to query.
    pub next_offset: Option<u64>,
}
