//! The module defines the `CoinAction` model.

use std::fmt;

use super::*;

/// [`CoinAction`]s are different state changes that a Coin can undergo. It is
/// assumed that a single Coin cannot be created or spent more than once.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CoinAction(String);

impl CoinAction {
    /// `CoinAction` indicating a Coin was created.
    pub const COIN_CREATED: &'static str = "coin_created";
    /// `CoinAction` indicating a Coin was spent.
    pub const COIN_SPENT: &'static str = "coin_spent";

    /// returns true if the `CoinAction` is a valid type
    pub fn valid(&self) -> bool {
        matches!(self.0.as_str(), Self::COIN_CREATED | Self::COIN_SPENT)
    }
}

impl fmt::Display for CoinAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CoinAction {
    fn from(act: String) -> Self {
        Self(act)
    }
}

impl From<&str> for CoinAction {
    fn from(act: &str) -> Self {
        act.to_string().into()
    }
}
