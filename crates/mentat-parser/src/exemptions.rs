//! TODO

use std::str::FromStr;

use num_bigint_dig::*;

use super::*;

/// `match_balance_exemption` returns a [`BalanceExemption`]
/// associated with the [`AccountIdentifier`], [`Currency`],
/// and difference, if it exists. The provided exemptions
/// should be produced using `find_exemptions`.
pub fn match_balance_exemption(
    matched_exemptions: Vec<BalanceExemption>,
    difference: &str,
) -> Option<BalanceExemption> {
    let big = match BigInt::from_str(difference) {
        Ok(big) => big,
        Err(_) => return None,
    };

    matched_exemptions.into_iter().find(|e| {
        (matches!(e.exemption_type, Some(ExemptionType::Dynamic)))
            || (matches!(e.exemption_type, Some(ExemptionType::GreaterOrEqual))
                && matches!(big.sign(), Sign::Plus | Sign::NoSign))
            || (matches!(e.exemption_type, Some(ExemptionType::LessOrEqual))
                && matches!(big.sign(), Sign::Minus | Sign::NoSign))
    })
}

impl Parser {
    /// `find_exemptions` returns all matching [`BalanceExemption`]
    /// for a particular [`AccountIdentifier`] and [`Currency`].
    pub fn find_exemptions(
        &self,
        account: &AccountIdentifier,
        currency: Option<&Currency>,
    ) -> Vec<BalanceExemption> {
        let mut matches = Vec::new();

        for be in self.balance_exemptions.iter() {
            if be.currency.is_some() && hash(currency) != hash(be.currency.as_ref()) {
                continue;
            }

            if (be.sub_account_address.is_some())
                && (account.sub_account.is_none()
                    || be.sub_account_address
                        != account.sub_account.as_ref().map(|sa| sa.address.clone()))
            {
                continue;
            }

            matches.push(be.clone());
        }

        matches
    }
}
