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

/// NEEDS DOCS
pub trait Sortable {
    /// NEEDS DOCS
    fn sort(&self) -> Self;
}
