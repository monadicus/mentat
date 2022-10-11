//! The module defines the `CoinIdentifier`.

use super::*;

/// [`CoinIdentifier`] uniquely identifies a Coin.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
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

impl From<String> for CoinIdentifier {
    fn from(identifier: String) -> Self {
        Self { identifier }
    }
}

impl From<&str> for CoinIdentifier {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}
