use mentat::serde::Deserialize;

use super::ErrorResponse;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct InnerResponse<R> {
    // jsonrpc: String,
    pub result: R,
    // id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
#[serde(untagged)]
pub enum Response<R> {
    Ok(InnerResponse<R>),
    Err(ErrorResponse),
}
