use super::*;

pub struct DummyConstructionApi;

#[async_trait::async_trait]
impl ConstructionApi for DummyConstructionApi {
    async fn combine(
        &self,
        _caller: Caller,
        _data: ConstructionCombineRequest,
    ) -> Response<ConstructionCombineResponse> {
        Err(ApiError::not_implemented())
    }

    async fn derive(
        &self,
        _caller: Caller,
        _data: ConstructionDeriveRequest,
    ) -> Response<ConstructionDeriveResponse> {
        Err(ApiError::not_implemented())
    }

    async fn hash(
        &self,
        _caller: Caller,
        _data: ConstructionHashRequest,
    ) -> Response<TransactionIdentifierResponse> {
        Err(ApiError::not_implemented())
    }

    async fn metadata(
        &self,
        _caller: Caller,
        _data: ConstructionMetadataRequest,
    ) -> Response<ConstructionMetadataResponse> {
        Err(ApiError::not_implemented())
    }

    async fn parse(
        &self,
        _caller: Caller,
        _data: ConstructionParseRequest,
    ) -> Response<ConstructionParseResponse> {
        Err(ApiError::not_implemented())
    }

    async fn payloads(
        &self,
        _caller: Caller,
        _data: ConstructionPayloadsRequest,
    ) -> Response<ConstructionPayloadsResponse> {
        Err(ApiError::not_implemented())
    }

    async fn preprocess(
        &self,
        _caller: Caller,
        _data: ConstructionPreprocessRequest,
    ) -> Response<ConstructionPreprocessResponse> {
        Err(ApiError::not_implemented())
    }

    async fn submit(
        &self,
        _caller: Caller,
        _data: ConstructionSubmitRequest,
    ) -> Response<TransactionIdentifierResponse> {
        Err(ApiError::not_implemented())
    }
}
