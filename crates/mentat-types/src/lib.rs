//! The types module as according to the rosetta sdk specification.

#![warn(clippy::todo, clippy::use_debug)]

use std::mem::size_of_val;

use indexmap::IndexMap;
use mentat_macros::Unchecked;
use serde::{Deserialize, Serialize};

mod errors;
pub use errors::*;
mod identifiers;
pub use identifiers::*;
mod misc;
pub use misc::*;
mod models;
pub use models::*;
mod requests;
pub use requests::*;
mod responses;
pub use responses::*;
mod utils;
pub use utils::*;

// TODO: are we missing case? https://github.dev/coinbase/rosetta-sdk-go/blob/136b591fb3f410ac0fd47f38f8eefb6a1a19e1d8/types/case.go

/// TODO NEEDS DOCS
pub trait Sortable {
    /// TODO NEEDS DOCS
    fn sort(&self) -> Self;
}

/// the current rosetta api version
#[allow(unused)]
const ROSETTA_API_VERSION: &str = "1.4.12";

#[cfg(test)]
#[path = ""]
mod tests {
    use super::*;

    mod serialize_test;
    mod utils_test;
}
