//! The asserter contains tools and methods to help validate the other types.

#![allow(clippy::absurd_extreme_comparisons)]
// TODO this is temporary to help find relevant warnings faster
#![allow(unused)]

use include_dir::{include_dir, Dir};

/// Includes the data dir files;
pub(crate) static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/asserter/data");

mod account;
pub(crate) use account::*;

mod asserter_tools;
pub(crate) use asserter_tools::*;

mod block;
pub(crate) use block::*;

mod coin;
pub(crate) use coin::*;

mod construction;
pub(crate) use construction::*;

mod error;
pub(crate) use error::*;

mod errors;
pub(crate) use errors::*;

mod events;
pub(crate) use events::*;

mod mempool;
pub(crate) use mempool::*;

mod network;
pub(crate) use network::*;

mod search;
pub(crate) use search::*;

mod server;
pub(crate) use server::*;

mod util;
pub(crate) use util::*;

use crate::types::*;

#[cfg(test)]
#[path = ""]
mod tests {
    pub use super::*;
    use crate::tests::Test;

    mod account_test;
    mod asserter_test;
    mod block_test;
    mod coin_test;
    mod construction_test;
    mod error_test;
    mod errors_test;
    mod events_test;
    mod network_test;
    mod search_test;
    mod server_test;
    pub(crate) use server_test::*;
    mod test_utils;
    pub(crate) use test_utils::*;
}
