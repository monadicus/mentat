use mentat::{
    api::{Caller, CallerConstructionApi, ConstructionApi, MentantResponse},
    async_trait,
    errors::*,
    requests::*,
    responses::*,
    serde_json, Json,
};
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

#[async_trait]
impl CallerConstructionApi for BitcoinConstructionApi {}

#[async_trait]
impl ConstructionApi for BitcoinConstructionApi {
    async fn combine(
        &self,
        _caller: Caller,
        data: ConstructionCombineRequest,
    ) -> MentantResponse<ConstructionCombineResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/construction/combine"))
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
        mentat::tracing::debug!("output /construction/combine {out}");
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn derive(
        &self,
        _caller: Caller,
        data: ConstructionDeriveRequest,
    ) -> MentantResponse<ConstructionDeriveResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/construction/derive"))
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
        mentat::tracing::debug!("output /construction/derive {out}");
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn hash(
        &self,
        _caller: Caller,
        data: ConstructionHashRequest,
    ) -> MentantResponse<TransactionIdentifierResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/construction/hash"))
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
        mentat::tracing::debug!("output /construction/hash {out}");
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn metadata(
        &self,
        _caller: Caller,
        data: ConstructionMetadataRequest,
    ) -> MentantResponse<ConstructionMetadataResponse> {
        #[cfg(debug_assertions)]
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/construction/metadata"))
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
        mentat::tracing::debug!("output /construction/metadata {out}");
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn parse(
        &self,
        _caller: Caller,
        data: ConstructionParseRequest,
    ) -> MentantResponse<ConstructionParseResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/construction/parse"))
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
        mentat::tracing::debug!("output /construction/parse {out}");
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn payloads(
        &self,
        _caller: Caller,
        data: ConstructionPayloadsRequest,
    ) -> MentantResponse<ConstructionPayloadsResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/construction/payloads"))
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
        mentat::tracing::debug!("output /construction/payloads {out}");
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn preprocess(
        &self,
        _caller: Caller,
        data: ConstructionPreprocessRequest,
    ) -> MentantResponse<ConstructionPreprocessResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/construction/preprocess"))
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
        mentat::tracing::debug!("output /construction/preprocess {out}");
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
    ) -> MentantResponse<TransactionIdentifierResponse> {
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
        mentat::tracing::debug!("output /construction/submit {out}");
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }
}
