use super::*;

/// ExemptionType is used to indicate if the live balance for an account subject to a BalanceExemption could increase above, decrease below, or equal the computed balance.
#[derive(Serialize, Deserialize, Debug)]
pub enum ExemptionType {
    /// The live balance may increase above or equal the computed balance. This typically occurs with staking rewards that accrue on each block.
    #[serde(rename = "greater_or_equal")]
    GreaterOrEqual,
    /// The live balance may decrease below or equal the computed balance. This typically occurs as balance moves from locked to spendable on a vesting account.
    #[serde(rename = "less_or_equal")]
    LessOrEqual,
    /// The live balance may increase above, decrease below, or equal the computed balance. This typically occurs with tokens that have a dynamic supply.
    #[serde(rename = "dynamic")]
    Dynamic,
}
