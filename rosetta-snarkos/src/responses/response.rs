use std::fmt::Debug;

use mentat::{
    errors::Result,
    serde::{de::DeserializeOwned, Deserialize},
    server::RpcResponse,
    tracing,
};

use super::ErrorResponse;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct InnerResponse<I> {
    // jsonrpc: String,
    pub result: I,
    // id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
#[serde(untagged)]
pub enum Response<O> {
    Ok(InnerResponse<O>),
    Err(ErrorResponse),
}

impl<O> RpcResponse<O> for Response<O>
where
    O: Debug + DeserializeOwned,
{
    fn unwrap_response(self) -> Result<O> {
        match self {
            Response::Ok(res) => {
                tracing::debug!("res: {res:#?}");
                Ok(res.result)
            }
            Response::Err(err) => err.into(),
        }
    }
}
