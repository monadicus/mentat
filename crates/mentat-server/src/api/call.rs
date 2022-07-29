//! Houses the traits for the Rosetta Call API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// Trait to define the endpoints necessary for the Rosetta Call API.
#[axum::async_trait]
pub trait CallApi {
    /// Make a Network-Specific Procedure Call
    async fn call(
        &self,
        _caller: Caller,
        _data: CallRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<CallResponse> {
        MentatError::not_implemented()
    }
}

/// Trait to wrap the `CallApi`.
/// This trait helps to define default behavior for running the endpoints
/// on different modes.
#[axum::async_trait]
pub trait CallerCallApi: CallApi + Clone + Default {
    /// This endpoint only runs in online mode.
    async fn call_call(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<NullableCallRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableCallResponse> {
        if mode.is_offline() {
            MentatError::wrong_network(Some(mode))
        } else {
            asserter.call_request(data.as_ref())?;
            Ok(Json(
                self.call(caller, data.unwrap().into(), rpc_caller)
                    .await?
                    .into(),
            ))
        }
    }
}
