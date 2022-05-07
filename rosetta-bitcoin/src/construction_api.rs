use std::str::FromStr;

use bitcoin::{
    hash_types::PubkeyHash,
    psbt::serialize::{Deserialize, Serialize},
    OutPoint,
    Script,
    Transaction,
    TxIn,
    TxOut,
    Txid,
    Witness,
};
use mentat::{
    api::{Caller, CallerConstructionApi, ConstructionApi, MentatResponse},
    axum::{async_trait, Json},
    errors::MentatError,
    identifiers::{AccountIdentifier, TransactionIdentifier},
    indexmap::IndexMap,
    models::{Amount, Coin, Currency, SignatureType, SigningPayload},
    requests::*,
    responses::*,
    serde_json::{self},
    server::RpcCaller,
};
use serde_json::json;

use crate::{
    request::BitcoinJrpc,
    responses::{
        common::{BitcoinTransaction, FeeEstimate},
        Response,
    },
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
        _rpc_caller: RpcCaller,
    ) -> MentatResponse<ConstructionCombineResponse> {
        let mut tx = Transaction::deserialize(
            &hex::decode(data.unsigned_transaction)
                .map_err(|_| MentatError::from("transaction malformed"))?,
        )?;
        for (vin, sig) in tx.input.iter_mut().zip(data.signatures) {
            vin.script_sig = Script::from(
                hex::decode(sig.hex_bytes).map_err(|_| MentatError::from("signature malformed"))?,
            );
        }

        Ok(Json(ConstructionCombineResponse {
            signed_transaction: hex::encode(tx.serialize()),
        }))
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
        let address = rpc_caller
            .rpc_call::<Response<String>>(BitcoinJrpc::new("deriveaddresses", &[descriptor]))
            .await?;
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
        let hash = rpc_caller
            .rpc_call::<Response<BitcoinTransaction>>(BitcoinJrpc::new(
                "decoderawtransaction",
                &[data.signed_transaction],
            ))
            .await?
            .hash;
        Ok(Json(TransactionIdentifierResponse {
            transaction_identifier: TransactionIdentifier { hash },
            metadata: IndexMap::new(),
        }))
    }

    async fn metadata(
        &self,
        _caller: Caller,
        _data: ConstructionMetadataRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<ConstructionMetadataResponse> {
        let suggested_fee = rpc_caller
            .rpc_call::<Response<FeeEstimate>>(BitcoinJrpc::new(
                "estimatesmartfee",
                // NOTE: this might produce slower to confirm transactions, but they will be
                // cheaper.
                // May want to look into making this configurable.
                &[6],
            ))
            .await?
            .feerate;

        Ok(Json(ConstructionMetadataResponse {
            metadata: Default::default(),
            suggested_fee: Some(vec![Amount {
                value: suggested_fee.to_string(),
                currency: Currency {
                    symbol: "BTC".to_string(),
                    decimals: 8,
                    metadata: Default::default(),
                },
                metadata: Default::default(),
            }]),
        }))
    }

    async fn parse(
        &self,
        _caller: Caller,
        data: ConstructionParseRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<ConstructionParseResponse> {
        let tx = BitcoinTransaction::from(Transaction::deserialize(
            &hex::decode(data.transaction)
                .map_err(|_| MentatError::from("transaction malformed"))?,
        )?);

        Ok(Json(ConstructionParseResponse {
            operations: tx
                .clone()
                .into_transaction(0, &rpc_caller)
                .await?
                .operations,
            signers: None,
            account_identifier_signers: if data.signed {
                let vin_len = tx.vin.len();
                let hash = tx.hash.clone();
                Some(
                    tx.vout
                        .into_iter()
                        .enumerate()
                        .filter_map(|(i, vout)| {
                            vout.into_operation((i + vin_len) as u64, &hash).account
                        })
                        .collect(),
                )
            } else {
                None
            },
            metadata: Default::default(),
        }))
    }

    // todo 0rphon: can clean this up once generalized jsonrpc_call is merged into
    // this branch
    async fn payloads(
        &self,
        _caller: Caller,
        data: ConstructionPayloadsRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<ConstructionPayloadsResponse> {
        let mut tx = Transaction {
            version: 2,
            lock_time: 0,
            input: vec![],
            output: vec![],
        };

        let coins = data
            .metadata
            .get("coins")
            .ok_or_else(|| MentatError::from("no coins provided"))?
            .as_array()
            .ok_or_else(|| MentatError::from("malformed coins field in metadata"))?;
        for coin in coins {
            let (txid, vout) = coin
                .get("coin_identifier")
                .ok_or_else(|| MentatError::from("no coin identifier on coin struct"))?
                .as_str()
                .ok_or_else(|| MentatError::from("coin identifier is wrong type"))?
                .split_once(':')
                .ok_or_else(|| MentatError::from("invalid coin identifier format"))?;
            tx.input.push(TxIn {
                previous_output: OutPoint {
                    txid: Txid::from_str(txid).map_err(|_| MentatError::from("invalid txid"))?,
                    vout: vout
                        .parse::<u32>()
                        .map_err(|_| MentatError::from("invalid vout field"))?,
                },
                // This gets filled in later in `combine`.
                script_sig: Script::new(),
                sequence: u32::MAX,
                witness: Witness::new(),
            });
        }

        let mut payloads = vec![];
        for (i, input) in tx.input.iter().enumerate() {
            let script_pub_key = rpc_caller
                .rpc_call::<Response<BitcoinTransaction>>(BitcoinJrpc::new(
                    "getrawtransaction",
                    &[json!(input.previous_output.txid.to_string()), json!(true)],
                ))
                .await?
                .vout
                .into_iter()
                .nth(input.previous_output.vout as usize)
                .unwrap()
                .scriptPubKey;

            payloads.push(SigningPayload {
                address: None,
                account_identifier: None,
                hex_bytes: tx
                    .signature_hash(i, &script_pub_key.try_into()?, 0)
                    .to_string(),
                signature_type: Some(SignatureType::Ecdsa),
            });
        }

        for op in data.operations {
            if op.type_ == "OUTPUT" {
                tx.output.push(TxOut {
                    value: op
                        .amount
                        .ok_or_else(|| MentatError::from("no amount for payment operation"))?
                        .value
                        .parse::<isize>()
                        .map_err(|_| MentatError::from("invalid value"))?
                        as u64,
                    script_pubkey: Script::new_p2pkh(
                        &PubkeyHash::from_str(
                            &op.account
                                .ok_or_else(|| {
                                    MentatError::from("no account for payment operation")
                                })?
                                .address,
                        )
                        .map_err(|_| MentatError::from("invalid address"))?,
                    ),
                })
            }
        }

        Ok(Json(ConstructionPayloadsResponse {
            unsigned_transaction: hex::encode(tx.serialize()),
            payloads,
        }))
    }

    async fn preprocess(
        &self,
        _caller: Caller,
        data: ConstructionPreprocessRequest,
        _rpc_caller: RpcCaller,
    ) -> MentatResponse<ConstructionPreprocessResponse> {
        let mut options = IndexMap::new();

        let coins: Vec<Coin> = data
            .operations
            .iter()
            .filter_map(|operation| {
                if let (Some(coin_change), Some(amount)) =
                    (&operation.coin_change, &operation.amount)
                {
                    Some(Coin {
                        coin_identifier: coin_change.coin_identifier.clone(),
                        amount: amount.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();

        options.insert("coins".to_string(), json!(coins));
        Ok(Json(ConstructionPreprocessResponse {
            options,
            required_public_keys: Some(
                data.operations
                    .into_iter()
                    .filter_map(|operation| {
                        if operation.account.is_some() {
                            operation.account
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
        }))
    }

    async fn submit(
        &self,
        _caller: Caller,
        data: ConstructionSubmitRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<TransactionIdentifierResponse> {
        let hash = rpc_caller
            .rpc_call::<Response<String>>(BitcoinJrpc::new(
                "sendrawtransaction",
                &[data.signed_transaction],
            ))
            .await?;
        Ok(Json(TransactionIdentifierResponse {
            transaction_identifier: TransactionIdentifier { hash },
            metadata: IndexMap::new(),
        }))
    }
}
