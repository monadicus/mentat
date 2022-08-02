use mentat_types::*;
use serde::{Deserialize, Serialize};

use crate::{Parser, ParserResult};

/// `BalanceChange` represents a balance change that affected
/// a [`AccountIdentifier`] and a [`Currency`].
#[derive(Debug, Deserialize, Serialize)]
pub struct BalanceChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockIdentifier>,
    pub difference: String,
}

impl<ExemptOperation> Parser<ExemptOperation>
where
    ExemptOperation: FnOnce(Option<Operation>) -> bool,
{
    // pub fn skip_operation(&self, op: Operation) -> ParserResult<bool> {
    //     let op = self.asserter
    // }
}
