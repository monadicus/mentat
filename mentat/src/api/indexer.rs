use super::*;

#[axum::async_trait]
pub trait IndexerApi: Default {
    /// /events/blocks allows the _caller to query a sequence of BlockEvents
    /// indicating which blocks were added and removed from storage to reach the
    /// current state. Following BlockEvents allows lightweight clients to
    /// update their state without needing to implement their own syncing logic
    /// (like finding the common parent in a reorg). /events/blocks is
    /// considered an "indexer" endpoint and Rosetta implementations are not
    /// required to complete it to adhere to the Rosetta spec. However, any
    /// Rosetta "indexer" MUST support this endpoint.
    async fn events_blocks(
        &self,
        _caller: Caller,
        _data: EventsBlocksRequest,
        _rpc_caller: RpcCaller,
    ) -> MentatResponse<EventsBlocksResponse> {
        ApiError::not_implemented()
    }

    /// /events/blocks allows the _caller to query a sequence of BlockEvents
    /// indicating which blocks were added and removed from storage to reach the
    /// current state. Following BlockEvents allows lightweight clients to
    /// update their state without needing to implement their own syncing logic
    /// (like finding the common parent in a reorg). /events/blocks is
    /// considered an "indexer" endpoint and Rosetta implementations are not
    /// required to complete it to adhere to the Rosetta spec. However, any
    /// Rosetta "indexer" MUST support this endpoint.
    async fn search_transactions(
        &self,
        _caller: Caller,
        _data: SearchTransactionsRequest,
        _rpc_caller: RpcCaller,
    ) -> MentatResponse<SearchTransactionsResponse> {
        ApiError::not_implemented()
    }
}

#[axum::async_trait]
pub trait CallerIndexerApi: Clone + IndexerApi {
    async fn call_events_blocks(
        &self,
        caller: Caller,
        data: EventsBlocksRequest,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<EventsBlocksResponse> {
        self.events_blocks(caller, data, rpc_caller).await
    }

    async fn call_search_transactions(
        &self,
        caller: Caller,
        data: SearchTransactionsRequest,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<SearchTransactionsResponse> {
        self.search_transactions(caller, data, rpc_caller).await
    }
}
