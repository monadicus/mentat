//! Validates that coin data is correct.

use super::*;

/// `coin` returns an error if the provided [`Coin`] is invalid.
pub fn coin(coin: Option<&UncheckedCoin>) -> AssertResult<()> {
    let coin = coin.ok_or(CoinError::IsNil)?;
    coin_identifier(coin.coin_identifier.as_ref())
        .map_err(|e| format!("coin identifier {:?} is invalid: {e}", coin.coin_identifier))?;
    amount(coin.amount.as_ref())
        .map_err(|e| format!("coin amount {:?} is invalid: {e}", coin.amount))?;
    Ok(())
}

/// `coins` returns an error if the provided
/// [`Coin`] is invalid. If there are any
/// duplicate identifiers, this function
/// will also return an error.
pub fn coins(coins: &[Option<UncheckedCoin>]) -> AssertResult<()> {
    let mut ids = IndexSet::new();
    for c in coins {
        coin(c.as_ref()).map_err(|e| format!("coin {c:?} is invalid: {e}"))?;
        let c = c.as_ref().unwrap();
        let c_ident = c.coin_identifier.as_ref().unwrap();
        if ids.contains(&c_ident.identifier) {
            Err(format!("coin {c:?} is invalid: {}", CoinError::Duplicate))?;
        }

        ids.insert(&c_ident.identifier);
    }

    Ok(())
}

/// [`coin_identifier`] returns an error if the provided [`CoinIdentifier`]
/// is invalid.
pub fn coin_identifier(coin_identifier: Option<&CoinIdentifier>) -> AssertResult<()> {
    let coin_identifier = coin_identifier.ok_or(CoinError::IdentifierIsNil)?;
    if coin_identifier.identifier.is_empty() {
        Err(CoinError::IdentifierNotSet)?
    } else {
        Ok(())
    }
}

/// `coin_change` returns an error if the provided [`CoinChange`]
/// is invalid.
pub fn coin_change(change: Option<&UncheckedCoinChange>) -> AssertResult<()> {
    let change = change.ok_or(CoinError::ChangeIsNil)?;

    coin_identifier(change.coin_identifier.as_ref()).map_err(|e| {
        format!(
            "coin identifier {:?} is invalid: {e}",
            change.coin_identifier
        )
    })?;
    coin_action(&change.coin_action)
        .map_err(|e| format!("coin action {:?} is invalid: {e}", change.coin_action))?;
    Ok(())
}

/// coin_action returns an error if the provided [`CoinAction`]
/// is invalid.
pub fn coin_action(act: &UncheckedCoinAction) -> AssertResult<()> {
    if !act.valid() {
        Err(AsserterError::from(format!(
            "failed to validate coin action {act}: {}",
            CoinError::ActionInvalid,
        )))
    } else {
        Ok(())
    }
}
