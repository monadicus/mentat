use mentat::serde::Deserialize;

use super::ErrorResponse;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct Response<I> {
    pub result: Option<I>,
    pub error: Option<ErrorResponse>,
}
