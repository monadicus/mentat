use crate::{
    jsonrpc_call,
    request::{trim_hash, BitcoinJrpc},
    responses::{data::*, Response},
};

use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    async_trait,
    errors::*,
    requests::*,
    responses::*,
    serde_json::{self, json},
    Client, Json,
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
            trim_hash(&block_hash).to_string()
        } else if let Some(block_id) = data.block_identifier.index {
            jsonrpc_call!("getblockhash", vec![block_id], client, String)
        } else {
            jsonrpc_call!("getbestblockhash", vec![] as Vec<u8>, client, String)
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

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
        client: Client,
    ) -> MentatResponse<BlockTransactionResponse> {
        let block_hash = trim_hash(&data.block_identifier.hash);
        let tx_hash = trim_hash(&data.transaction_identifier.hash);
        
        let block = jsonrpc_call!(
            "getblock",
            vec![json!(block_hash), json!(2u32)],
            client,
            GetBlockResponse
        );
        if let Some((i, tx)) = block.tx.iter().enumerate().find_map(|(i, tx)| {
            if tx.hash == tx_hash {
                Some((i, tx))
            } else {
                None
            }
        }) {
            Ok(Json(BlockTransactionResponse {
                transaction: tx.into_transaction(i, &client).await?,
            }))
        } else {
            MentatResponse::from(ApiError::unable_to_find_transaction(
                &data.transaction_identifier.hash,
            ))
        }
    }

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
