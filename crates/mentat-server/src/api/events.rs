//! Houses the traits for the Rosetta Events API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// EventsAPIServicer defines the api actions for the EventsAPI service
#[axum::async_trait]
pub trait EventsApi {
    /// the caller used to interact with the underlying node
    type NodeCaller: Send + Sync;

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

/// EventsAPIRouter defines the required methods for binding the api requests to
/// a responses for the EventsAPI
/// The EventsAPIRouter implementation should parse necessary information from
/// the http request, pass the data to a EventsAPIServicer to perform the
/// required actions, then write the service results to the http response.
#[axum::async_trait]
pub trait EventsApiRouter: EventsApi + Clone + Default {
    /// This endpoint runs in both offline and online mode.
    async fn call_events_blocks(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedEventsBlocksRequest>,
        _mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> MentatResponse<UncheckedEventsBlocksResponse> {
        asserter.events_block_request(data.as_ref())?;
        let resp = self
            .events_blocks(caller, data.unwrap().into(), node_caller)
            .await?
            .into();
        // if assert_resp {
        //     events_blocks_response(Some(&resp)).unwrap();
        // }
        Ok(Json(resp))
    }
}
