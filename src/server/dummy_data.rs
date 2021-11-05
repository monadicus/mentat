use super::*;

pub struct DummyDataApi;

#[async_trait::async_trait]
impl DataApi for DummyDataApi {
    async fn network_list(&self, _caller: Caller, _data: MetadataRequest) -> Result<NetworkListResponse> {
        not_implemented()
    }

    async fn network_options(&self, _caller: Caller, _data: NetworkRequest) -> Result<NetworkOptionsResponse> {
        not_implemented()
    }

    async fn network_status(&self, _caller: Caller, _data: NetworkRequest) -> Result<NetworkStatusResponse> {
        not_implemented()
    }

    async fn account_balance(&self, _caller: Caller, _data: AccountBalanceRequest) -> Result<AccountBalanceResponse> {
        not_implemented()
    }

    async fn account_coins(&self, _caller: Caller, _data: AccountCoinsRequest) -> Result<AccountCoinsResponse> {
        not_implemented()
    }

    async fn block(&self, _caller: Caller, _data: BlockRequest) -> Result<BlockResponse> {
        not_implemented()
    }

    async fn block_transaction(&self, _caller: Caller, _data: BlockTransactionRequest) -> Result<BlockTransactionResponse> {
        not_implemented()
    }

    async fn mempool(&self, _caller: Caller, _data: NetworkRequest) -> Result<MempoolResponse> {
        not_implemented()
    }

    async fn mempool_transaction(&self, _caller: Caller, _data: MempoolTransactionRequest) -> Result<MempoolTransactionResponse> {
        not_implemented()
    }
}

