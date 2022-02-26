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
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/network/list"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn network_options(
        &self,
        _caller: Caller,
        data: NetworkRequest,
    ) -> Response<NetworkOptionsResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/network/options"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data: NetworkOptionsResponse = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn network_status(
        &self,
        _caller: Caller,
        data: NetworkRequest,
    ) -> Response<NetworkStatusResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/network/status"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data: NetworkStatusResponse = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn account_balance(
        &self,
        _caller: Caller,
        data: AccountBalanceRequest,
    ) -> Response<AccountBalanceResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/account/balance"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data: AccountBalanceResponse = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn account_coins(
        &self,
        _caller: Caller,
        data: AccountCoinsRequest,
    ) -> Response<AccountCoinsResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/account/coins"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data: AccountCoinsResponse = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn block(&self, _caller: Caller, data: BlockRequest) -> Response<BlockResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/block"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data: BlockResponse = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
    ) -> Response<BlockTransactionResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/block/transaction"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data: BlockTransactionResponse = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn mempool(&self, _caller: Caller, data: NetworkRequest) -> Response<MempoolResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/mempool"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data: MempoolResponse = resp.json().await.unwrap();
        Ok(Json(data))
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        data: MempoolTransactionRequest,
    ) -> Response<MempoolTransactionResponse> {
        let resp = self
            .client
            .post(&format!("{}{}", self.url, "/mempool/transaction"))
            .json(&data)
            .send()
            .await
            .unwrap();
        let data: MempoolTransactionResponse = resp.json().await.unwrap();
        Ok(Json(data))
    }
}
