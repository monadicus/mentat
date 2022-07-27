//! The module defines the `AccountCoin` model.

use mentat_macros::Nullable;

use super::*;

/// `AccountCurrency` is a simple struct combining
/// an [`AccountIdentifier`] and [`Currency`]. This can
/// be useful for looking up balances.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableAccountCurrency {
    /// the identifier for the [`Account`]
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<AccountIdentifier>,
    /// the currency used by the [`Account`]
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<NullableCurrency>,
}