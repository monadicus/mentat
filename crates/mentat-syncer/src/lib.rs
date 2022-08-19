//! The Syncer package provides support for syncing blocks from any Rosetta Data API
//! implementation

#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![warn(clippy::todo)]

pub mod errors;
#[cfg(test)]
pub mod errors_test;
// mod golang;
pub mod syncer;
#[cfg(test)]
pub mod syncer_test;
pub mod types;