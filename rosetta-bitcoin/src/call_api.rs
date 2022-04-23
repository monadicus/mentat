use mentat::{
    api::{CallApi, CallerCallApi, MentatResponse},
    axum::{async_trait, Json},
    indexmap::IndexMap,
    requests::*,
    responses::*,
    serde_json::{self, Value},
    server::RpcCaller,
    Caller,
};

use crate::{jsonrpc_call, request::BitcoinJrpc, responses::Response};

#[derive(Clone, Default)]
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
        let result = jsonrpc_call!(
            &data.method,
            data.parameters.into_iter().map(|(_, p)| p).collect(),
            rpc_caller,
            IndexMap<String, Value>
        );
        Ok(Json(CallResponse {
            result,
            // TODO: figure out when to set this as true
            //     Idempotent indicates that if /call is invoked with the same CallRequest again, at
            // any point in time, it will return the same CallResponse. Integrators may cache the
            // CallResponse if this is set to true to avoid making unnecessary calls to the Rosetta
            // implementation. For this reason, implementers should be very conservative about
            // returning true here or they could cause issues for the caller.
            idempotent: false,
        }))
    }
}
