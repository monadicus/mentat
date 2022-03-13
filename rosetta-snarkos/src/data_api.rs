use crate::{
    jsonrpc_call,
    responses::{data::*, Response},
};

use super::SnarkosJrpc;

use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    async_trait,
    errors::MentatError,
    requests::*,
    responses::*,
    Client,
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
    ) -> MentatResponse<BlockResponse> {
        if let Some(block_id) = data.block_identifier.index {
            jsonrpc_call!("getblock", vec![block_id], client, GetBlockResponse)
        } else {
            Err(MentatError::from("wtf"))
        }
    }
}
