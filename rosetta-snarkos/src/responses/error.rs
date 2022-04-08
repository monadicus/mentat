// {"jsonrpc":"2.0","error":{"code":-32000,"message":"Odd number of
// digits"},"id":"1"}

use mentat::{
    errors::{ApiError, MentatError, Result},
    serde::Deserialize,
    serde_json::Value,
    IndexMap,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct ErrorResponse {
    pub jsonrpc: String,
    pub error: IndexMap<String, Value>,
    pub id: String,
}

impl<R> From<ErrorResponse> for Result<R> {
    fn from(response: ErrorResponse) -> Self {
        Err(MentatError::Internal(ApiError {
            code: 500,
            message: "Snarkos JsonRPC Error.".to_string(),
            description: None,
            retriable: true,
            details: response.error,
        }))
    }
}
