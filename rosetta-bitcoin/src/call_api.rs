use mentat::{
    api::{CallApi, Caller, CallerCallApi, MentatResponse},
    async_trait,
    errors::*,
    requests::*,
    responses::*,
    serde_json,
    server::RpcCaller,
    Json,
};

#[derive(Default)]
pub struct BitcoinCallApi;

#[async_trait]
impl CallerCallApi for BitcoinCallApi {}

#[async_trait]
impl CallApi for BitcoinCallApi {
    async fn call(
        &self,
        _caller: Caller,
        data: CallRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<CallResponse> {
        let resp = match rpc_caller
            .client
            .post(&rpc_caller.node_rpc_url)
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                return Err(match serde_json::from_str(&e.to_string()) {
                    Ok(s) => MentatError::Internal(s),
                    Err(_) => MentatError::from(format!("unhandled rosetta-bitcoin error: {}", e)),
                });
            }
        };

        let out = resp.text().await?;
        match serde_json::from_str(&out) {
            Ok(o) => Ok(Json(o)),
            Err(_) => Err(MentatError::Internal(serde_json::from_str(&out)?)),
        }
    }
}
