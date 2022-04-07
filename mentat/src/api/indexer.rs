//! Houses the traits for the Rosetta Indexer API.
//! These traits are easily overridable for custom
//! implementations.
use super::*;

/// Trait to define the endpoints necessary for the Rosetta Indexer API.
#[axum::async_trait]
pub trait IndexerApi: Default {
    /// `/events/blocks` allows the `_caller` to query a sequence of
    /// [`crate::models::BlockEvent`]s indicating which blocks were added and
    /// removed from storage to reach the current state. Following
    /// [`crate::models::BlockEvent`]s  allows lightweight clients to update
    /// their state without needing to implement their own syncing logic
    /// (like finding the common parent in a reorg). `/events/blocks` is
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

    /// `/events/blocks` allows the `_caller` to query a sequence of
    /// [`crate::models::BlockEvent`]s indicating which blocks were added and
    /// removed from storage to reach the current state. Following
    /// [`crate::models::BlockEvent`]s allows lightweight clients to update
    /// their state without needing to implement their own syncing logic
    /// (like finding the common parent in a reorg). `/events/blocks` is
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

/// Trait to wrap the `IndexerApi`.
/// This trait helps to define default behavior for running the endpoints
/// on different modes.
#[axum::async_trait]
pub trait CallerIndexerApi: Clone + IndexerApi {
    /// This endpoint runs in both offline and online mode.
    async fn call_events_blocks(
        &self,
        caller: Caller,
        data: EventsBlocksRequest,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<EventsBlocksResponse> {
        self.events_blocks(caller, data, rpc_caller).await
    }

    /// This endpoint runs in both offline and online mode.
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
