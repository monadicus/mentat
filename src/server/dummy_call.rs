use super::*;

pub struct DummyCallApi;

#[async_trait::async_trait]
impl CallApi for DummyCallApi {
    async fn call(&self, _caller: Caller, _data: CallRequest) -> Response<CallResponse> {
        Err(ApiError::not_implemented())
    }
}
