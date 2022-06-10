//! The asserter contains tools and methods to help validate the other types.

#![allow(clippy::absurd_extreme_comparisons)]
mod account;
mod asserter_tools;
mod block;
mod coin;
mod construction;
mod error;
mod errors;
mod events;
mod mempool;
mod network;
mod search;
mod server;
mod util;

#[cfg(test)]
#[path = ""]
mod tests {
    mod account_test;
}
