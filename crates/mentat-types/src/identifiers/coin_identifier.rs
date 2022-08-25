//! The module defines the `CoinIdentifier`.

use std::mem::size_of_val;

use from_tuple::FromTuple;

use super::*;

/// [`CoinIdentifier`] uniquely identifies a Coin.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct CoinIdentifier {
    /// Identifier should be populated with a globally unique identifier of a
    /// Coin. In Bitcoin, this identifier would be transaction_hash:index.
    pub identifier: String,
}

impl EstimateSize for CoinIdentifier {
    fn estimated_size(&self) -> usize {
        size_of_val(self) + size_of_val(self.identifier.as_str())
    }
}
