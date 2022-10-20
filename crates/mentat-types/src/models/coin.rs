//! The module defines the `Coin` model.

use super::*;

/// [`Coin`] contains its unique identifier and the amount it represents.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default, deny_unknown_fields)]
pub struct UncheckedCoin {
    /// [`Amount`] is some Value of a [`Currency`]. It is considered invalid to
    /// specify a Value without a [`Currency`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<UncheckedAmount>,
    /// [`CoinIdentifier`] uniquely identifies a Coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_identifier: Option<CoinIdentifier>,
}
