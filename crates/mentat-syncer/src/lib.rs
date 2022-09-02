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
use types::*;
mod utils;
use std::{
    collections::VecDeque,
    mem::size_of_val,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
        Weak,
    },
    thread::{sleep, spawn, JoinHandle},
    time::{Duration, Instant},
};

use mentat_types::*;
use parking_lot::Mutex;
use utils::*;
