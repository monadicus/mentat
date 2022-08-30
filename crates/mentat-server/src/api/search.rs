//! Houses the traits for the Rosetta Search API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// SearchAPIServicer defines the api actions for the SearchAPI service
#[axum::async_trait]
pub trait SearchApi {
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

/// SearchAPIRouter defines the required methods for binding the api requests to a responses for the
/// SearchAPI
/// The SearchAPIRouter implementation should parse necessary information from the http request,
/// pass the data to a SearchAPIServicer to perform the required actions, then write the service
/// results to the http response.
#[axum::async_trait]
pub trait SearchApiRouter: SearchApi + Clone + Default {
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
        Ok(Json(resp))
    }
}
