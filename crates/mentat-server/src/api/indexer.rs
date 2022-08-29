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
    ) -> Result<EventsBlocksResponse> {
        MentatError::not_implemented()
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
    ) -> Result<SearchTransactionsResponse> {
        MentatError::not_implemented()
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
        asserter: &Asserter,
        data: Option<UncheckedEventsBlocksRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedEventsBlocksResponse> {
        asserter.events_block_request(data.as_ref())?;
        let resp = self
            .events_blocks(caller, data.unwrap().into(), rpc_caller)
            .await?
            .into();
        // if assert_resp {
        //     events_blocks_response(Some(&resp)).unwrap();
        // }
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    async fn call_search_transactions(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedSearchTransactionsRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedSearchTransactionsResponse> {
        asserter.search_transactions_request(data.as_ref())?;
        let resp = self
            .search_transactions(caller, data.unwrap().into(), rpc_caller)
            .await?
            .into();
        // if assert_resp {
        //     asserter.search_transaction_response(Some(&resp)).unwrap();
        // }
        Ok(Json(resp))
    }
}
