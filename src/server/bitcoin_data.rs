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
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/network/list"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }

    async fn network_options(
        &self,
        _caller: Caller,
        data: NetworkRequest,
    ) -> Response<NetworkOptionsResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/network/options"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }

    async fn network_status(
        &self,
        _caller: Caller,
        data: NetworkRequest,
    ) -> Response<NetworkStatusResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/network/status"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }

    async fn account_balance(
        &self,
        _caller: Caller,
        data: AccountBalanceRequest,
    ) -> Response<AccountBalanceResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/account/balance"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }

    async fn account_coins(
        &self,
        _caller: Caller,
        data: AccountCoinsRequest,
    ) -> Response<AccountCoinsResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/account/coins"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }

    async fn block(&self, _caller: Caller, data: BlockRequest) -> Response<BlockResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/block"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
    ) -> Response<BlockTransactionResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/block/transaction"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }

    async fn mempool(&self, _caller: Caller, data: NetworkRequest) -> Response<MempoolResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/mempool"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        data: MempoolTransactionRequest,
    ) -> Response<MempoolTransactionResponse> {
        let resp = match self
            .client
            .post(&format!("{}{}", self.url, "/mempool/transaction"))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let err: ApiError = serde_json::from_str(&e.to_string()).unwrap();
                return Err(MentatError::Internal(err));
            }
        };

        match resp.json().await {
            Ok(d) => Ok(Json(d)),
            Err(e) => ApiError::internal_server(anyhow!(e)),
        }
    }
}
