//! The module defines the `AccountCoin` model.

use super::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountCurrency {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<AccountIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<Currency>,
}
