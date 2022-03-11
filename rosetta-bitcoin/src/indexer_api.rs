use mentat::{
    api::{Caller, CallerIndexerApi, IndexerApi, MentantResponse},
    async_trait,
    errors::*,
    requests::*,
    responses::*,
    serde_json, Client, Json,
};

pub struct BitcoinIndexerApi {
    url: String,
}

impl Default for BitcoinIndexerApi {
    fn default() -> Self {
        Self {
            url: "http://127.0.0.1:8080".to_string(),
        }
    }
}

#[async_trait]
impl CallerIndexerApi for BitcoinIndexerApi {}

#[async_trait]
impl IndexerApi for BitcoinIndexerApi {
    async fn events_blocks(
        &self,
        _caller: Caller,
        data: EventsBlocksRequest,
        client: Client,
    ) -> MentantResponse<EventsBlocksResponse> {
        let resp = match client
            .post(&format!("{}{}", self.url, "/events/blocks"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                return Err(match serde_json::from_str(&e.to_string()) {
                    Ok(s) => MentatError::Internal(s),
                    Err(_) => MentatError::from(format!("unhandled rosetta-bitcoin error: {}", e)),
                })
            }
        };

        let out = resp.text().await?;
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn search_transactions(
        &self,
        _caller: Caller,
        data: SearchTransactionsRequest,
        client: Client,
    ) -> MentantResponse<SearchTransactionsResponse> {
        let resp = match client
            .post(&format!("{}{}", self.url, "/construction/submit"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                return Err(match serde_json::from_str(&e.to_string()) {
                    Ok(s) => MentatError::Internal(s),
                    Err(_) => MentatError::from(format!("unhandled rosetta-bitcoin error: {}", e)),
                })
            }
        };

        let out = resp.text().await?;
        Ok(Json(serde_json::from_str(&out)?))
    }
}
