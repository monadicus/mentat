#[cfg(debug_assertions)]
use super::log_payload;

use mentat::{
    api::{Caller, CallerIndexerApi, IndexerApi, Response},
    errors::*,
    requests::*,
    responses::*,
};
use reqwest::Client;
use rocket::serde::json::{Json, serde_json};

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

#[rocket::async_trait]
impl CallerIndexerApi for BitcoinIndexerApi {}

#[rocket::async_trait]
impl IndexerApi for BitcoinIndexerApi {
    async fn events_blocks(
        &self,
        _caller: Caller,
        data: EventsBlocksRequest,
    ) -> Response<EventsBlocksResponse> {
        #[cfg(debug_assertions)]
        log_payload(
            "input  /events/blocks",
            serde_json::to_string(&data).unwrap(),
        );
        let resp = match self
            .client
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
        #[cfg(debug_assertions)]
        log_payload("output /events/blocks", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn search_transactions(
        &self,
        _caller: Caller,
        data: SearchTransactionsRequest,
    ) -> Response<SearchTransactionsResponse> {
        #[cfg(debug_assertions)]
        log_payload(
            "input  /construction/submit",
            serde_json::to_string(&data).unwrap(),
        );
        let resp = match self
            .client
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
        #[cfg(debug_assertions)]
        log_payload("output /construction/submit", &out);
        Ok(Json(serde_json::from_str(&out)?))
    }
}
