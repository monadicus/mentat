//! The asserter contains tools and methods to help validate the other types.

#![allow(clippy::absurd_extreme_comparisons)]
mod account;
use account::*;
mod asserter_tools;
use asserter_tools::*;
mod block;
use block::*;
mod coin;
use coin::*;
mod construction;
use construction::*;
mod error;
use error::*;
mod errors;
use errors::*;
mod events;
use events::*;
mod mempool;
use mempool::*;
mod network;
use network::*;
mod search;
use search::*;
mod server;
use server::*;
mod util;
use util::*;

use crate::types::*;

#[cfg(test)]
#[path = ""]
mod tests {
    mod account_test;
}
