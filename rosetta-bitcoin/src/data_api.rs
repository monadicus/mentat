use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    async_trait,
    errors::*,
    identifiers::{BlockIdentifier, TransactionIdentifier},
    misc::{OperationStatus, Version},
    models::Allow,
    requests::*,
    responses::*,
    serde_json::{self, json},
    server::RpcCaller,
    IndexMap, Json,
};

use crate::{
    jsonrpc_call,
    request::{trim_hash, BitcoinJrpc, ScanObjectsDescriptor},
    responses::{
        common::{BitcoinTransaction, GetNetworkInfo, PeerInfo, ScanTxOutSetResult},
        data::*,
        Response,
    },
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

    // TODO: this can be quite general for all mentat implementations
    async fn network_options(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NetworkOptionsResponse> {
        let node_version = jsonrpc_call!(
            "getnetworkinfo",
            vec![] as Vec<u8>,
            rpc_caller,
            GetNetworkInfo
        )
        .version;

        Ok(Json(NetworkOptionsResponse {
            version: Version {
                // TODO: fetch this
                // This is just the current Rosetta version for now
                rosetta_version: "1.4.10".to_owned(),
                node_version: node_version.to_string(),
                middleware_version: Some(env!("CARGO_PKG_VERSION").to_owned()),
                metadata: IndexMap::new(),
            },
            allow: Allow {
                operation_statuses: vec![OperationStatus {
                    status: "SUCCESS".to_owned(),
                    successful: true,
                }],
                operation_types: vec![
                    "COINBASE".to_owned(),
                    "INPUT".to_owned(),
                    "OUTPUT".to_owned(),
                ],
                errors: vec![
                    ApiError::not_implemented(),
                    ApiError::wrong_network("payload"),
                    ApiError::invalid_account_format(),
                    ApiError::unable_to_find_transaction("hash"),
                ],
                historical_balance_lookup: true,
                timestamp_start_index: None,
                // TODO: populate this when `/call` is populated.
                call_methods: None,
                balance_exemptions: None,
                mempool_coins: true,
            },
        }))
    }

    async fn network_status(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NetworkStatusResponse> {
        let current_hash = jsonrpc_call!("getbestblockhash", vec![] as Vec<u8>, rpc_caller, String);
        let current_block = jsonrpc_call!(
            "getblock",
            vec![json!(current_hash), json!(2)],
            rpc_caller,
            GetBlockResponse
        );

        let genesis_hash = jsonrpc_call!("getblockhash", vec![0], rpc_caller, String);
        let genesis_block = jsonrpc_call!(
            "getblock",
            vec![json!(current_hash), json!(2)],
            rpc_caller,
            GetBlockResponse
        );

        Ok(Json(NetworkStatusResponse {
            current_block_identifier: BlockIdentifier {
                index: current_block.height,
                hash: current_hash,
            },
            current_block_timestamp: current_block.time,
            genesis_block_identifier: BlockIdentifier {
                index: genesis_block.height,
                hash: genesis_hash,
            },
            oldest_block_identifier: None,
            sync_status: None,
            peers: jsonrpc_call!("getpeerinfo", vec![] as Vec<u8>, rpc_caller, Vec<PeerInfo>)
                .into_iter()
                .map(|p| p.into())
                .collect(),
        }))
    }

    async fn account_balance(
        &self,
        _caller: Caller,
        data: AccountBalanceRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<AccountBalanceResponse> {
        let args = if let Some(id) = data.block_identifier {
            let range = if let Some(i) = id.index {
                i
            } else if let Some(hash) = id.hash {
                jsonrpc_call!(
                    "getblock",
                    vec![json!(hash), json!(2u32)],
                    rpc_caller,
                    GetBlockResponse
                )
                .height
            } else {
                jsonrpc_call!("getblockcount", vec![] as Vec<u8>, rpc_caller, u64)
            };
            vec![
                json!("start"),
                json!(ScanObjectsDescriptor {
                    desc: data.account_identifier.address,
                    range,
                }),
            ]
        } else {
            vec![json!("start"), json!(data.account_identifier.address)]
        };

        Ok(Json(
            jsonrpc_call!("scantxoutset", args, rpc_caller, ScanTxOutSetResult).into(),
        ))
    }

    // async fn account_coins(
    //     &self,
    //     _caller: Caller,
    //     data: AccountCoinsRequest,
    //     rpc_caller: RpcCaller,
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
            Err(ApiError::unable_to_find_transaction(&data.transaction_identifier.hash).into())
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
            Err(ApiError::unable_to_find_transaction(&data.transaction_identifier.hash).into())
        }
    }
}
