use mentat::{
    api::{CallApi, Caller, CallerCallApi, MentantResponse},
    async_trait,
    errors::*,
    requests::*,
    responses::*,
    serde_json, Json,
};
use reqwest::Client;

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

#[async_trait]
impl CallerCallApi for BitcoinCallApi {}

#[async_trait]
impl CallApi for BitcoinCallApi {
    async fn call(&self, _caller: Caller, data: CallRequest) -> MentantResponse<CallResponse> {
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        mentat::tracing::debug!("output /call {out}");
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }
}
