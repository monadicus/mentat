#[cfg(debug_assertions)]
use super::log_payload;

use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentantResponse},
    async_trait,
    errors::*,
    requests::*,
    responses::*,
    serde_json, Json,
};
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

#[async_trait]
impl CallerDataApi for BitcoinDataApi {}

#[async_trait]
impl DataApi for BitcoinDataApi {
    async fn network_list(
        &self,
        _caller: Caller,
        data: MetadataRequest,
    ) -> MentantResponse<NetworkListResponse> {
        #[cfg(debug_assertions)]
        log_payload(
            "input  /network/list",
            serde_json::to_string(&data).unwrap(),
        );
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        log_payload("output /network/list", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn network_options(
        &self,
        _caller: Caller,
        data: NetworkRequest,
    ) -> MentantResponse<NetworkOptionsResponse> {
        #[cfg(debug_assertions)]
        log_payload(
            "input  /network/options",
            serde_json::to_string(&data).unwrap(),
        );
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        log_payload("output /network/options", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn network_status(
        &self,
        _caller: Caller,
        data: NetworkRequest,
    ) -> MentantResponse<NetworkStatusResponse> {
        #[cfg(debug_assertions)]
        log_payload(
            "input  /network/status",
            serde_json::to_string(&data).unwrap(),
        );
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        log_payload("output /network/status", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn account_balance(
        &self,
        _caller: Caller,
        data: AccountBalanceRequest,
    ) -> MentantResponse<AccountBalanceResponse> {
        #[cfg(debug_assertions)]
        log_payload(
            "input  /account/balance",
            serde_json::to_string(&data).unwrap(),
        );
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        log_payload("output /account/balance", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn account_coins(
        &self,
        _caller: Caller,
        data: AccountCoinsRequest,
    ) -> MentantResponse<AccountCoinsResponse> {
        #[cfg(debug_assertions)]
        log_payload(
            "input  /account/coins",
            serde_json::to_string(&data).unwrap(),
        );
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        log_payload("output /account/coins", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn block(&self, _caller: Caller, data: BlockRequest) -> MentantResponse<BlockResponse> {
        #[cfg(debug_assertions)]
        log_payload("input  /block", serde_json::to_string(&data).unwrap());
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        log_payload("output /block", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
    ) -> MentantResponse<BlockTransactionResponse> {
        #[cfg(debug_assertions)]
        log_payload(
            "input  /block/transaction",
            serde_json::to_string(&data).unwrap(),
        );
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        log_payload("output /block/transaction", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn mempool(
        &self,
        _caller: Caller,
        data: NetworkRequest,
    ) -> MentantResponse<MempoolResponse> {
        #[cfg(debug_assertions)]
        log_payload("input  /mempool", serde_json::to_string(&data).unwrap());
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        log_payload("output /mempool", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        data: MempoolTransactionRequest,
    ) -> MentantResponse<MempoolTransactionResponse> {
        #[cfg(debug_assertions)]
        log_payload(
            "input  /mempool/transaction",
            serde_json::to_string(&data).unwrap(),
        );
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

        let out = resp.text().await?;
        #[cfg(debug_assertions)]
        log_payload("output /mempool/transaction", &out);
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }
}
