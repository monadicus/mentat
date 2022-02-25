use super::*;

#[async_trait::async_trait]
pub trait IndexerApi: Send + Sync {
    /// /events/blocks allows the caller to query a sequence of BlockEvents indicating which blocks were added and removed from storage to reach the current state. Following BlockEvents allows lightweight clients to update their state without needing to implement their own syncing logic (like finding the common parent in a reorg). /events/blocks is considered an "indexer" endpoint and Rosetta implementations are not required to complete it to adhere to the Rosetta spec. However, any Rosetta "indexer" MUST support this endpoint.
    async fn events_blocks(
        &self,
        caller: Caller,
        data: EventsBlocksRequest,
    ) -> Response<EventsBlocksResponse>;

    /// /events/blocks allows the caller to query a sequence of BlockEvents indicating which blocks were added and removed from storage to reach the current state. Following BlockEvents allows lightweight clients to update their state without needing to implement their own syncing logic (like finding the common parent in a reorg). /events/blocks is considered an "indexer" endpoint and Rosetta implementations are not required to complete it to adhere to the Rosetta spec. However, any Rosetta "indexer" MUST support this endpoint.
    async fn search_transactions(
        &self,
        caller: Caller,
        data: SearchTransactionsRequest,
    ) -> Response<SearchTransactionsResponse>;
}
