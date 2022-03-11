use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentantResponse},
    async_trait,
    requests::*,
    responses::*,
    serde_json, tracing, Client, Json,
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
        _data: BlockRequest,
        client: Client,
    ) -> MentantResponse<BlockResponse> {
        let data = serde_json::json!(
        {
        "jsonrpc": "2.0",
        "id": "1",
        "method": "getblock",
        "params": [0]
        });

        let response = client
            .post("http://127.0.0.1:3032")
            .json(&data)
            .send()
            .await?;
        let text = response.text().await?;
        tracing::debug!("output /block {text}");

        Ok(Json(BlockResponse {
            block: None,
            other_transactions: None,
        }))
    }
}
