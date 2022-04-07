use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    axum::{async_trait, Json},
    errors::*,
    identifiers::TransactionIdentifier,
    indexmap::IndexMap,
    requests::*,
    responses::*,
    serde_json::{self, json},
    server::RpcCaller,
};

use crate::{
    jsonrpc_call,
    request::{trim_hash, BitcoinJrpc},
    responses::{common::BitcoinTransaction, data::*, Response},
};

#[derive(Clone, Default)]
pub struct BitcoinDataApi;

#[async_trait]
impl CallerDataApi for BitcoinDataApi {}

#[async_trait]
impl DataApi for BitcoinDataApi {
    async fn network_list(
        &self,
        _caller: Caller,
        _data: MetadataRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NetworkListResponse> {
        Ok(Json(
            jsonrpc_call!(
                "getblockchaininfo",
                vec![] as Vec<u8>,
                rpc_caller,
                GetBlockchainInfoResponse
            )
            .into(),
        ))
    }

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
        rpc_caller: RpcCaller,
    ) -> MentatResponse<BlockResponse> {
        let hash = if let Some(block_hash) = data.block_identifier.hash {
            trim_hash(&block_hash).to_string()
        } else if let Some(block_id) = data.block_identifier.index {
            jsonrpc_call!("getblockhash", vec![block_id], rpc_caller, String)
        } else {
            jsonrpc_call!("getbestblockhash", vec![] as Vec<u8>, rpc_caller, String)
        };

        jsonrpc_call!(
            "getblock",
            vec![json!(hash), json!(2)],
            rpc_caller,
            GetBlockResponse
        )
        .into_block_response(&rpc_caller)
        .await
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<BlockTransactionResponse> {
        let block_hash = trim_hash(&data.block_identifier.hash);
        let tx_hash = trim_hash(&data.transaction_identifier.hash);

        let block = jsonrpc_call!(
            "getblock",
            vec![json!(block_hash), json!(2u32)],
            rpc_caller,
            GetBlockResponse
        );

        if let Some((i, tx)) = block
            .tx
            .into_iter()
            .enumerate()
            .find(|(_, tx)| tx.hash == tx_hash)
        {
            Ok(Json(BlockTransactionResponse {
                transaction: tx.into_transaction(i, &rpc_caller).await?,
            }))
        } else {
            ApiError::unable_to_find_transaction(&data.transaction_identifier.hash)
        }
    }

    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<MempoolResponse> {
        let transaction_identifiers =
            jsonrpc_call!("getrawmempool", vec![] as Vec<u8>, rpc_caller, Vec<String>)
                .into_iter()
                .map(|hash| TransactionIdentifier { hash })
                .collect();
        Ok(Json(MempoolResponse {
            transaction_identifiers,
        }))
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        data: MempoolTransactionRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<MempoolTransactionResponse> {
        let tx_hash = trim_hash(&data.transaction_identifier.hash);
        let mempool = jsonrpc_call!("getrawmempool", vec![] as Vec<u8>, rpc_caller, Vec<String>);

        if let Some((i, _)) = mempool
            .into_iter()
            .enumerate()
            .find(|(_, id)| id.as_str() == tx_hash)
        {
            let transaction = jsonrpc_call!(
                "getrawtransaction",
                vec![json!(tx_hash), json!(true)],
                rpc_caller,
                BitcoinTransaction
            )
            .into_transaction(i, &rpc_caller)
            .await?;
            Ok(Json(MempoolTransactionResponse {
                transaction,
                metadata: IndexMap::new(),
            }))
        } else {
            ApiError::unable_to_find_transaction(&data.transaction_identifier.hash)
        }
    }
}
