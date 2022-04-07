//! Defines endpoint errors for the Rosetta API.

use std::fmt::{Debug, Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::MentatResponse;

/// The Error type for any mentat responses.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiError {
    /// The http status code.
    pub code: u16,
    /// The message for the error.
    pub message: String,
    /// The optional description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If the method is retriable.
    pub retriable: bool,
    /// Any additional details for the error.
    #[serde(default)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub details: IndexMap<String, Value>,
}

impl ApiError {
    /// For when a method is not implemented.
    pub fn not_implemented<R>() -> MentatResponse<R> {
        Err(MentatError::NotImplemented(ApiError {
            code: 501,
            message: "Not Implemented".to_string(),
            description: None,
            retriable: false,
            details: Default::default(),
        }))
    }

    /// For when a method called but not available on the current network.
    pub fn wrong_network<P: Debug, R>(payload: P) -> MentatResponse<R> {
        Err(MentatError::Internal(ApiError {
            code: 500,
            message: format!("requestNetwork not supported {payload:?}"),
            description: None,
            retriable: false,
            details: Default::default(),
        }))
    }

    /// This error is returned when the requested
    /// [`crate::identifiers::AccountIdentifier`] is improperly formatted.
    pub fn invalid_account_format<R>() -> MentatResponse<R> {
        Err(MentatError::Internal(ApiError{
            code: 12,
            message: "Invalid account format".to_string(),
            description: Some("This error is returned when the requested AccountIdentifier is improperly formatted.".to_string()),
            retriable: true,
            details: Default::default(),
        }))
    }

    /// When a transaction could not be found.
    pub fn unable_to_find_transaction<R>(hash: &str) -> MentatResponse<R> {
        Err(MentatError::Internal(ApiError {
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
        }))
    }
}

/// Represents the different types of http Errors.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MentatError {
    /// An Internal HTTP Error.
    Internal(ApiError),
    /// A NotImplemented HTTP Error.
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

/// The Result type for Mentat to always return a `MentatError`.
pub type Result<T, E = MentatError> = std::result::Result<T, E>;
