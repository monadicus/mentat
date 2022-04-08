use std::net::SocketAddr;

use axum::Json;

use crate::server::RpcCaller;

mod construction;
pub use construction::*;

mod call;
pub use call::*;

mod data;
pub use data::*;

mod indexer;
pub use indexer::*;

use crate::{conf::Mode, errors::Result, requests::*, responses::*};

pub struct Caller {
    pub ip: SocketAddr,
}

pub type MentatResponse<T> = Result<Json<T>>;
