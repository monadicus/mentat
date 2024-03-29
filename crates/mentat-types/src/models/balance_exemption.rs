//! The module defines the `BalanceExemption` model.

use super::*;

/// [`BalanceExemption`] indicates that the balance for an exempt account could
/// change without a corresponding [`Operation`]. This typically occurs with
/// staking rewards, vesting balances, and Currencies with a dynamic supply.
/// Currently, it is possible to exempt an account from strict reconciliation by
/// [`SubAccountIdentifier`]. Address or by [`Currency`]. This means that any
/// account with [`SubAccountIdentifier`]. Address would be exempt or any
/// balance of a particular [`Currency`] would be exempt, respectively.
/// [`BalanceExemption`]s should be used sparingly as they may introduce
/// significant complexity for integrators that attempt to reconcile all account
/// balance changes. If your implementation relies on any `[BalanceExemption]`s,
/// you MUST implement historical balance lookup (the ability to query an
/// account balance at any [`BlockIdentifier`]).
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedBalanceExemption {
    /// SubAccountAddress is the [`SubAccountIdentifier`]. Address that the
    /// BalanceExemption applies to (regardless of the value of
    /// [`SubAccountIdentifier`].Metadata).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub sub_account_address: Option<String>,
    /// `Currency` is composed of a canonical Symbol and Decimals. This Decimals
    /// value is used to convert an Amount.Value from atomic units (Satoshis) to
    /// standard units (Bitcoins).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[unchecked(retain)]
    pub currency: Option<UncheckedCurrency>,
    /// `ExemptionType` is used to indicate if the live balance for an account
    /// subject to a [`BalanceExemption`] could increase above, decrease below,
    /// or equal the computed balance. * `greater_or_equal`: The live
    /// balance may increase above or equal the computed balance. This
    /// typically occurs with staking rewards that accrue on each block. *
    /// `less_or_equal`: The live balance may decrease below or equal the
    /// computed balance. This typically occurs as balance moves from locked
    /// to spendable on a vesting account. * dynamic: The live balance may
    /// increase above, decrease below, or equal the computed balance. This
    /// typically occurs with tokens that have a dynamic supply.
    #[unchecked(option_enum)]
    pub exemption_type: UncheckedExemptionType,
}
