//! Validates that account data is correct.

use std::collections::HashSet;

use super::*;

/// `contains_duplicate_currency` returns a boolean indicating
/// if an array of [`Currency`] contains any duplicate currencies.
pub fn contains_duplicate_currency<'a>(
    currencies: &[Option<&'a UncheckedCurrency>],
) -> Option<&'a UncheckedCurrency> {
    let mut seen = HashSet::new();

    for currency in currencies.iter() {
        let key = hash(*currency);

        if seen.contains(&key) {
            return *currency;
        }

        seen.insert(key);
    }

    None
}

/// `contains_currency` returns a boolean indicating if a
/// [`Currency`] is contained within a slice of
/// [`Currency`]. The check for equality takes
/// into account everything within the [`Currency`]
/// struct (including currency.Metadata).
pub fn contains_currency(currencies: &[Currency], currency: &Currency) -> bool {
    currencies.iter().any(|other| other == currency)
}

/// `assert_unique_amounts` returns an error if a slice
/// of [`Amount`] is invalid. It is considered invalid if the same
/// currency is returned multiple times (these should be
/// consolidated) or if a [`Amount`] is considered invalid.
pub fn assert_unique_amounts(amounts: &[Option<UncheckedAmount>]) -> AssertResult<()> {
    let mut seen = HashSet::new();

    for amt in amounts.iter().filter_map(|a| a.as_ref()) {
        let key = hash(amt.currency.as_ref());

        if seen.contains(&key) {
            Err(format!(
                "amount currency {:?} of amount {amt:?} is invalid: {}",
                amt.currency,
                AccountBalanceError::CurrencyUsedMultipleTimes
            ))?;
        }

        seen.insert(key);

        amount(Some(amt)).map_err(|e| format!("amount {amt:?} is invalid: {e}"))?;
    }

    Ok(())
}

/// `account_balance_response` returns an error if the provided
/// [`UncheckedPartialBlockIdentifier`] is invalid, if the requestBlock
/// is not nil and not equal to the response block, or
/// if the same currency is present in multiple amounts.
pub fn account_balance_response(
    request_block: Option<&UncheckedPartialBlockIdentifier>,
    response: &UncheckedAccountBalanceResponse,
) -> AssertResult<()> {
    block_identifier(response.block_identifier.as_ref()).map_err(|e| {
        format!(
            "block identifier {:?} is invalid: {e}",
            response.block_identifier
        )
    })?;
    assert_unique_amounts(&response.balances)
        .map_err(|e| format!("balance amounts {:?} are invalid: {e}", response.balances))?;

    if request_block.is_none() {
        return Ok(());
    }
    let request_block = request_block.unwrap();
    let block_identifier = response.block_identifier.as_ref().unwrap();

    if matches!(request_block.hash.as_ref(), Some(i) if i != &block_identifier.hash) {
        Err(format!(
            "requested block hash {}, but got {}: {}",
            request_block.hash.as_ref().unwrap(),
            block_identifier.hash,
            AccountBalanceError::ReturnedBlockHashMismatch,
        ))?
    } else if matches!(request_block.index, Some(i) if i != block_identifier.index) {
        Err(format!(
            "requested block index {} but got {}: {}",
            request_block.index.unwrap(),
            block_identifier.index,
            AccountBalanceError::ReturnedBlockIndexMismatch,
        ))?
    } else {
        Ok(())
    }
}

/// `account_coins` returns an error if the provided
/// [`AccountCoinsResponse`] is invalid.
pub fn account_coins(response: &UncheckedAccountCoinsResponse) -> AssertResult<()> {
    block_identifier(response.block_identifier.as_ref()).map_err(|e| {
        format!(
            "block identifier {:?} is invalid: {e}",
            response.block_identifier
        )
    })?;
    coins(&response.coins).map_err(|e| format!("coins {:?} are invalid: {e}", response.coins))?;
    Ok(())
}
