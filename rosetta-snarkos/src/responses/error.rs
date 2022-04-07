// {"jsonrpc":"2.0","error":{"code":-32000,"message":"Odd number of
// digits"},"id":"1"}

use mentat::{
    api::MentatResponse,
    errors::{ApiError, MentatError},
    indexmap::IndexMap,
    serde::Deserialize,
    serde_json::Value,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct ErrorResponse {
    pub jsonrpc: String,
    pub error: IndexMap<String, Value>,
    pub id: String,
}

impl<R> From<ErrorResponse> for MentatResponse<R> {
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
