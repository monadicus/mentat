#[cfg(feature = "debug")]
use super::bitcoin_indexer::debug;
use super::*;
use reqwest::Client;

pub struct BitcoinDataApi {
    client: Client,
    url: String,
}

impl Default for BitcoinDataApi {
    fn default() -> Self {
        Self {
            client: Client::new(),
            url: "http://127.0.0.1:8080".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl DataApi for BitcoinDataApi {
    async fn network_list(
        &self,
        _caller: Caller,
        data: MetadataRequest,
    ) -> Response<NetworkListResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /network/list", &data);
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/network/list"))
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
        log_payload("output /network/list", &out);
        Ok(Json(out))
    }

    async fn network_options(
        &self,
        _caller: Caller,
        data: NetworkRequest,
    ) -> Response<NetworkOptionsResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /network/options", &data);
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/network/options"))
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
        log_payload("output /network/options", &out);
        Ok(Json(out))
    }

    async fn network_status(
        &self,
        _caller: Caller,
        data: NetworkRequest,
    ) -> Response<NetworkStatusResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /network/status", &data);
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/network/status"))
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
        log_payload("output /network/status", &out);
        Ok(Json(out))
    }

    async fn account_balance(
        &self,
        _caller: Caller,
        data: AccountBalanceRequest,
    ) -> Response<AccountBalanceResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /account/balance", &data);
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/account/balance"))
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
        log_payload("output /account/balance", &out);
        Ok(Json(out))
    }

    async fn account_coins(
        &self,
        _caller: Caller,
        data: AccountCoinsRequest,
    ) -> Response<AccountCoinsResponse> {
        #[cfg(feature = "debug")]
        log_payload("input /account/coins", &data);
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/account/coins"))
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
        log_payload("output /account/coins", &out);
        Ok(Json(out))
    }

    async fn block(&self, _caller: Caller, data: BlockRequest) -> Response<BlockResponse> {
        #[cfg(feature = "debug")]
        log_payload("input /block", &data);
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/block"))
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
        log_payload("output /block", &out);
        Ok(Json(out))
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
    ) -> Response<BlockTransactionResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /block/transaction", &data);
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/block/transaction"))
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
        log_payload("output /block/transaction", &out);
        Ok(Json(out))
    }

    async fn mempool(&self, _caller: Caller, data: NetworkRequest) -> Response<MempoolResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /mempool", &data);
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/mempool"))
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
        log_payload("output /mempool", &out);
        Ok(Json(out))
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        data: MempoolTransactionRequest,
    ) -> Response<MempoolTransactionResponse> {
        #[cfg(feature = "debug")]
        log_payload("input  /mempool/transaction", &data);
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/mempool/transaction"))
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
        log_payload("output /mempool/transaction", &out);
        Ok(Json(out))
    }
}
