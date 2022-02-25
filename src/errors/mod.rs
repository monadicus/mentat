use indexmap::IndexMap;
use rocket::{
    response::{self, Responder},
    serde::{
        json::{Json, Value},
        Deserialize, Serialize,
    },
    Request, Responder as DeriveResponder,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    code: u16,
    message: String,
    description: Option<String>,
    retryable: bool,
    details: IndexMap<String, Value>,
}

impl ApiError {
    pub fn internal_server<T>(error: anyhow::Error) -> Result<T> {
        Err(MentatError::Internal(ApiError {
            code: 500,
            message: "Internal Server Error".to_string(),
            description: Some(error.to_string()),
            retryable: false,
            details: Default::default(),
        }))
    }

    pub fn not_implemented<T>() -> Result<T> {
        Err(MentatError::NotImplemented(ApiError {
            code: 501,
            message: "Not Implemented".to_string(),
            description: None,
            retryable: false,
            details: Default::default(),
        }))
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, r: &'r Request<'_>) -> response::Result<'static> {
        Json(self).respond_to(r)
    }
}

#[derive(Debug, Deserialize, DeriveResponder, Serialize)]
pub enum MentatError {
    #[response(status = 500, content_type = "json")]
    Internal(ApiError),
    #[response(status = 501, content_type = "json")]
    NotImplemented(ApiError),
}

pub type Result<T, E = MentatError> = std::result::Result<T, E>;
