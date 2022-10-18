#![warn(clippy::todo, clippy::use_debug)]

// TODO gated everything behind test macro because this is NOT a proper implementation of storage

mod database;
mod encoder;
mod errors;
mod modules;

pub use database::Transaction;

#[cfg(feature = "incomplete")]
pub use database::*;
