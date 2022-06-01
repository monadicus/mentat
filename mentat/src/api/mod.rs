//! Defines the different API traits needed for Rosetta.

use axum::Json;

use crate::{errors::MentatError, server::RpcCaller};

mod optional;
pub use optional::*;

mod call;
pub use call::*;

mod construction;
pub use construction::*;

mod data;
pub use data::*;

mod indexer;
pub use indexer::*;

use crate::{conf::Mode, errors::Result, models::Caller, requests::*, responses::*};

/// A custom response type for the APIs to avoid duplicate and long types.
pub type MentatResponse<T> = Result<Json<T>>;
