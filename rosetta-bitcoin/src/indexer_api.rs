use mentat::{
    api::{Caller, CallerIndexerApi, IndexerApi, MentatResponse},
    axum::{async_trait, Json},
    errors::*,
    requests::*,
    responses::*,
    serde_json,
    server::RpcCaller,
};

#[derive(Clone, Default)]
pub struct BitcoinIndexerApi;

#[async_trait]
impl CallerIndexerApi for BitcoinIndexerApi {}

#[async_trait]
impl IndexerApi for BitcoinIndexerApi {
    async fn events_blocks(
        &self,
        _caller: Caller,
        data: EventsBlocksRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<EventsBlocksResponse> {
        let resp = rpc_caller
            .client
            .post(rpc_caller.node_rpc_url)
            .json(&data)
            .send()
            .await
            .merr(|e| {
                serde_json::from_str(&e.to_string())
                    .unwrap_or_else(|e| format!("unhandled rosetta-bitcoin error: {}", e))
            })?;

        let out = resp.text().await?;
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }

    async fn search_transactions(
        &self,
        _caller: Caller,
        data: SearchTransactionsRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<SearchTransactionsResponse> {
        let resp = rpc_caller
            .client
            .post(rpc_caller.node_rpc_url)
            .json(&data)
            .send()
            .await
            .merr(|e| {
                serde_json::from_str(&e.to_string())
                    .unwrap_or_else(|e| format!("unhandled rosetta-bitcoin error: {}", e))
            })?;

        let out = resp.text().await?;
        Ok(Json(serde_json::from_str(&out)?))
    }
}
