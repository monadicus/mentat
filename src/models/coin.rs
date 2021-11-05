use super::*;

/// Coin contains its unique identifier and the amount it represents.
#[derive(Serialize, Deserialize)]
pub struct Coin {
    /// CoinIdentifier uniquely identifies a Coin.
    pub coin_identifier: CoinIdentifier,
    /// Amount is some Value of a Currency. It is considered invalid to specify a Value without a Currency.
    pub amount: Amount,
}