use mentat_types::*;
use serde::{Deserialize, Serialize};

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
