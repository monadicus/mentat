use std::net::SocketAddr;

use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{errors::MentatError, server::RpcCaller};

mod additional;
pub use additional::*;

mod construction;
pub use construction::*;

mod call;
pub use call::*;

mod data;
pub use data::*;

mod indexer;
pub use indexer::*;

use crate::{conf::Mode, errors::Result, requests::*, responses::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Caller {
    pub ip: SocketAddr,
}

pub type MentatResponse<T> = Result<Json<T>>;
