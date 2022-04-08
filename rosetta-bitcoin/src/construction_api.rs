use mentat::{
    api::{CallerConstructionApi, ConstructionApi},
    axum::async_trait,
};

#[derive(Clone, Default)]
pub struct BitcoinConstructionApi;

#[async_trait]
impl CallerConstructionApi for BitcoinConstructionApi {}

#[async_trait]
impl ConstructionApi for BitcoinConstructionApi {
    // async fn combine(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionCombineRequest,
    //     client: Client,
    // ) -> MentatResponse<ConstructionCombineResponse> {
    //     todo!()
    // }

    // async fn derive(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionDeriveRequest,
    //     client: Client,
    // ) -> MentatResponse<ConstructionDeriveResponse> {
    //     todo!()
    // }

    // async fn hash(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionHashRequest,
    //     client: Client,
    // ) -> MentatResponse<TransactionIdentifierResponse> {
    //     todo!()
    // }

    // async fn metadata(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionMetadataRequest,
    //     client: Client,
    // ) -> MentatResponse<ConstructionMetadataResponse> {
    //     todo!()
    // }

    // async fn parse(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionParseRequest,
    //     client: Client,
    // ) -> MentatResponse<ConstructionParseResponse> {
    //     todo!()
    // }

    // async fn payloads(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionPayloadsRequest,
    //     client: Client,
    // ) -> MentatResponse<ConstructionPayloadsResponse> {
    //     todo!()
    // }

    // async fn preprocess(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionPreprocessRequest,
    //     client: Client,
    // ) -> MentatResponse<ConstructionPreprocessResponse> {
    //     todo!()
    // }

    // async fn submit(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionSubmitRequest,
    //     client: Client,
    // ) -> MentatResponse<TransactionIdentifierResponse> {
    //     todo!()
    // }
}
