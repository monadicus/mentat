use super::*;

#[axum::async_trait]
pub trait CallApi: Send + Sync {
    /// Make a Network-Specific Procedure Call
    async fn call(
        &self,
        _caller: Caller,
        _data: CallRequest,
        _client: Client,
    ) -> MentatResponse<CallResponse> {
        ApiError::not_implemented()
    }
}

#[axum::async_trait]
pub trait CallerCallApi: CallApi + Send + Sync {
    /// Make a Network-Specific Procedure Call
    async fn call_call(
        &self,
        caller: Caller,
        data: CallRequest,
        mode: &Mode,
        client: Client,
    ) -> MentatResponse<CallResponse> {
        if mode.is_offline() {
            ApiError::wrong_network(&data)
        } else {
            self.call(caller, data, client).await
        }
    }
}
