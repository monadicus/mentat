//! Defines endpoint errors for the Rosetta API.

use std::fmt::{Debug, Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::ApiError;
use crate::api::MentatResponse;

/// Represents the different types of http Errors.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MentatError {
    /// A Not Found HTTP Error.
    NotFound(ApiError),
    /// An Internal HTTP Error.
    Internal(ApiError),
    /// A NotImplemented HTTP Error.
    NotImplemented(ApiError),
}

impl MentatError {
    /// For when a route is not found.
    pub async fn not_found() -> MentatError {
        MentatError::NotFound(ApiError {
            code: 404,
            message: "Not Found".to_string(),
            description: None,
            retriable: false,
            details: Default::default(),
        })
    }

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
            MentatError::NotFound(error) => (StatusCode::NOT_FOUND, Json(error)),
            MentatError::Internal(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error)),
            MentatError::NotImplemented(error) => (StatusCode::NOT_IMPLEMENTED, Json(error)),
        };

        (status, error).into_response()
    }
}

/// allows you to easily return a MentatError instead of a none/err
pub trait MapErrMentat<F> {
    /// the type to return
    type T;
    /// like `map_err` except returns a mentat error containing the output of the given closure as a string
    fn map_err_mentat(self, err: F) -> Result<Self::T, MentatError>;
}

impl<T, O: Display, F: FnOnce() -> O> MapErrMentat<F> for Option<T> {
    type T = T;

    fn map_err_mentat(self, err: F) -> Result<Self::T, MentatError> {
        match self {
            Some(t) => Ok(t),
            None => Err(MentatError::from(err())),
        }
    }
}

impl<T, E, O: Display, F: FnOnce(E) -> O> MapErrMentat<F> for Result<T, E> {
    type T = T;

    fn map_err_mentat(self, err: F) -> Result<Self::T, MentatError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(MentatError::from(err(e))),
        }
    }
}

/// The Result type for Mentat to always return a `MentatError`.
pub type Result<T, E = MentatError> = std::result::Result<T, E>;
