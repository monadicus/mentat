//! The module defines the `CoinAction` model.

use super::*;

/// [`CoinAction`]s are different state changes that a Coin can undergo. It is
/// assumed that a single Coin cannot be created or spent more than once.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CoinAction {
    /// `CoinAction` indicating a Coin was created.
    #[serde(rename = "coin_created")]
    CoinCreated,
    /// `CoinAction` indicating a Coin was spent.
    #[serde(rename = "coin_spent")]
    CoinSpent,
}
