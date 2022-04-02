use mentat::{
    api::{Caller, CallerConstructionApi, ConstructionApi, MentatResponse},
    async_trait,
    identifiers::{AccountIdentifier, TransactionIdentifier},
    requests::*,
    responses::*,
    serde_json::{self},
    server::RpcCaller,
    IndexMap, Json,
};

use crate::{
    jsonrpc_call,
    request::BitcoinJrpc,
    responses::{common::BitcoinTransaction, Response},
};

#[derive(Clone, Default)]
pub struct BitcoinConstructionApi;

#[async_trait]
impl CallerConstructionApi for BitcoinConstructionApi {}

#[async_trait]
impl ConstructionApi for BitcoinConstructionApi {
    async fn combine(
        &self,
        _caller: Caller,
        data: ConstructionCombineRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<ConstructionCombineResponse> {
        let combination = [data.unsigned_transaction]
            .into_iter()
            .chain(data.signatures.into_iter().map(|sig| sig.hex_bytes))
            .collect();
        let signed_transaction =
            jsonrpc_call!("combinerawtransaction", combination, rpc_caller, String);
        Ok(Json(ConstructionCombineResponse { signed_transaction }))
    }

    async fn derive(
        &self,
        _caller: Caller,
        data: ConstructionDeriveRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<ConstructionDeriveResponse> {
        // NOTE: This will get P2PKH SegWit addresses.
        // Most exchanges implement this as standard nowadays.
        let descriptor = format!("wpkh({})", data.public_key.hex_bytes);
        let address = jsonrpc_call!("deriveaddresses", vec!(descriptor), rpc_caller, String);
        Ok(Json(ConstructionDeriveResponse {
            address: None,
            account_identifier: Some(AccountIdentifier {
                address,
                sub_account: None,
                metadata: IndexMap::new(),
            }),
            metadata: IndexMap::new(),
        }))
    }

    async fn hash(
        &self,
        _caller: Caller,
        data: ConstructionHashRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<TransactionIdentifierResponse> {
        let hash = jsonrpc_call!(
            "decoderawtransaction",
            vec!(data.signed_transaction),
            rpc_caller,
            BitcoinTransaction
        )
        .hash;
        Ok(Json(TransactionIdentifierResponse {
            transaction_identifier: TransactionIdentifier { hash },
            metadata: IndexMap::new(),
        }))
    }

    // async fn metadata(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionMetadataRequest,
    //     rpc_caller: RpcCaller,
    // ) -> MentatResponse<ConstructionMetadataResponse> {
    //     todo!()
    // }

    // async fn parse(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionParseRequest,
    //     rpc_caller: RpcCaller,
    // ) -> MentatResponse<ConstructionParseResponse> {
    //     todo!()
    // }

    // async fn payloads(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionPayloadsRequest,
    //     rpc_caller: RpcCaller,
    // ) -> MentatResponse<ConstructionPayloadsResponse> {
    //     todo!()
    // }

    // async fn preprocess(
    //     &self,
    //     _caller: Caller,
    //     data: ConstructionPreprocessRequest,
    //     rpc_caller: RpcCaller,
    // ) -> MentatResponse<ConstructionPreprocessResponse> {
    //     todo!()
    // }

    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<TransactionIdentifierResponse> {
        let hash = jsonrpc_call!(
            "sendrawtransaction",
            vec!(data.signed_transaction),
            rpc_caller,
            String
        );
        Ok(Json(TransactionIdentifierResponse {
            transaction_identifier: TransactionIdentifier { hash },
            metadata: IndexMap::new(),
        }))
    }
}
