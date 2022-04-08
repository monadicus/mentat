//! Defines the different API traits needed for Rosetta.

use std::net::SocketAddr;

use axum::Json;

use crate::server::RpcCaller;

mod call;
pub use call::*;

mod construction;
pub use construction::*;

mod data;
pub use data::*;

mod indexer;
pub use indexer::*;

use crate::{conf::Mode, errors::Result, requests::*, responses::*};

/// The struct to represent the user who called the endpoint.
pub struct Caller {
    /// The socket address of the user who called the end point.
    pub ip: SocketAddr,
}

/// A custom response type for the APIs to avoid duplicate and long types.
pub type MentatResponse<T> = Result<Json<T>>;
