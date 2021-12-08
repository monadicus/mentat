use super::*;

/// CoinIdentifier uniquely identifies a Coin.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CoinIdentifier {
    /// Identifier should be populated with a globally unique identifier of a Coin. In Bitcoin, this identifier would be transaction_hash:index.
    pub identifier: String,
}