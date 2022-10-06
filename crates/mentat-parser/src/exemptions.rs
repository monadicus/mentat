//! handles parsing of [`BalanceExemption`]'s for associated accounts/currencies

use std::str::FromStr;

use num_bigint_dig::*;

use super::*;

/// `match_balance_exemption` returns a [`BalanceExemption`]
/// associated with the [`AccountIdentifier`], [`Currency`],
/// and difference, if it exists. The provided exemptions
/// should be produced using `find_exemptions`.
pub fn match_balance_exemption<'a>(
    matched_exemptions: &[&'a BalanceExemption],
    difference: &str,
) -> Option<&'a BalanceExemption> {
    let big = match BigInt::from_str(difference) {
        Ok(big) => big,
        Err(_) => return None,
    };

    matched_exemptions
        .iter()
        .find(|e| {
            (matches!(e.exemption_type, Some(ExemptionType::Dynamic)))
                || (matches!(e.exemption_type, Some(ExemptionType::GreaterOrEqual))
                    && matches!(big.sign(), Sign::Plus | Sign::NoSign))
                || (matches!(e.exemption_type, Some(ExemptionType::LessOrEqual))
                    && matches!(big.sign(), Sign::Minus | Sign::NoSign))
        })
        .copied()
}

impl Parser {
    /// `find_exemptions` returns all matching [`BalanceExemption`]
    /// for a particular [`AccountIdentifier`] and [`Currency`].
    pub fn find_exemptions(
        &self,
        account: &AccountIdentifier,
        currency: Option<&Currency>,
    ) -> Vec<&BalanceExemption> {
        self.balance_exemptions
            .iter()
            .filter(|be| {
                (be.currency.is_none() || hash(currency) == hash(be.currency.as_ref()))
                    && (be.sub_account_address.is_none()
                        || account.sub_account.is_some()
                            && be.sub_account_address.as_ref()
                                == account.sub_account.as_ref().map(|sa| &sa.address))
            })
            .collect()
    }
}
