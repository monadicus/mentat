use super::*;

#[async_trait::async_trait]
pub trait CallApi: Send + Sync {
    /// Make a Network-Specific Procedure Call
    async fn call(&self, _caller: Caller, _data: CallRequest) -> Response<CallResponse> {
        ApiError::not_implemented()
    }
}

#[async_trait::async_trait]
pub trait CallerCallApi: CallApi + Send + Sync {
    /// Make a Network-Specific Procedure Call
    async fn call_call(
        &self,
        caller: Caller,
        data: CallRequest,
        mode: &ModeState,
    ) -> Response<CallResponse> {
        if mode.is_offline() {
            ApiError::wrong_network(&data)
        } else {
            self.call(caller, data).await
        }
    }
}
