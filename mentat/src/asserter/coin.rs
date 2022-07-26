//! Validates that coin data is correct.

use indexmap::IndexSet;

use super::{
    amount, errors::AsserterError, AssertResult, CoinAction, CoinError, CoinIdentifier,
    NullableCoin, NullableCoinChange,
};

/// `coin` returns an error if the provided [`Coin`] is invalid.
pub fn coin(coin: Option<&NullableCoin>) -> AssertResult<()> {
    let coin = coin.ok_or(CoinError::IsNil)?;
    coin_identifier(coin.coin_identifier.as_ref())
        .map_err(|e| format!("{e}: coin identifier is invalid"))?;
    amount(coin.amount.as_ref()).map_err(|e| format!("{e}: coin amount is invalid"))?;
    Ok(())
}

/// `coins` returns an error if the provided
/// [`Coin`] is invalid. If there are any
/// duplicate identifiers, this function
/// will also return an error.
pub fn coins(coins: &[Option<NullableCoin>]) -> AssertResult<()> {
    let mut ids = IndexSet::new();
    for c in coins {
        coin(c.as_ref()).map_err(|err| format!("{err}: coin is invalid"))?;
        let c = c.as_ref().unwrap();
        let c_ident = c.coin_identifier.as_ref().unwrap();
        if ids.contains(&c_ident.identifier) {
            Err(format!("{}: {}", CoinError::Duplicate, c_ident.identifier))?;
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
pub fn coin_change(change: Option<&NullableCoinChange>) -> AssertResult<()> {
    let change = change.ok_or(CoinError::ChangeIsNil)?;

    coin_identifier(change.coin_identifier.as_ref())
        .map_err(|e| format!("{e}: coin identifier is invalid"))?;
    coin_action(&change.coin_action).map_err(|e| format!("{e}: coin action is invalid"))?;
    Ok(())
}

/// coin_action returns an error if the provided [`CoinAction`]
/// is invalid.
pub fn coin_action(act: &CoinAction) -> AssertResult<()> {
    if !act.valid() {
        Err(AsserterError::from(format!(
            "{}: {}",
            CoinError::ActionInvalid,
            act
        )))
    } else {
        Ok(())
    }
}
