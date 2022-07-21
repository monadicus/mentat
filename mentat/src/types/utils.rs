//! TEMP DOC STRING

use std::{error::Error, fmt::Debug, str::FromStr};

use num_bigint_dig::*;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    AccountIdentifier,
    Amount,
    BlockIdentifier,
    Currency,
    PartialBlockIdentifier,
    Sortable,
};

/// `hash_bytes` returns a hex-encoded sha256 hash of the provided
/// byte slice.
pub(crate) fn hash_bytes(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();

    format!("{hash:x}")
}

/// 'hash' returns a deterministic hash for any interface.
/// This works because Golang's JSON marshaler sorts all map keys, recursively.
/// Source: https://golang.org/pkg/encoding/json/#Marshal
/// Inspiration:
/// https://github.com/onsi/gomega/blob/c0be49994280db30b6b68390f67126d773bc5558/matchers/match_json_matcher.go#L16
///
/// It is important to note that any interface that is a slice
/// or contains slices will not be equal if the slice ordering is
/// different.
pub(crate) fn hash<T>(hashable: &T) -> String
where
    T: Debug + Serialize + Sortable,
{
    let sorted = hashable.sort();

    let json = match serde_json::to_string(&sorted) {
        Ok(json) => json,
        Err(e) => panic!("{e}: unable to jsonify {hashable:?}"),
    };

    hash_bytes(json)
}

/// 'construct_partialblock_identifier' constructs a *PartialBlockIdentifier
/// from a *BlockIdentifier.
///
/// It is useful to have this helper when making block requests
/// with the fetcher.
pub(crate) fn construct_partialblock_identifier(block: &BlockIdentifier) -> PartialBlockIdentifier {
    PartialBlockIdentifier {
        index: Some(block.index),
        hash: Some(block.hash.clone()),
    }
}

/// `amount_value` returns a [`BigInt`] representation of an
/// Amount.Value or an error.
pub(crate) fn amount_value(amount: &Amount) -> Result<BigInt, Box<dyn Error>> {
    Ok(BigInt::from_str(&amount.value)?)
}

/// `account_string` returns a human-readable representation of a
/// [`AccountIdentifier`].
pub(crate) fn account_string(account: &AccountIdentifier) -> String {
    let sub_account = if account.sub_account.is_none() {
        return account.address.clone();
    } else {
        account.sub_account.as_ref().unwrap()
    };

    if sub_account.metadata.is_empty() {
        return format!("{}:{}", account.address, sub_account.address);
    }

    format!(
        "{}:{}:{:?}",
        account.address, sub_account.address, sub_account.metadata
    )
}

/// `currency_string` returns a human-readable representation
/// of a *Currency.
pub(crate) fn currency_string(currency: &Currency) -> String {
    if currency.metadata.is_empty() {
        return format!("{}:{}", currency.symbol, currency.decimals);
    }
    return format!(
        "{}:{}:{:?}",
        currency.symbol, currency.decimals, currency.metadata
    );
}

/// `big_int` returns a *big.Int representation of a value.
pub(crate) fn big_int(value: &str) -> Result<BigInt, Box<dyn Error>> {
    let parsed_val = BigInt::from_str(value)?;
    Ok(parsed_val)
}

/// `add_values` adds string amounts using
/// big.Int.
pub(crate) fn add_values(a: &str, b: &str) -> Result<String, Box<dyn Error>> {
    let a_val = BigInt::from_str(a)?;
    let b_val = BigInt::from_str(b)?;
    let new_val = a_val + b_val;
    Ok(new_val.to_string())
}

/// `subtract_values` subtracts a-b using
/// big.Int.
pub(crate) fn sub_values(a: &str, b: &str) -> Result<String, Box<dyn Error>> {
    let a_val = BigInt::from_str(a)?;
    let b_val = BigInt::from_str(b)?;
    let new_val = a_val - b_val;
    Ok(new_val.to_string())
}

/// `multiply_values` multiplies a*b using
/// big.Int.
pub(crate) fn multiply_values(a: &str, b: &str) -> Result<String, Box<dyn Error>> {
    let a_val = BigInt::from_str(a)?;
    let b_val = BigInt::from_str(b)?;
    let new_val = a_val * b_val;
    Ok(new_val.to_string())
}

/// `divide_values` divides a/b using
/// big.Int.
pub(crate) fn divide_values(a: &str, b: &str) -> Result<String, Box<dyn Error>> {
    let a_val = BigInt::from_str(a)?;
    let b_val = BigInt::from_str(b)?;
    let new_val = a_val % b_val;
    Ok(new_val.to_string())
}

/// `negate_value` flips the sign of a value.
pub(crate) fn negative_value(val: &str) -> Result<String, Box<dyn Error>> {
    let existing = big_int(val)?;
    Ok((-existing).to_string())
}

/// `extract_amount` returns the Amount from a slice of Balance
/// pertaining to an AccountAndCurrency.
pub(crate) fn extract_amount(balances: &[Option<Amount>], currency: &Currency) -> Amount {
    balances
        .iter()
        .find(|amt| {
            if amt.is_some() && amt.unwrap().currency.is_some() {
                hash(&amt.unwrap().currency.unwrap()) == hash(currency)
            } else {
                false
            }
        })
        .cloned()
        .flatten()
        .unwrap_or(Amount {
            value: "0".to_string(),
            currency: Some(currency.clone()),
            ..Default::default()
        })
}
