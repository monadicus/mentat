//! Houses the traits for the Rosetta Events API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// EventsAPIServicer defines the api actions for the EventsAPI service
#[axum::async_trait]
pub trait EventsApi: Clone + Debug + Default + Send + Sync {
    /// the caller used to interact with the underlying node
    type NodeCaller: Clone + Debug + Send + Sync + 'static;

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
        _node_caller: &Self::NodeCaller,
    ) -> Result<EventsBlocksResponse> {
        MentatError::not_implemented()
    }
}

crate::router!(EventsApiRouter, EventsApi);

impl<Api: EventsApi> EventsApiRouter<Api> {
    /// This endpoint runs in both offline and online mode.
    #[tracing::instrument(name = "/events/blocks")]
    async fn call_events_blocks(
        &self,
        caller: Caller,
        data: Option<UncheckedEventsBlocksRequest>,
    ) -> MentatResponse<UncheckedEventsBlocksResponse> {
        self.asserter.events_block_request(data.as_ref())?;
        let resp = self
            .api
            .events_blocks(caller, data.unwrap().into(), &self.node_caller)
            .await?
            .into();
        // if assert_resp {
        //     events_blocks_response(Some(&resp)).unwrap();
        // }
        Ok(Json(resp))
    }
}

impl<Api> ToRouter for EventsApiRouter<Api>
where
    Api: EventsApi + 'static,
{
    fn to_router<CustomConfig: NodeConf>(self) -> axum::Router<Arc<AppState<CustomConfig>>> {
        axum::Router::new().route(
            "/blocks",
            axum::routing::post(
                |ConnectInfo(ip): ConnectInfo<::std::net::SocketAddr>,
                 Json(req_data): Json<Option<UncheckedEventsBlocksRequest>>| async move {
                    self.call_events_blocks(Caller { ip }, req_data).await
                },
            ),
        )
    }
}
