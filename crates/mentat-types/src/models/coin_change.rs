//! The module defines the `CoinChange` model.

use super::*;

/// [`CoinChange`] is used to represent a change in state of a some coin
/// identified by a coin_identifier. This object is part of the [`Operation`]
/// model and must be populated for UTXO-based blockchains. Coincidentally, this
/// abstraction of UTXOs allows for supporting both account-based transfers and
/// UTXO-based transfers on the same blockchain (when a transfer is
/// account-based, don't populate this model).
#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq, Unchecked)]
#[serde(default)]
pub struct UncheckedCoinChange {
    /// [`CoinIdentifier`] uniquely identifies a Coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_identifier: Option<CoinIdentifier>,
    /// [`CoinAction`]s are different state changes that a [`Coin`] can undergo.
    /// When a [`Coin`] is created, it is coin_created. When a [`Coin`] is
    /// spent, it is coin_spent. It is assumed that a single [`Coin'] cannot
    /// be created or spent more than once.
    pub coin_action: UncheckedCoinAction,
}

impl EstimateSize for CoinChange {
    fn estimated_size(&self) -> usize {
        size_of_val(self) + self.coin_identifier.estimated_size()
    }
}
