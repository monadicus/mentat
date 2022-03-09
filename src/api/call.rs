use super::*;

#[async_trait::async_trait]
pub trait CallApi: Send + Sync {
    /// Make a Network-Specific Procedure Call
    async fn call(&self, _caller: Caller, data: CallRequest) -> Response<CallResponse>;
}
