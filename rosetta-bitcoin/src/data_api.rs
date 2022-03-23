use crate::{
    jsonrpc_call,
    request::BitcoinJrpc,
    responses::{data::*, Response},
};

use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    async_trait,
    errors::*,
    requests::*,
    responses::*,
    serde_json::{self, json},
    Client,
};

#[derive(Default)]
pub struct BitcoinDataApi;

#[async_trait]
impl CallerDataApi for BitcoinDataApi {}

#[async_trait]
impl DataApi for BitcoinDataApi {
    // async fn network_list(
    //     &self,
    //     _caller: Caller,
    //     data: MetadataRequest,
    //     client: Client,
    // ) -> MentatResponse<NetworkListResponse> {
    //     todo!()
    // }

    // async fn network_options(
    //     &self,
    //     _caller: Caller,
    //     data: NetworkRequest,
    //     client: Client,
    // ) -> MentatResponse<NetworkOptionsResponse> {
    //     todo!()
    // }

    // async fn network_status(
    //     &self,
    //     _caller: Caller,
    //     data: NetworkRequest,
    //     client: Client,
    // ) -> MentatResponse<NetworkStatusResponse> {
    //     todo!()
    // }

    // async fn account_balance(
    //     &self,
    //     _caller: Caller,
    //     data: AccountBalanceRequest,
    //     client: Client,
    // ) -> MentatResponse<AccountBalanceResponse> {
    //     todo!()
    // }

    // async fn account_coins(
    //     &self,
    //     _caller: Caller,
    //     data: AccountCoinsRequest,
    //     client: Client,
    // ) -> MentatResponse<AccountCoinsResponse> {
    //     todo!()
    // }

    async fn block(
        &self,
        _caller: Caller,
        data: BlockRequest,
        client: Client,
    ) -> MentatResponse<BlockResponse> {
        let hash = if let Some(block_hash) = data.block_identifier.hash {
            block_hash
        } else if let Some(block_id) = data.block_identifier.index {
            jsonrpc_call!("getblockhash", vec![block_id], client, String)
        } else {
            return Err(MentatError::from("wtf"));
        };

        jsonrpc_call!(
            "getblock",
            vec![json!(hash), json!(2)],
            client,
            GetBlockResponse
        )
        .into_block_response(&client)
        .await
    }

    // async fn block_transaction(
    //     &self,
    //     _caller: Caller,
    //     data: BlockTransactionRequest,
    //     client: Client,
    // ) -> MentatResponse<BlockTransactionResponse> {
    //     todo!()
    // }

    // async fn mempool(
    //     &self,
    //     _caller: Caller,
    //     data: NetworkRequest,
    //     client: Client,
    // ) -> MentatResponse<MempoolResponse> {
    //     todo!()
    // }

    // async fn mempool_transaction(
    //     &self,
    //     _caller: Caller,
    //     data: MempoolTransactionRequest,
    //     client: Client,
    // ) -> MentatResponse<MempoolTransactionResponse> {
    //     todo!()
    // }
}
