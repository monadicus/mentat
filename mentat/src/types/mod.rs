//! The types module as according to the rosetta sdk specification.

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
pub(crate) use utils::*;

/// TODO NEEDS DOCS
pub trait Sortable {
    /// TODO NEEDS DOCS
    fn sort(&self) -> Self;
}

const ROSETTA_API_VERSION: &str = "1.4.12";
