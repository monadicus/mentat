use super::*;
use reqwest::Client;

pub struct BitcoinConstructionApi {
    client: Client,
    url: String,
}

impl Default for BitcoinConstructionApi {
    fn default() -> Self {
        Self {
            client: Client::new(),
            url: "http://127.0.0.1:8080".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl ConstructionApi for BitcoinConstructionApi {
    async fn combine(
        &self,
        _caller: Caller,
        data: ConstructionCombineRequest,
    ) -> Response<ConstructionCombineResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/construction/combine"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn derive(
        &self,
        _caller: Caller,
        data: ConstructionDeriveRequest,
    ) -> Response<ConstructionDeriveResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/construction/derive"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn hash(
        &self,
        _caller: Caller,
        data: ConstructionHashRequest,
    ) -> Response<TransactionIdentifierResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/construction/hash"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn metadata(
        &self,
        _caller: Caller,
        data: ConstructionMetadataRequest,
    ) -> Response<ConstructionMetadataResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/construction/metadata"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn parse(
        &self,
        _caller: Caller,
        data: ConstructionParseRequest,
    ) -> Response<ConstructionParseResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/construction/parse"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn payloads(
        &self,
        _caller: Caller,
        data: ConstructionPayloadsRequest,
    ) -> Response<ConstructionPayloadsResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/construction/payloads"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn preprocess(
        &self,
        _caller: Caller,
        data: ConstructionPreprocessRequest,
    ) -> Response<ConstructionPreprocessResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/construction/preprocess"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
    ) -> Response<TransactionIdentifierResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/construction/submit"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }
}
