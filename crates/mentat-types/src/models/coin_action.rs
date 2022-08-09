//! The module defines the `CoinAction` model.

use std::fmt;

use super::*;

/// [`CoinAction`]s are different state changes that a Coin can undergo. It is
/// assumed that a single Coin cannot be created or spent more than once.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct NullableCoinAction(String);

impl NullableCoinAction {
    /// `CoinAction` indicating a Coin was created.
    pub const COIN_CREATED: &'static str = "coin_created";
    /// `CoinAction` indicating a Coin was spent.
    pub const COIN_SPENT: &'static str = "coin_spent";

    /// returns true if the `CoinAction` is a valid type
    pub fn valid(&self) -> bool {
        matches!(self.0.as_str(), Self::COIN_CREATED | Self::COIN_SPENT)
    }

    /// returns true if the underlying string is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for NullableCoinAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NullableCoinAction {
    fn from(act: String) -> Self {
        Self(act)
    }
}

impl From<&str> for NullableCoinAction {
    fn from(act: &str) -> Self {
        act.to_string().into()
    }
}

/// [`CoinAction`]s are different state changes that a Coin can undergo. It is
/// assumed that a single Coin cannot be created or spent more than once.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CoinAction {
    #[cfg(test)]
    None,
    /// `CoinAction` indicating a Coin was created.
    CoinCreated,
    /// `CoinAction` indicating a Coin was spent.
    CoinSpent,
}

impl Default for CoinAction {
    fn default() -> Self {
        #[cfg(test)]
        return Self::None;

        #[cfg(not(test))]
        return Self::CoinCreated;
    }
}

impl From<NullableCoinAction> for CoinAction {
    fn from(other: NullableCoinAction) -> Self {
        match other.0.as_ref() {
            NullableCoinAction::COIN_CREATED => Self::CoinCreated,
            NullableCoinAction::COIN_SPENT => Self::CoinSpent,
            i => panic!("unsupported CoinAction: {i}"),
        }
    }
}

impl From<CoinAction> for NullableCoinAction {
    fn from(other: CoinAction) -> Self {
        match other {
            CoinAction::CoinCreated => Self::COIN_CREATED.into(),
            CoinAction::CoinSpent => Self::COIN_SPENT.into(),
            #[cfg(test)]
            CoinAction::None => unreachable!("Only reachable in test mode"),
        }
    }
}

impl fmt::Display for CoinAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoinAction::CoinCreated => write!(f, "coin_created"),
            CoinAction::CoinSpent => write!(f, "coin_spent"),
            #[cfg(test)]
            CoinAction::None => unreachable!("Only reachable in test mode"),
        }
    }
}
