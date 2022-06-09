use indexmap::IndexSet;

use super::{
    block::amount,
    errors::{AssertResult, CoinError},
};
use crate::{
    identifiers::CoinIdentifier,
    models::{Coin, CoinAction, CoinChange},
};

/// `coin` returns an error if the provided [`Coin`] is invalid.
pub(crate) fn coin(coin: &Coin) -> AssertResult<()> {
    // TODO coin == nil
    coin_identifier(&coin.coin_identifier).map_err(|e| format!("{e}: identifier is invalid"))?;
    amount(Some(&coin.amount)).map_err(|e| format!("{e}: coin amount invalid"))?;
    Ok(())
}

/// `coins` returns an error if the provided
/// [`Coin`] is invalid. If there are any
/// duplicate identifiers, this function
/// will also return an error.
pub(crate) fn coins(coins: &[Coin]) -> AssertResult<()> {
    // TODO if coins == nil
    let mut ids = IndexSet::new();
    for c in coins {
        coin(c).map_err(|err| format!("{err}: coin is invalid"))?;

        if ids.contains(&c.coin_identifier.identifier) {
            Err(format!(
                "{}: {}",
                CoinError::Duplicate,
                c.coin_identifier.identifier
            ))?;
        }

        ids.insert(&c.coin_identifier.identifier);
    }

    Ok(())
}

/// [`coin_identifier`] returns an error if the provided [`CoinIdentifier`]
/// is invalid.
pub(crate) fn coin_identifier(coin_identifier: &CoinIdentifier) -> AssertResult<()> {
    // TODO coin_identifier == nil
    if coin_identifier.identifier.is_empty() {
        Err(CoinError::IdentifierNotSet)?
    } else {
        Ok(())
    }
}

/// `coin_change` returns an error if the provided [`CoinChange`]
/// is invalid.
pub(crate) fn coin_change(change: Option<&CoinChange>) -> AssertResult<()> {
    let change = change.ok_or(CoinError::ChangeIsNil)?;

    coin_identifier(&change.coin_identifier)
        .map_err(|e| format!("{e}: coin identifier is invalid"))?;
    coin_action(&change.coin_action).map_err(|e| format!("{e}: coin action is invalid"))?;
    Ok(())
}

/// coin_action returns an error if the provided [`CoinAction`]
/// is invalid.
pub(crate) fn coin_action(_: &CoinAction) -> AssertResult<()> {
    todo!("impossible case");
    // match action {
    //     CoinAction::CoinCreated => Ok(()),
    //     CoinAction::CoinSpent => Ok(()),
    // }

    Ok(())
}
