use mentat::{
    errors::{MentatError, Result},
    serde::{de::DeserializeOwned, Deserialize},
    server::RpcResponse,
    tracing,
};

use crate::request::BitcoinJrpc;

use super::ErrorResponse;

use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct Response<R> {
    pub result: Option<R>,
    pub error: Option<ErrorResponse>,
}

impl<O> RpcResponse for Response<O>
where
    O: Debug + DeserializeOwned,
{
    type I = BitcoinJrpc;
    type O = O;

    fn unwrap_response(self) -> Result<Self::O> {
        tracing::debug!("res: {self:#?}");
        match self {
            Response {
                result: Some(res),
                error: None,
            } => Ok(res),
            Response {
                result: None,
                error: Some(err),
            } => err.into(),
            _ => Err(MentatError::from(format!("unknown response: {self:?}"))),
        }
    }
}
