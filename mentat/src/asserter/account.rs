use std::collections::HashSet;

use super::{
    block::{amount, block_identifier},
    coins::coins,
    errors::{AccountBalanceError, AssertResult},
    util::hash,
};
use crate::{
    identifiers::PartialBlockIdentifier,
    models::{Amount, Currency},
    responses::{AccountBalanceResponse, AccountCoinsResponse},
};

/// `contains_duplicate_currency` retruns a boolean indicating
/// if an array of [`Currency`] contains any duplicate currencies.
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

/// `contains_currency` returns a boolean indicating if a
/// [`Currency`] is contained within a slice of
/// [`Currency`]. The check for equality takes
/// into account everything within the [`Currency`]
/// struct (including currency.Metadata).
fn contains_currency(currencies: &[Currency], currency: &Currency) -> bool {
    currencies.iter().any(|other| other == currency)
}

/// `assert_unique_amounts` returns an error if a slice
/// of [`Amount`] is invalid. It is considered invalid if the same
/// currency is returned multiple times (these should be
/// consolidated) or if a [`Amount`] is considered invalid.
fn assert_unique_amounts(amounts: &[Amount]) -> AssertResult<()> {
    let mut seen = HashSet::new();

    for amt in amounts.iter() {
        let key = hash(&amt.currency);

        if seen.contains(&key) {
            return Err(format!("currency {:?} used multiple times", amt.currency).into());
        }

        seen.insert(key);

        amount(Some(amt))?;
    }

    Ok(())
}

/// `account_balance_response` returns an error if the provided
/// [`PartialBlockIdentifier`] is invalid, if the requestBlock
/// is not nil and not equal to the response block, or
/// if the same currency is present in multiple amounts.
fn account_balance_response(
    request_block: &PartialBlockIdentifier,
    response: &AccountBalanceResponse,
) -> AssertResult<()> {
    block_identifier(&response.block_identifier)
        .map_err(|e| format!("{e}: block identifier is invalid"))?;
    assert_unique_amounts(&response.balances)
        .map_err(|e| format!("{e}: balance amounts are invalid"));
    // if request.block == nil
    if matches!(request_block.hash, Some(i) if i == response.block_identifier.hash) {
        Err(format!(
            "{}: requested block hash {} but got {}",
            AccountBalanceError::ReturnedBlockHashMismatch,
            request_block.hash.unwrap(),
            response.block_identifier.hash
        )
        .into())
    } else if matches!(request_block.index, Some(i) if i == response.block_identifier.index) {
        Err(format!(
            "{}: requested block index {} but got {}",
            AccountBalanceError::ReturnedBlockIndexMismatch,
            request_block.index.unwrap(),
            response.block_identifier.index
        )
        .into())
    } else {
        Ok(())
    }
}

/// `account_coins` returns an error if the provided
/// [`AccountCoinsResponse`] is invalid.
fn account_coins(response: &AccountCoinsResponse) -> AssertResult<()> {
    block_identifier(&response.block_identifier)
        .map_err(|e| format!("{e}: block identifier is invalid"))?;
    coins(&response.coins).map_err(|e| format!("{e}: coins are invalid"))?;
    Ok(())
}
