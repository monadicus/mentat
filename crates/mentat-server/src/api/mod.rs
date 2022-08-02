//! Defines the different API traits needed for Rosetta.

use axum::Json;

use crate::{conf::Mode, server::RpcCaller};

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
use mentat_asserter::*;
use mentat_types::*;
