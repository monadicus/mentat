mod construction;
use std::net::SocketAddr;

use rocket::serde::json::Json;

pub use construction::*;

mod data;
pub use data::*;

mod indexer;
pub use indexer::*;

use crate::errors::Result;
use crate::requests::*;
use crate::responses::*;

pub struct Caller {
    pub ip: SocketAddr,
}

pub type Response<T> = Result<Json<T>>;
