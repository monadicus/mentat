//! Houses the traits for the Rosetta Call API.
//! These traits are easily overridable for custom
//! implementations.

use super::*;

/// CallAPIServicer defines the api actions for the CallAPI service
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

/// CallAPIRouter defines the required methods for binding the api requests to a responses for the
/// CallAPI
/// The CallAPIRouter implementation should parse necessary information from the http request,
/// pass the data to a CallAPIServicer to perform the required actions, then write the service
/// results to the http response.
#[axum::async_trait]
pub trait CallApiRouter: CallApi + Clone + Default {
    /// This endpoint only runs in online mode
    async fn call_call(
        &self,
        caller: Caller,
        asserter: &Asserter,
        data: Option<UncheckedCallRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<UncheckedCallResponse> {
        if mode.is_offline() {
            MentatError::unavailable_offline(Some(mode))
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
