use std::net::SocketAddr;

use axum::Json;

mod construction;
pub use construction::*;

mod call;
pub use call::*;

mod data;
pub use data::*;

mod indexer;
pub use indexer::*;

mod mode;
pub use mode::*;

use crate::errors::{ApiError, Result};
use crate::requests::*;
use crate::responses::*;

pub struct Caller {
    pub ip: SocketAddr,
}

pub type MentantResponse<T> = Result<Json<T>>;
