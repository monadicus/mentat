//! The parser struct itself.

use super::*;

/// a closure that determines if an operation should be skipped
pub type ExemptionFunc = Box<dyn Fn(&Operation) -> bool>;

/// `Parser` provides support for parsing Rosetta blocks.
#[allow(clippy::missing_docs_in_private_items)]
pub struct Parser {
    pub asserter: Option<Asserter>,
    pub exempt_func: Option<ExemptionFunc>,
    pub balance_exemptions: Vec<BalanceExemption>,
}

impl Parser {
    /// creates a new `Parser`.
    pub fn new(
        asserter: Option<Asserter>,
        exempt_func: Option<ExemptionFunc>,
        balance_exemptions: Vec<BalanceExemption>,
    ) -> Self {
        Self {
            asserter,
            exempt_func,
            balance_exemptions,
        }
    }
}
