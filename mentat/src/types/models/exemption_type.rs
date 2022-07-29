//! The module defines the `ExemptionType` model.

use std::fmt;

use super::*;

/// `ExemptionType` is used to indicate if the live balance for an account
/// subject to a `BalanceExemption` could increase above, decrease below, or
/// equal the computed balance.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NullableExemptionType(String);

impl NullableExemptionType {
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

    /// returns true if the `ExemptionType` is a valid type
    pub fn valid(&self) -> bool {
        matches!(
            self.0.as_str(),
            Self::GREATER_OR_EQUAL | Self::LESS_OR_EQUAL | Self::DYNAMIC
        )
    }
}

impl fmt::Display for NullableExemptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NullableExemptionType {
    fn from(et: String) -> Self {
        Self(et)
    }
}

impl From<&str> for NullableExemptionType {
    fn from(et: &str) -> Self {
        et.to_string().into()
    }
}

/// `ExemptionType` is used to indicate if the live balance for an account
/// subject to a `BalanceExemption` could increase above, decrease below, or
/// equal the computed balance.
#[derive(Default, Debug, Clone)]
pub enum ExemptionType {
    /// The live balance may increase above, decrease below, or equal the
    /// computed balance. This typically occurs with tokens that have a dynamic
    /// supply.
    #[default]
    Dynamic,
    /// The live balance may increase above or equal the computed balance. This
    /// typically occurs with staking rewards that accrue on each block.
    GreaterOrEqual,
    /// The live balance may decrease below or equal the computed balance. This
    /// typically occurs as balance moves from locked to spendable on a vesting
    /// account.
    LessOrEqual,
}

impl From<NullableExemptionType> for ExemptionType {
    fn from(other: NullableExemptionType) -> Self {
        match other.0.as_ref() {
            NullableExemptionType::DYNAMIC => Self::Dynamic,
            NullableExemptionType::GREATER_OR_EQUAL => Self::GreaterOrEqual,
            NullableExemptionType::LESS_OR_EQUAL => Self::LessOrEqual,
            i => panic!("unsupported ExemptionType: {i}"),
        }
    }
}

impl From<ExemptionType> for NullableExemptionType {
    fn from(other: ExemptionType) -> Self {
        match other {
            ExemptionType::Dynamic => Self::DYNAMIC.into(),
            ExemptionType::GreaterOrEqual => Self::GREATER_OR_EQUAL.into(),
            ExemptionType::LessOrEqual => Self::LESS_OR_EQUAL.into(),
        }
    }
}
