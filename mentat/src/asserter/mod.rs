//! The asserter contains tools and methods to help validate the other types.

#![allow(clippy::absurd_extreme_comparisons)]
// TODO this is temporary to help find relevant warnings faster
#![allow(unused)]

use include_dir::{include_dir, Dir};

/// Includes the data dir files;
pub static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/asserter/data");

mod account;
pub use account::*;

mod asserter_tools;
pub use asserter_tools::*;

mod block;
pub use block::*;

mod coin;
pub use coin::*;

mod construction;
pub use construction::*;

mod error;
pub use error::*;

mod errors;
pub use errors::*;

mod events;
pub use events::*;

mod mempool;
pub use mempool::*;

mod network;
pub use network::*;

mod search;
pub use search::*;

mod server;
pub use server::*;

mod util;
pub use util::*;

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
    pub use server_test::*;
    mod test_utils;
    pub use test_utils::*;
}
