use std::collections::HashSet;

use super::{block::amount, util::hash};
use crate::{
    identifiers::PartialBlockIdentifier,
    models::{Amount, Currency},
    responses::{AccountBalanceResponse, AccountCoinsResponse},
};

fn contains_duplicate_currency(currencies: &[Currency]) -> Option<Currency> {
    let mut seen = HashSet::new();

    for currency in currencies.iter() {
        let key = hash(currency);

        if seen.contains(&key) {
            return Some(currency.clone());
        }

        seen.insert(key);
    }

    return None;
}

fn contains_currency(currencies: &[Currency], currency: &Currency) -> bool {
    currencies.iter().any(|other| other == currency)
}

fn assert_unique_amounts(amounts: &[Amount]) -> Result<(), String> {
    let mut seen = HashSet::new();

    for amt in amounts.iter() {
        let key = hash(&amt.currency);

        if seen.contains(&key) {
            return Err(format!("currency {:?} used multiple times", amt.currency));
        }

        seen.insert(key);

        amount(amt)?;
    }

    Ok(())
}

fn account_balance_response(
    request_block: &PartialBlockIdentifier,
    response: &AccountBalanceResponse,
) -> Result<(), String> {
    todo!()
}

fn account_coins(response: &AccountCoinsResponse) -> Result<(), String> {
    todo!()
}
