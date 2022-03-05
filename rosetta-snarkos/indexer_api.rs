use mentat::{
    api::{Caller, IndexerApi, Response},
    errors::ApiError,
    requests::*,
    responses::*,
};

#[derive(Default)]
pub struct SnarkosIndexerApi;

#[async_trait::async_trait]
impl IndexerApi for SnarkosIndexerApi {
    async fn events_blocks(
        &self,
        _caller: Caller,
        _data: EventsBlocksRequest,
    ) -> Response<EventsBlocksResponse> {
        ApiError::not_implemented()
    }

    async fn search_transactions(
        &self,
        _caller: Caller,
        _data: SearchTransactionsRequest,
    ) -> Response<SearchTransactionsResponse> {
        ApiError::not_implemented()
    }
}
