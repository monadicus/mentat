use mentat::{
    api::{Caller, ConstructionApi, Response},
    errors::ApiError,
    requests::*,
    responses::*,
};

#[derive(Default)]
pub struct SnarkosConstructionApi;

#[async_trait::async_trait]
impl ConstructionApi for SnarkosConstructionApi {
    async fn combine(
        &self,
        _caller: Caller,
        _data: ConstructionCombineRequest,
    ) -> Response<ConstructionCombineResponse> {
        ApiError::not_implemented()
    }

    async fn derive(
        &self,
        _caller: Caller,
        _data: ConstructionDeriveRequest,
    ) -> Response<ConstructionDeriveResponse> {
        ApiError::not_implemented()
    }

    async fn hash(
        &self,
        _caller: Caller,
        _data: ConstructionHashRequest,
    ) -> Response<TransactionIdentifierResponse> {
        ApiError::not_implemented()
    }

    async fn metadata(
        &self,
        _caller: Caller,
        _data: ConstructionMetadataRequest,
    ) -> Response<ConstructionMetadataResponse> {
        ApiError::not_implemented()
    }

    async fn parse(
        &self,
        _caller: Caller,
        _data: ConstructionParseRequest,
    ) -> Response<ConstructionParseResponse> {
        ApiError::not_implemented()
    }

    async fn payloads(
        &self,
        _caller: Caller,
        _data: ConstructionPayloadsRequest,
    ) -> Response<ConstructionPayloadsResponse> {
        ApiError::not_implemented()
    }

    async fn preprocess(
        &self,
        _caller: Caller,
        _data: ConstructionPreprocessRequest,
    ) -> Response<ConstructionPreprocessResponse> {
        ApiError::not_implemented()
    }

    async fn submit(
        &self,
        _caller: Caller,
        _data: ConstructionSubmitRequest,
    ) -> Response<TransactionIdentifierResponse> {
        ApiError::not_implemented()
    }
}
