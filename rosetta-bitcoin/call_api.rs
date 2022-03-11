#[cfg(debug_assertions)]
use super::log_payload;

use mentat::{
    api::{CallApi, Caller, CallerCallApi, Response},
    errors::*,
    requests::*,
    responses::*,
};
use reqwest::Client;
use rocket::serde::json::{serde_json, Json};

pub struct BitcoinCallApi {
    client: Client,
    url: String,
}

impl Default for BitcoinCallApi {
    fn default() -> Self {
        Self {
            client: Client::new(),
            url: "http://127.0.0.1:8080".to_string(),
        }
    }
}

#[rocket::async_trait]
impl CallerCallApi for BitcoinCallApi {}

#[rocket::async_trait]
impl CallApi for BitcoinCallApi {
    async fn call(&self, _caller: Caller, data: CallRequest) -> Response<CallResponse> {
        #[cfg(debug_assertions)]
        log_payload("input  /call", serde_json::to_string(&data).unwrap());
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/call"))
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

        let out = resp.text().await?.to_string();
        #[cfg(debug_assertions)]
        log_payload("output /call", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }
}