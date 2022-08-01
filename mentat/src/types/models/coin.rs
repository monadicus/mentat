//! The module defines the `Coin` model.

use mentat_macros::Nullable;

use super::*;

/// [`Coin`] contains its unique identifier and the amount it represents.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableCoin {
    /// [`CoinIdentifier`] uniquely identifies a Coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_identifier: Option<CoinIdentifier>,
    /// [`Amount`] is some Value of a [`Currency`]. It is considered invalid to
    /// specify a Value without a [`Currency`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<NullableAmount>,
}
