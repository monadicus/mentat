use super::*;

pub struct DummyDataApi;

#[async_trait::async_trait]
impl DataApi for DummyDataApi {
    async fn network_list(
        &self,
        _caller: Caller,
        _data: MetadataRequest,
    ) -> Response<NetworkListResponse> {
        ApiError::not_implemented()
    }

    async fn network_options(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
    ) -> Response<NetworkOptionsResponse> {
        ApiError::not_implemented()
    }

    async fn network_status(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
    ) -> Response<NetworkStatusResponse> {
        ApiError::not_implemented()
    }

    async fn account_balance(
        &self,
        _caller: Caller,
        _data: AccountBalanceRequest,
    ) -> Response<AccountBalanceResponse> {
        ApiError::not_implemented()
    }

    async fn account_coins(
        &self,
        _caller: Caller,
        _data: AccountCoinsRequest,
    ) -> Response<AccountCoinsResponse> {
        ApiError::not_implemented()
    }

    async fn block(&self, _caller: Caller, _data: BlockRequest) -> Response<BlockResponse> {
        ApiError::not_implemented()
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        _data: BlockTransactionRequest,
    ) -> Response<BlockTransactionResponse> {
        ApiError::not_implemented()
    }

    async fn mempool(&self, _caller: Caller, _data: NetworkRequest) -> Response<MempoolResponse> {
        ApiError::not_implemented()
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        _data: MempoolTransactionRequest,
    ) -> Response<MempoolTransactionResponse> {
        ApiError::not_implemented()
    }
}
