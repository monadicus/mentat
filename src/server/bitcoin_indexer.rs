use super::*;
use reqwest::Client;

pub struct BitcoinIndexerApi {
    client: Client,
    url: String,
}

impl Default for BitcoinIndexerApi {
    fn default() -> Self {
        Self {
            client: Client::new(),
            url: "http://127.0.0.1:8080".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl IndexerApi for BitcoinIndexerApi {
    async fn events_blocks(
        &self,
        _caller: Caller,
        data: EventsBlocksRequest,
    ) -> Response<EventsBlocksResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/events/blocks"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn search_transactions(
        &self,
        _caller: Caller,
        data: SearchTransactionsRequest,
    ) -> Response<SearchTransactionsResponse> {
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
