use super::*;
use reqwest::Client;
#[cfg(feature = "debug")]
use serde::Serialize;
#[cfg(feature = "debug")]
use std::{fs, io::Write};

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

#[cfg(feature = "debug")]
pub fn log_payload<T: Serialize>(route: &str, payload: &T) {
    let t = format!("{}: {}\n", route, serde_json::to_string(payload).unwrap());
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.json")
        .unwrap();
    file.write_all(t.as_bytes()).unwrap();
}

#[async_trait::async_trait]
impl IndexerApi for BitcoinIndexerApi {
    async fn events_blocks(
        &self,
        _caller: Caller,
        data: EventsBlocksRequest,
    ) -> Response<EventsBlocksResponse> {
        #[cfg(feature = "debug")]
        log_payload("input /events/blocks", &data);
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

        let out = resp.json().await?;
        #[cfg(feature = "debug")]
        log_payload("output /events/blocks", &out);
        Ok(Json(out))
    }

    async fn search_transactions(
        &self,
        _caller: Caller,
        data: SearchTransactionsRequest,
    ) -> Response<SearchTransactionsResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /construction/submit", &data);
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

        let out = resp.json().await?;
        #[cfg(feature = "debug")]
        log_payload("output /construction/submit", &out);
        Ok(Json(out))
    }
}
