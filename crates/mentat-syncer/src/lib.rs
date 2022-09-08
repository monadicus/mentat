//! The Syncer package provides support for syncing blocks from any Rosetta Data
//! API implementation

#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![warn(clippy::todo)]

pub mod errors;
use errors::*;
#[cfg(test)]
pub mod errors_test;
pub mod syncer;
#[cfg(test)]
pub mod syncer_test;
pub mod types;
use std::{
    collections::VecDeque,
    mem::size_of_val,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
    thread::{sleep, spawn},
    time::Duration,
};
use types::*;

use mentat_types::*;
use mentat_utils::rust_utils::Context;
use parking_lot::Mutex;
