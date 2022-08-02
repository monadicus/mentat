use mentat_asserter::Asserter;
use mentat_types::{BalanceExemption, Operation};

pub struct Parser<ExemptOperation>
where
    ExemptOperation: FnOnce(Option<Operation>) -> bool,
{
    pub asserter: Asserter,
    pub exempt_func: ExemptOperation,
    pub balance_exemptions: Vec<Option<BalanceExemption>>,
}

impl<ExemptOperation> Parser<ExemptOperation>
where
    ExemptOperation: FnOnce(Option<Operation>) -> bool,
{
    pub fn new(
        asserter: Asserter,
        exempt_func: ExemptOperation,
        balance_exemptions: Vec<Option<BalanceExemption>>,
    ) -> Self {
        Self {
            asserter,
            exempt_func,
            balance_exemptions,
        }
    }
}
