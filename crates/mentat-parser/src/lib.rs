//! The parser module

#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![warn(clippy::todo)]

mod balance_changes;
pub use balance_changes::*;

mod errors;
pub use errors::*;

mod exemptions;
pub use exemptions::*;

mod group_operations;
pub use group_operations::*;

mod intent;
pub use intent::*;

mod match_operations;
pub use match_operations::*;

mod parser;
use indexmap::IndexMap;
use mentat_asserter::*;
use mentat_types::*;
pub use parser::*;
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = ""]
pub mod tests {
    use mentat_test_utils::*;

    use super::*;

    mod balance_changes_test;
    mod errors_test;
    mod exemptions_test;
    mod group_operations_test;
    mod intent_test;
    mod match_operations_test;
}
