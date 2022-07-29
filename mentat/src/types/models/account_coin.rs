//! The module defines the `AccountCoin` model.

use mentat_macros::Nullable;

use super::*;

/// `AccountCoin` contains an [`AccountIdentifier`] and a [`Coin`] that it owns.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableAccountCoin {
    /// the `AccountIdentifier` that owns the [`Coin`]
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<AccountIdentifier>,
    /// the `Coin` that the [`AccountIdentifier`] owns
    #[serde(skip_serializing_if = "Option::is_none")]
    coin: Option<NullableCoin>,
}
