//! The parser struct itself.

use mentat_asserter::Asserter;
use mentat_types::{BalanceExemption, Operation};

/// `Parser` provides support for parsing Rosetta blocks.
#[allow(clippy::missing_docs_in_private_items)]
pub struct Parser<ExemptOperation>
where
    ExemptOperation: Fn(&Operation) -> bool,
{
    pub asserter: Asserter,
    pub exempt_func: Option<ExemptOperation>,
    pub balance_exemptions: Vec<Option<BalanceExemption>>,
}

impl<ExemptOperation> Parser<ExemptOperation>
where
    ExemptOperation: Fn(&Operation) -> bool,
{
    /// creates a new `Parser`.
    pub fn new(
        asserter: Asserter,
        exempt_func: Option<ExemptOperation>,
        balance_exemptions: Vec<Option<BalanceExemption>>,
    ) -> Self {
        Self {
            asserter,
            exempt_func,
            balance_exemptions,
        }
    }
}
