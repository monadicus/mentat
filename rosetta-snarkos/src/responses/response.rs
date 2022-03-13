use mentat::serde::Deserialize;

use super::ErrorResponse;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
#[serde(untagged)]
pub enum Response<R> {
    Ok(R),
    Err(ErrorResponse),
}
