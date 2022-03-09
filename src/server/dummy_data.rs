use super::*;

pub struct DummyDataApi;

#[async_trait::async_trait]
impl DataApi for DummyDataApi {
    async fn network_list(
        &self,
        _caller: Caller,
        _data: MetadataRequest,
    ) -> Response<NetworkListResponse> {
        Err(ApiError::not_implemented())
    }

    async fn network_options(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
    ) -> Response<NetworkOptionsResponse> {
        Err(ApiError::not_implemented())
    }

    async fn network_status(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
    ) -> Response<NetworkStatusResponse> {
        Err(ApiError::not_implemented())
    }

    async fn account_balance(
        &self,
        _caller: Caller,
        _data: AccountBalanceRequest,
    ) -> Response<AccountBalanceResponse> {
        Err(ApiError::not_implemented())
    }

    async fn account_coins(
        &self,
        _caller: Caller,
        _data: AccountCoinsRequest,
    ) -> Response<AccountCoinsResponse> {
        Err(ApiError::not_implemented())
    }

    async fn block(&self, _caller: Caller, _data: BlockRequest) -> Response<BlockResponse> {
        Err(ApiError::not_implemented())
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        _data: BlockTransactionRequest,
    ) -> Response<BlockTransactionResponse> {
        Err(ApiError::not_implemented())
    }

    async fn call(&self, _caller: Caller, _data: CallRequest) -> Response<CallResponse> {
        Err(ApiError::not_implemented())
    }

    async fn mempool(&self, _caller: Caller, _data: NetworkRequest) -> Response<MempoolResponse> {
        Err(ApiError::not_implemented())
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        _data: MempoolTransactionRequest,
    ) -> Response<MempoolTransactionResponse> {
        Err(ApiError::not_implemented())
    }
}
