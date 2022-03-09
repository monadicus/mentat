use std::net::SocketAddr;

use rocket::serde::json::Json;

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

pub type Response<T> = Result<Json<T>>;
