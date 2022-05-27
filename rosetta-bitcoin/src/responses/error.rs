// {"jsonrpc":"2.0","error":{"code":-32000,"message":"Odd number of
// digits"},"id":"1"}

use mentat::{
    errors::{ApiError, MentatError, Result},
    serde::Deserialize,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
}

impl<R> From<ErrorResponse> for Result<R> {
    fn from(response: ErrorResponse) -> Self {
        Err(MentatError::Internal(ApiError {
            code: 500,
            message: "Bitcoin JsonRPC Error.".to_string(),
            description: None,
            retriable: true,
            details: [
                ("code".to_string(), response.code.into()),
                ("message".to_string(), response.message.into()),
            ]
            .into(),
        }))
    }
}
