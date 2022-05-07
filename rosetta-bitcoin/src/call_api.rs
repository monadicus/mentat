use mentat::{
    api::{CallApi, Caller, CallerCallApi, MentatResponse},
    axum::{async_trait, Json},
    indexmap::IndexMap,
    requests::*,
    responses::*,
    serde_json::Value,
    server::RpcCaller,
};

use crate::{request::BitcoinJrpc, responses::Response};

#[derive(Clone, Default)]
pub struct BitcoinCallApi;

#[async_trait]
impl CallerCallApi for BitcoinCallApi {}

#[async_trait]
impl CallApi for BitcoinCallApi {
    // TODO associated constant ROUTES that contains a CallRoute struct with all
    // routes and idempotent status      probably shouldnt check if route is
    // valid before calling though, let node handle that

    async fn call(
        &self,
        _caller: Caller,
        data: CallRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<CallResponse> {
        let result = rpc_caller
            .rpc_call::<Response<IndexMap<String, Value>>>(BitcoinJrpc::new(
                &data.method,
                &data
                    .parameters
                    .into_iter()
                    .map(|(_, p)| p)
                    .collect::<Vec<_>>(),
            ))
            .await?;
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
