use mentat::serde::Deserialize;

use super::ErrorResponse;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct Response<R> {
    pub result: Option<R>,
    pub error: Option<ErrorResponse>,
}
