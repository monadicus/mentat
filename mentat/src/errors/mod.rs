use std::fmt::{Debug, Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub description: Option<String>,
    pub retriable: bool,
    #[serde(default)]
    pub details: IndexMap<String, Value>,
}

impl ApiError {
    pub fn not_implemented() -> Self {
        ApiError {
            code: 501,
            message: "Not Implemented".to_string(),
            description: None,
            retriable: false,
            details: Default::default(),
        }
    }

    pub fn wrong_network<P: Debug>(payload: P) -> Self {
        ApiError {
            code: 500,
            message: format!("requestNetwork not supported {payload:?}"),
            description: None,
            retriable: false,
            details: Default::default(),
        }
    }

    pub fn invalid_account_format() -> Self {
        ApiError{
            code: 12,
            message: "Invalid account format".to_string(),
            description: Some("This error is returned when the requested AccountIdentifier is improperly formatted.".to_string()),
            retriable: true,
            details: Default::default(),
        }
    }

    pub fn unable_to_find_transaction(hash: &str) -> Self {
        ApiError {
            code: 16,
            message: "Invalid account format".to_string(),
            description: Some(String::from("Transaction not found")),
            retriable: true,
            details: {
                let mut map = IndexMap::new();
                map.insert(
                    String::from("context"),
                    format!("unable to find transaction {hash}").into(),
                );
                map
            },
        }
    }

    pub fn unable_to_get_balance(context: &str) -> Self {
        ApiError {
            code: 18,
            message: String::from("Unable to get balance"),
            description: None,
            retriable: false,
            details: {
                let mut map = IndexMap::new();
                map.insert(String::from("context"), context.into());
                map
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl From<ApiError> for MentatError {
    fn from(e: ApiError) -> Self {
        MentatError::Internal(e)
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
