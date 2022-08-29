//! The module defines the `AccountCoin` model.

use super::*;

/// `AccountCoin` contains an [`AccountIdentifier`] and a [`Coin`] that it owns.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedAccountCoin {
    /// the `AccountIdentifier` that owns the [`Coin`]
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<AccountIdentifier>,
    /// the `Coin` that the [`AccountIdentifier`] owns
    #[serde(skip_serializing_if = "Option::is_none")]
    coin: Option<UncheckedCoin>,
}
