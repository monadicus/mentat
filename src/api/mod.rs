
mod construction;
use std::net::SocketAddr;

pub use construction::*;

mod data;
pub use data::*;

mod indexer;
pub use indexer::*;

mod result;
pub use result::*;

use crate::requests::*;
use crate::responses::*;

pub struct Caller {
    pub ip: SocketAddr,
}