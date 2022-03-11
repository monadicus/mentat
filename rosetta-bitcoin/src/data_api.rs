use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentantResponse},
    async_trait,
    errors::*,
    requests::*,
    responses::*,
    serde_json, Client, Json,
};

pub struct BitcoinDataApi {
    url: String,
}

impl Default for BitcoinDataApi {
    fn default() -> Self {
        Self {
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
        client: Client,
    ) -> MentantResponse<NetworkListResponse> {
        let resp = match client
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
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn network_options(
        &self,
        _caller: Caller,
        data: NetworkRequest,
        client: Client,
    ) -> MentantResponse<NetworkOptionsResponse> {
        let resp = match client
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
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn network_status(
        &self,
        _caller: Caller,
        data: NetworkRequest,
        client: Client,
    ) -> MentantResponse<NetworkStatusResponse> {
        let resp = match client
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
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn account_balance(
        &self,
        _caller: Caller,
        data: AccountBalanceRequest,
        client: Client,
    ) -> MentantResponse<AccountBalanceResponse> {
        let resp = match client
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
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn account_coins(
        &self,
        _caller: Caller,
        data: AccountCoinsRequest,
        client: Client,
    ) -> MentantResponse<AccountCoinsResponse> {
        let resp = match client
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
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn block(
        &self,
        _caller: Caller,
        data: BlockRequest,
        client: Client,
    ) -> MentantResponse<BlockResponse> {
        let resp = match client
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
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
        client: Client,
    ) -> MentantResponse<BlockTransactionResponse> {
        let resp = match client
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
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn mempool(
        &self,
        _caller: Caller,
        data: NetworkRequest,
        client: Client,
    ) -> MentantResponse<MempoolResponse> {
        let resp = match client
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
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        data: MempoolTransactionRequest,
        client: Client,
    ) -> MentantResponse<MempoolTransactionResponse> {
        let resp = match client
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
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }
}
