#[cfg(feature = "debug")]
use super::bitcoin_indexer::debug;
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
        #[cfg(feature = "debug")]
        log_payload("input  /construction/combine", &data);
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

        let out = resp.json().await?;
        #[cfg(feature = "debug")]
        log_payload("output /construction/combine", &out);
        Ok(Json(out))
    }

    async fn derive(
        &self,
        _caller: Caller,
        data: ConstructionDeriveRequest,
    ) -> Response<ConstructionDeriveResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /construction/derive", &data);
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

        let out = resp.json().await?;
        #[cfg(feature = "debug")]
        log_payload("output /construction/derive", &out);
        Ok(Json(out))
    }

    async fn hash(
        &self,
        _caller: Caller,
        data: ConstructionHashRequest,
    ) -> Response<TransactionIdentifierResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /construction/hash", &data);
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

        let out = resp.json().await?;
        #[cfg(feature = "debug")]
        log_payload("output /construction/hash", &out);
        Ok(Json(out))
    }

    async fn metadata(
        &self,
        _caller: Caller,
        data: ConstructionMetadataRequest,
    ) -> Response<ConstructionMetadataResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /construction/metadata", &data);
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

        let out = resp.json().await?;
        #[cfg(feature = "debug")]
        log_payload("output /construction/metadata", &out);
        Ok(Json(out))
    }

    async fn parse(
        &self,
        _caller: Caller,
        data: ConstructionParseRequest,
    ) -> Response<ConstructionParseResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /construction/parse", &data);
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

        let out = resp.json().await?;
        #[cfg(feature = "debug")]
        log_payload("output /construction/parse", &out);
        Ok(Json(out))
    }

    async fn payloads(
        &self,
        _caller: Caller,
        data: ConstructionPayloadsRequest,
    ) -> Response<ConstructionPayloadsResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /construction/payloads", &data);
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

        let out = resp.json().await?;
        #[cfg(feature = "debug")]
        log_payload("output /construction/payloads", &out);
        Ok(Json(out))
    }

    async fn preprocess(
        &self,
        _caller: Caller,
        data: ConstructionPreprocessRequest,
    ) -> Response<ConstructionPreprocessResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /construction/preprocess", &data);
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

        let out = resp.json().await?;
        #[cfg(feature = "debug")]
        log_payload("output /construction/preprocess", &out);
        Ok(Json(out))
    }

    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
    ) -> Response<TransactionIdentifierResponse> {
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
