use from_tuple::FromTuple;

use super::*;

/// CoinIdentifier uniquely identifies a Coin.
#[derive(Clone, Serialize, Deserialize, Debug, Default, FromTuple)]
pub struct CoinIdentifier {
    /// Identifier should be populated with a globally unique identifier of a
    /// Coin. In Bitcoin, this identifier would be transaction_hash:index.
    pub identifier: String,
}
