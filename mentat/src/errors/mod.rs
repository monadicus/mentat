use std::fmt::{Debug, Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::MentantResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    code: u16,
    message: String,
    description: Option<String>,
    retriable: bool,
    #[serde(default)]
    details: IndexMap<String, Value>,
}

impl ApiError {
    pub fn not_implemented<R>() -> MentantResponse<R> {
        Err(MentatError::NotImplemented(ApiError {
            code: 501,
            message: "Not Implemented".to_string(),
            description: None,
            retriable: false,
            details: Default::default(),
        }))
    }

    pub fn wrong_network<P: Debug, R>(payload: P) -> MentantResponse<R> {
        Err(MentatError::Internal(ApiError {
            code: 500,
            message: format!("requestNetwork not supported {payload:?}"),
            description: None,
            retriable: false,
            details: Default::default(),
        }))
    }

    pub fn invalid_account_format<R>() -> MentantResponse<R> {
        Err(MentatError::Internal(ApiError{
            code: 12,
            message: "Invalid account format".to_string(),
            description: Some("This error is returned when the requested AccountIdentifier is improperly formatted.".to_string()),
            retriable: true,
            details: Default::default(),
        }))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MentatError {
    Internal(ApiError),
    NotImplemented(ApiError),
}

impl<T: Display> From<T> for MentatError {
    fn from(e: T) -> Self {
        MentatError::Internal(ApiError {
            code: 500,
            message: "Internal Server Error".to_string(),
            description: Some(e.to_string()),
            retriable: false,
            details: Default::default(),
        })
    }
}

impl IntoResponse for MentatError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            MentatError::Internal(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error)),
            MentatError::NotImplemented(error) => (StatusCode::NOT_IMPLEMENTED, Json(error)),
        };

        (status, error).into_response()
    }
}

pub type Result<T, E = MentatError> = std::result::Result<T, E>;
