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
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/events/blocks"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }

    async fn search_transactions(
        &self,
        _caller: Caller,
        data: SearchTransactionsRequest,
    ) -> Response<SearchTransactionsResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/construction/submit"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }
}
