//! Defines the different API traits needed for Rosetta.

mod account;
pub use account::*;
mod block;
pub use block::*;
mod call;
pub use call::*;
mod construction;
pub use construction::*;
mod events;
pub use events::*;
mod mempool;
pub use mempool::*;
mod network;
pub use network::*;
mod optional;
pub use optional::*;
mod search;
use axum::Json;
use mentat_asserter::Asserter;
use mentat_types::*;
pub use search::*;

use crate::conf::Mode;
