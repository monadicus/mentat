//! The module defines the `CoinAction` model.

use super::*;

/// [`CoinAction`]s are different state changes that a Coin can undergo. It is
/// assumed that a single Coin cannot be created or spent more than once.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CoinAction(pub String);

impl CoinAction {
    /// `CoinAction` indicating a Coin was created.
    pub const COIN_CREATED: &'static str = "coin_created";
    /// `CoinAction` indicating a Coin was spent.
    pub const COIN_SPENT: &'static str = "coin_spent";

    pub fn valid(&self) -> bool {
        match self.0.as_str() {
            Self::COIN_CREATED | Self::COIN_SPENT => true,
            _ => false,
        }
    }
}
