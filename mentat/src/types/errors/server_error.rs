//! Defines endpoint errors for the Rosetta API.

use std::fmt::{self, Debug, Display};

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
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct MentatError {
    /// The http status code.
    #[serde(skip)]
    pub status_code: u16,
    /// The rosetta error code
    pub code: i32,
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

impl MentatError {
    /// create an optional details field on an error
    fn context<T: Display>(c: Option<T>, message: fn(T) -> String) -> IndexMap<String, Value> {
        c.map(|c| [("context".to_string(), message(c).into())].into())
            .unwrap_or_default()
    }

    // has to be done separate from the display trait due to a conflict with rust
    // default impls
    /// display formatting
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(desc) = &self.description {
            write!(f, "[{}] {}: {}", self.code, self.message, desc)
        } else {
            write!(f, "[{}] {}", self.code, self.message)
        }
    }

    /// returns a list of all built in mentat errors
    pub fn all_errors() -> Vec<Self> {
        vec![
            MentatError::not_implemented::<()>().unwrap_err(),
            MentatError::unavailable_offline::<&str, ()>(None).unwrap_err(),
            MentatError::node_not_ready::<&str, ()>(None).unwrap_err(),
            MentatError::node_error::<&str, ()>(None).unwrap_err(),
            MentatError::block_not_found::<&str, ()>(None).unwrap_err(),
            MentatError::unable_to_derive_address::<&str, ()>(None).unwrap_err(),
            MentatError::bad_intent::<&str, ()>(None).unwrap_err(),
            MentatError::bad_intermediate::<&str, ()>(None).unwrap_err(),
            MentatError::missing_keys::<&str, ()>(None).unwrap_err(),
            MentatError::invalid_coin::<&str, ()>(None).unwrap_err(),
            MentatError::bad_address::<&str, ()>(None).unwrap_err(),
            MentatError::bad_pub_key::<&str, ()>(None).unwrap_err(),
            MentatError::calc_sig_hash::<&str, ()>(None).unwrap_err(),
            MentatError::unsupported_script_type::<&str, ()>(None).unwrap_err(),
            MentatError::compute_pk_script::<&str, ()>(None).unwrap_err(),
            MentatError::failed_to_get_coins::<&str, ()>(None).unwrap_err(),
            MentatError::transaction_not_found::<&str, ()>(None).unwrap_err(),
            MentatError::couldnt_get_fee_rate::<&str, ()>(None).unwrap_err(),
            MentatError::couldnt_get_balance::<&str, ()>(None).unwrap_err(),
            MentatError::wrong_network::<&str, ()>(None).unwrap_err(),
            MentatError::not_found_example::<&str, ()>(None).unwrap_err(),
        ]
    }

    /// For when a method called but not available on the current network.
    pub fn wrong_network<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 500,
            message: "requestNetwork not supported".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| format!("unsupported network {n}")),
        })
    }

    /// Not Found
    fn not_found_example<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 404,
            code: 404,
            message: "Not Found".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Not Found
    pub async fn not_found() -> Self {
        MentatError {
            status_code: 404,
            code: 404,
            message: "Not Found".to_string(),
            description: None,
            retriable: false,
            details: Default::default(),
        }
    }

    /// default error
    pub fn default_error() -> Self {
        MentatError {
            status_code: 0,
            code: 1,
            message: "error".into(),
            description: None,
            retriable: true,
            details: Default::default(),
        }
    }

    /// Endpoint not implemented
    pub fn not_implemented<R>() -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 0,
            message: "Endpoint not implemented".to_string(),
            description: None,
            retriable: false,
            details: Default::default(),
        })
    }

    /// Endpoint unavailable offline
    pub fn unavailable_offline<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 1,
            message: "Endpoint unavailable offline".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Node is not ready
    pub fn node_not_ready<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 2,
            message: "Node is not ready".to_string(),
            description: None,
            retriable: true,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Node error
    pub fn node_error<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 3,
            message: "Node error".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Block not found
    pub fn block_not_found<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 4,
            message: "Block not found".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Unable to derive address
    pub fn unable_to_derive_address<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 5,
            message: "Unable to derive address".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    // TODO not sure what this is
    /// Unable to parse intent
    pub fn bad_intent<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 6,
            message: "Unable to parse intent".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Unable to parse intermediate result
    pub fn bad_intermediate<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 7,
            message: "Unable to parse intermediate result".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Missing ScriptPubKeys
    pub fn missing_keys<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 8,
            message: "Missing ScriptPubKeys".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Coin is invalid
    pub fn invalid_coin<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 9,
            message: "Coin is invalid".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Unable to decode address
    pub fn bad_address<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 10,
            message: "Unable to decode address".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Unable to decode ScriptPubKey
    pub fn bad_pub_key<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 11,
            message: "Unable to decode ScriptPubKey".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Unable to calculate signature hash
    pub fn calc_sig_hash<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 12,
            message: "Unable to calculate signature hash".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Script type is not supported
    pub fn unsupported_script_type<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 13,
            message: "Script type is not supported".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Unable to compute PK script
    pub fn compute_pk_script<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 14,
            message: "Unable to compute PK script".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Unable to get coins
    pub fn failed_to_get_coins<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 15,
            message: "Unable to get coins".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Transaction not found
    pub fn transaction_not_found<D: Display, R>(hash: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 16,
            message: "Transaction not found".to_string(),
            description: None,
            retriable: false,
            details: Self::context(hash, |h| format!("unable to find transaction {h}")),
        })
    }

    /// Could not get suggested fee rate
    pub fn couldnt_get_fee_rate<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 17,
            message: "Could not get suggested fee rate".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }

    /// Unable to get balance
    pub fn couldnt_get_balance<D: Display, R>(details: Option<D>) -> MentatResponse<R> {
        Err(MentatError {
            status_code: 500,
            code: 18,
            message: "Unable to get balance".to_string(),
            description: None,
            retriable: false,
            details: Self::context(details, |n| n.to_string()),
        })
    }
}

impl<T: Display> From<T> for MentatError {
    fn from(e: T) -> Self {
        Self::node_error::<T, ()>(Some(e)).unwrap_err()
    }
}

impl IntoResponse for MentatError {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status_code).unwrap(), Json(self)).into_response()
    }
}

/// allows you to easily return a MentatError instead of a none/err
pub trait MapErrMentat<F> {
    /// the type to return
    type T;
    /// like `map_err` except returns a mentat error containing the output of
    /// the given closure as a string
    fn merr(self, err: F) -> Result<Self::T, MentatError>;
}

impl<T, O: Display, F: FnOnce() -> O> MapErrMentat<F> for Option<T> {
    type T = T;

    fn merr(self, err: F) -> Result<Self::T, MentatError> {
        match self {
            Some(t) => Ok(t),
            None => Err(MentatError::from(err())),
        }
    }
}

impl<T, E, O: Display, F: FnOnce(E) -> O> MapErrMentat<F> for Result<T, E> {
    type T = T;

    fn merr(self, err: F) -> Result<Self::T, MentatError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(MentatError::from(err(e))),
        }
    }
}

/// The Result type for Mentat to always return a `MentatError`.
pub type Result<T, E = MentatError> = std::result::Result<T, E>;
