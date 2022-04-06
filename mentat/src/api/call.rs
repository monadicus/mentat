//! Houses the traits for the Rosetta Call API.
//! These traits are easily overridable for custom
//! implementations.
use super::*;

///
/// Trait to define the endpoints necessary for the Rosetta Call API.
#[axum::async_trait]
pub trait CallApi {
    /// Make a Network-Specific Procedure Call
    async fn call(
        &self,
        _caller: Caller,
        _data: CallRequest,
        _rpc_caller: RpcCaller,
    ) -> MentatResponse<CallResponse> {
        ApiError::not_implemented()
    }
}

///
/// Trait to wrap the [`CallApi`].
/// This trait helps to define default behavior for running the endpoints
/// on different modes.
#[axum::async_trait]
pub trait CallerCallApi: CallApi + Clone + Default {
    /// This endpoint only runs in online mode.
    async fn call_call(
        &self,
        caller: Caller,
        data: CallRequest,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<CallResponse> {
        if mode.is_offline() {
            ApiError::wrong_network(&data)
        } else {
            self.call(caller, data, rpc_caller).await
        }
    }
}
