//! The asserter contains tools and methods to help validate the other types.

#![allow(clippy::absurd_extreme_comparisons, unused)]

use include_dir::{include_dir, Dir};

/// Includes the data dir files;
#[allow(unused)]
const DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/asserter/data");

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
use util::*;

use crate::types::*;

#[cfg(test)]
#[path = ""]
pub mod tests {
    use super::*;
    use crate::tests::{status_message, Test};

    mod account_test;
    mod asserter_tools_test;
    mod block_test;
    mod coin_test;
    mod construction_test;
    mod error_test;
    mod errors_test;
    mod events_test;
    mod network_test;
    mod search_test;
    mod server_test;
    use server_test::*;
    mod test_utils;
    use test_utils::*;
}
