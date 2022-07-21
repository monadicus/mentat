//! The module defines the `ExemptionType` model.

use super::*;

/// `ExemptionType` is used to indicate if the live balance for an account
/// subject to a `BalanceExemption` could increase above, decrease below, or
/// equal the computed balance.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ExemptionType(pub String);

impl ExemptionType {
    /// The live balance may increase above, decrease below, or equal the
    /// computed balance. This typically occurs with tokens that have a dynamic
    /// supply.
    pub const DYNAMIC: &'static str = "dynamic";
    /// The live balance may increase above or equal the computed balance. This
    /// typically occurs with staking rewards that accrue on each block.
    pub const GREATER_OR_EQUAL: &'static str = "greater_or_equal";
    /// The live balance may decrease below or equal the computed balance. This
    /// typically occurs as balance moves from locked to spendable on a vesting
    /// account.
    pub const LESS_OR_EQUAL: &'static str = "less_or_equal";

    pub fn valid(&self) -> bool {
        match self.0.as_str() {
            Self::GREATER_OR_EQUAL | Self::LESS_OR_EQUAL | Self::DYNAMIC => true,
            _ => false,
        }
    }
}
