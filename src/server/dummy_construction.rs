use super::*;

pub struct DummyConstructionApi;

#[async_trait::async_trait]
impl ConstructionApi for DummyConstructionApi {
    async fn combine(&self, _caller: Caller, _data: ConstructionCombineRequest) -> Result<ConstructionCombineResponse> {
        not_implemented()
    }

    async fn derive(&self, _caller: Caller, _data: ConstructionDeriveRequest) -> Result<ConstructionDeriveResponse> {
        not_implemented()
    }

    async fn hash(&self, _caller: Caller, _data: ConstructionHashRequest) -> Result<TransactionIdentifierResponse> {
        not_implemented()
    }

    async fn metadata(&self, _caller: Caller, _data: ConstructionMetadataRequest) -> Result<ConstructionMetadataResponse> {
        not_implemented()
    }

    async fn parse(&self, _caller: Caller, _data: ConstructionParseRequest) -> Result<ConstructionParseResponse> {
        not_implemented()
    }

    async fn payloads(&self, _caller: Caller, _data: ConstructionPayloadsRequest) -> Result<ConstructionPayloadsResponse> {
        not_implemented()
    }

    async fn preprocess(&self, _caller: Caller, _data: ConstructionPreprocessRequest) -> Result<ConstructionPreprocessResponse> {
        not_implemented()
    }

    async fn submit(&self, _caller: Caller, _data: ConstructionSubmitRequest) -> Result<TransactionIdentifierResponse> {
        not_implemented()
    }
}

