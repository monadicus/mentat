//! Types module Util functions

use std::{fmt::Debug, str::FromStr};

use num_bigint_dig::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};

use super::{
    AccountIdentifier,
    Amount,
    BlockIdentifier,
    NullableAmount,
    NullableCurrency,
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
pub fn hash<T>(hashable: Option<&T>) -> String
where
    T: Debug + Serialize + Sortable,
{
    if let Some(hashable) = hashable {
        let sorted = hashable.sort();

        let json = match serde_json::to_string(&sorted) {
            Ok(json) => json,
            // TODO this should log?
            Err(e) => panic!("{e}: unable to jsonify {hashable:?}"),
        };

        hash_bytes(json)
    } else {
        String::new()
    }
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
pub fn amount_value(amount: Option<&Amount>) -> Result<BigInt, String> {
    let amount = amount.ok_or("amount value cannot be nil")?;
    BigInt::from_str(&amount.value).map_err(|_| format!("{} is not an integer", amount.value))
}

/// `account_string` returns a human-readable representation of a
/// [`AccountIdentifier`].
pub(crate) fn account_string(account: &AccountIdentifier) -> String {
    let sub_account = if let Some(sub_account) = account.sub_account.as_ref() {
        sub_account
    } else {
        return account.address.clone();
    };

    if sub_account.metadata.is_empty() {
        format!("{}:{}", account.address, sub_account.address)
    } else {
        format!(
            "{}:{}:{:?}",
            account.address, sub_account.address, sub_account.metadata
        )
    }
}

/// `currency_string` returns a human-readable representation
/// of a *Currency.
pub(crate) fn currency_string(currency: &NullableCurrency) -> String {
    if currency.metadata.is_empty() {
        format!("{}:{}", currency.symbol, currency.decimals)
    } else {
        format!(
            "{}:{}:{:?}",
            currency.symbol, currency.decimals, currency.metadata
        )
    }
}

/// `big_int` returns a *big.Int representation of a value.
pub(crate) fn big_int(value: &str) -> Result<BigInt, String> {
    let parsed_val = BigInt::from_str(value).map_err(|_| format!("{value} is not an integer"))?;
    Ok(parsed_val)
}

/// `add_values` adds string amounts using
/// big.Int.
pub fn add_values(a: &str, b: &str) -> Result<String, String> {
    let a_val = BigInt::from_str(a).map_err(|_| format!("{a} is not an integer"))?;
    let b_val = BigInt::from_str(b).map_err(|_| format!("{b} is not an integer"))?;
    let new_val = a_val + b_val;
    Ok(new_val.to_string())
}

/// `subtract_values` subtracts a-b using
/// big.Int.
pub(crate) fn sub_values(a: &str, b: &str) -> Result<String, String> {
    let a_val = BigInt::from_str(a).map_err(|_| format!("{a} is not an integer"))?;
    let b_val = BigInt::from_str(b).map_err(|_| format!("{b} is not an integer"))?;
    let new_val = a_val - b_val;
    Ok(new_val.to_string())
}

/// `multiply_values` multiplies a*b using
/// big.Int.
pub(crate) fn multiply_values(a: &str, b: &str) -> Result<String, String> {
    let a_val = BigInt::from_str(a).map_err(|_| format!("{a} is not an integer"))?;
    let b_val = BigInt::from_str(b).map_err(|_| format!("{b} is not an integer"))?;
    let new_val = a_val * b_val;
    Ok(new_val.to_string())
}

/// `divide_values` divides a/b using
/// big.Int.
pub(crate) fn divide_values(a: &str, b: &str) -> Result<String, String> {
    let a_val = BigInt::from_str(a).map_err(|_| format!("{a} is not an integer"))?;
    let b_val = BigInt::from_str(b).map_err(|_| format!("{b} is not an integer"))?;
    let new_val = a_val % b_val;
    Ok(new_val.to_string())
}

/// `negate_value` flips the sign of a value.
pub fn negate_value(val: &str) -> Result<String, String> {
    let existing = big_int(val)?;
    Ok((-existing).to_string())
}

/// `extract_amount` returns the Amount from a slice of Balance
/// pertaining to an AccountAndCurrency.
pub(crate) fn extract_amount(
    balances: &[Option<NullableAmount>],
    currency: Option<&NullableCurrency>,
) -> NullableAmount {
    balances
        .iter()
        .find(|amt| {
            if amt.is_some() && amt.as_ref().unwrap().currency.is_some() {
                hash(amt.as_ref().unwrap().currency.as_ref()) == hash(currency)
            } else {
                false
            }
        })
        .cloned()
        .flatten()
        .unwrap_or(NullableAmount {
            value: "0".to_string(),
            currency: currency.cloned(),
            ..Default::default()
        })
}

/// custom deserializer that replaces `null` values with default ones
pub(crate) fn null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// custom deserializer that makes string values all uppercase
pub(crate) fn string_as_uppercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|s| s.to_uppercase())
}

/// custom serializer that makes string values all uppercase
pub(crate) fn string_to_uppercase<S>(str: &str, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&str.to_uppercase())
}

/// For hex look ups when encoding bytes to hex
const HEXTABLE: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Encodes a slice of bytes to a hex string.
pub fn encode_to_hex_string(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        hex.push(HEXTABLE[(byte >> 4) as usize]);
        hex.push(HEXTABLE[(byte & 0x0f) as usize]);
    }

    hex
}

/// Converts from a hex byte to a non hex byte.
fn from_hex_char(byte: u8) -> (u8, bool) {
    if (b'0'..=b'9').contains(&byte) {
        (byte - b'0', true)
    } else if (b'a'..=b'f').contains(&byte) {
        (byte - b'a' + 10, true)
    } else if (b'A'..=b'F').contains(&byte) {
        (byte - b'A' + 10, true)
    } else {
        (0, false)
    }
}

/// Encodes a slice of bytes to a hex string.
pub fn decode_from_hex_string(hex: String) -> Result<Vec<u8>, u8> {
    let bytes = hex.as_bytes();
    let mut converted_bytes = Vec::with_capacity(bytes.len() / 2);

    for j in (1..bytes.len()).step_by(2) {
        let (a, success) = from_hex_char(bytes[j - 1]);
        if !success {
            return Err(bytes[j - 1]);
        }

        let (b, success) = from_hex_char(bytes[j]);
        if !success {
            return Err(bytes[j]);
        }

        converted_bytes.push((a << 4) | b);
    }
    Ok(converted_bytes)
}

/// For serializing a slice of bytes to a hex string.
pub(crate) fn bytes_to_hex_str<S>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex = encode_to_hex_string(bytes);
    s.serialize_str(&hex)
}

/// custom deserializer that replaces `null` with an empty vec of bytes
pub(crate) fn null_default_bytes_to_hex<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    if let Some(hex_str) = opt {
        decode_from_hex_string(hex_str).map_err(serde::de::Error::custom)
    } else {
        Ok(Vec::new())
    }
}
