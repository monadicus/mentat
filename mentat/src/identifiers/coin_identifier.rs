//! The module defines the `CoinIdentifier`.

use from_tuple::FromTuple;

use super::*;

/// `CoinIdentifier` uniquely identifies a Coin.
#[derive(Clone, Debug, Default, Deserialize, FromTuple, Serialize)]
pub struct CoinIdentifier {
    /// Identifier should be populated with a globally unique identifier of a
    /// Coin. In Bitcoin, this identifier would be transaction_hash:index.
    pub identifier: String,
}
