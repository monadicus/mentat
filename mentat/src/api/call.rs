use super::*;
use crate::errors::MentatError;

#[axum::async_trait]
pub trait CallApi {
    /// Make a Network-Specific Procedure Call
    async fn call(
        &self,
        _caller: Caller,
        _data: CallRequest,
        _rpc_caller: RpcCaller,
    ) -> MentatResponse<CallResponse> {
        MentatError::not_implemented()
    }
}

#[axum::async_trait]
pub trait CallerCallApi: CallApi + Clone + Default {
    /// Make a Network-Specific Procedure Call
    async fn call_call(
        &self,
        caller: Caller,
        data: CallRequest,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<CallResponse> {
        if mode.is_offline() {
            MentatError::wrong_network(&data)
        } else {
            self.call(caller, data, rpc_caller).await
        }
    }
}
