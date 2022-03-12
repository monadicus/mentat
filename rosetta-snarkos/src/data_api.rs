use crate::responses::SnarkOSBlockResponse;

use super::SnarkOsJrpc;

use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentantResponse},
    async_trait,
    errors::MentatError,
    requests::*,
    responses::*,
    Client, Json,
};

#[derive(Default)]
pub struct SnarkosDataApi;

#[async_trait]
impl CallerDataApi for SnarkosDataApi {}

#[async_trait]
impl DataApi for SnarkosDataApi {
    async fn block(
        &self,
        _caller: Caller,
        data: BlockRequest,
        client: Client,
    ) -> MentantResponse<BlockResponse> {
        if let Some(block_id) = data.block_identifier.index {
            let req = SnarkOsJrpc::new("getblock", vec![block_id]);

            let response = client
                .post("http://127.0.0.1:3032")
                .json(&req)
                .send()
                .await?;

            let result: SnarkOSBlockResponse = response.json().await?;
            Ok(Json(result.into()))
        } else {
            Err(MentatError::from("wtf"))
        }
    }
}
