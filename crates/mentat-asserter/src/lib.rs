//! The asserter contains tools and methods to help validate the other types.

#![allow(clippy::absurd_extreme_comparisons, unused)]

use include_dir::{include_dir, Dir};
use indexmap::{IndexMap, IndexSet};
use mentat_types::*;
use serde_json::{json, Value};

/// Includes the data dir files;
#[allow(unused)]
const DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data");

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

#[cfg(test)]
#[path = ""]
pub mod tests {
    use indexmap::indexmap;
    use mentat_test_utils::*;

    use super::*;

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
}
