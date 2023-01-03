//! Houses the traits for the Rosetta Search API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// SearchAPIServicer defines the api actions for the SearchAPI service
#[axum::async_trait]
pub trait SearchApi: Clone + Debug + Default + Send + Sync {
    /// the caller used to interact with the underlying node
    type NodeCaller: Clone + Debug + Send + Sync + 'static;

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
        _node_caller: &Self::NodeCaller,
    ) -> Result<SearchTransactionsResponse> {
        MentatError::not_implemented()
    }
}

crate::router!(SearchApiRouter, SearchApi);

impl<Api: SearchApi> SearchApiRouter<Api> {
    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/search/transactions")]
    async fn call_search_transactions(
        &self,
        caller: Caller,
        data: Option<UncheckedSearchTransactionsRequest>,
    ) -> MentatResponse<UncheckedSearchTransactionsResponse> {
        self.asserter.search_transactions_request(data.as_ref())?;
        let resp = self
            .api
            .search_transactions(caller, data.unwrap().into(), &self.node_caller)
            .await?
            .into();
        Ok(Json(resp))
    }
}

impl<Api> ToRouter for SearchApiRouter<Api>
where
    Api: SearchApi + 'static,
{
    fn to_router<CustomConfig: NodeConf>(self) -> axum::Router<Arc<AppState<CustomConfig>>> {
        axum::Router::new().route(
            "/transactions",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 Json(req_data): Json<Option<UncheckedSearchTransactionsRequest>>| async move {
                    self.call_search_transactions(Caller { ip }, req_data).await
                },
            ),
        )
    }
}
