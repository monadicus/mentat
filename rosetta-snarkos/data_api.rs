use mentat::{
    api::{Caller, CallerDataApi, DataApi, Response},
    requests::*,
    responses::*,
};
use rocket::serde::json::{serde_json, Json};

#[derive(Default)]
pub struct SnarkosDataApi;

#[rocket::async_trait]
impl CallerDataApi for SnarkosDataApi {}

#[rocket::async_trait]
impl DataApi for SnarkosDataApi {
    async fn block(&self, _caller: Caller, _data: BlockRequest) -> Response<BlockResponse> {
        let data = serde_json::json!(
        {
        "jsonrpc": "2.0",
        "id": "1",
        "method": "getblock",
        "params": [0]
        });

        let client = reqwest::Client::new();
        let response = client
            .post("http://127.0.0.1:3032")
            .json(&data)
            .send()
            .await?;
        rocket::error!("{:?}", response.text().await);

        Ok(Json(BlockResponse {
            block: None,
            other_transactions: None,
        }))
    }
}
