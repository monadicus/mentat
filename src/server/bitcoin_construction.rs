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
        log_payload("/construction/combine", &data);
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

        Ok(Json(resp.json().await?))
    }

    async fn derive(
        &self,
        _caller: Caller,
        data: ConstructionDeriveRequest,
    ) -> Response<ConstructionDeriveResponse> {
        #[cfg(feature = "debug")]
        log_payload("/construction/derive", &data);
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

        Ok(Json(resp.json().await?))
    }

    async fn hash(
        &self,
        _caller: Caller,
        data: ConstructionHashRequest,
    ) -> Response<TransactionIdentifierResponse> {
        #[cfg(feature = "debug")]
        log_payload("/construction/hash", &data);
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

        Ok(Json(resp.json().await?))
    }

    async fn metadata(
        &self,
        _caller: Caller,
        data: ConstructionMetadataRequest,
    ) -> Response<ConstructionMetadataResponse> {
        #[cfg(feature = "debug")]
        log_payload("/construction/metadata", &data);
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

        Ok(Json(resp.json().await?))
    }

    async fn parse(
        &self,
        _caller: Caller,
        data: ConstructionParseRequest,
    ) -> Response<ConstructionParseResponse> {
        #[cfg(feature = "debug")]
        log_payload("/construction/parse", &data);
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

        Ok(Json(resp.json().await?))
    }

    async fn payloads(
        &self,
        _caller: Caller,
        data: ConstructionPayloadsRequest,
    ) -> Response<ConstructionPayloadsResponse> {
        #[cfg(feature = "debug")]
        log_payload("/construction/payloads", &data);
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

        Ok(Json(resp.json().await?))
    }

    async fn preprocess(
        &self,
        _caller: Caller,
        data: ConstructionPreprocessRequest,
    ) -> Response<ConstructionPreprocessResponse> {
        #[cfg(feature = "debug")]
        log_payload("/construction/preprocess", &data);
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

        Ok(Json(resp.json().await?))
    }

    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
    ) -> Response<TransactionIdentifierResponse> {
        #[cfg(feature = "debug")]
        log_payload("/construction/submit", &data);
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

        Ok(Json(resp.json().await?))
    }
}
