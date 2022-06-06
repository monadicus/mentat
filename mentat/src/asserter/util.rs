use std::fmt::Debug;

use indexmap::IndexSet;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{block::account_identifier, errors::AssertResult};
use crate::{identifiers::AccountIdentifier, models::Sortable};

/// string_array ensures all strings in an array
/// are non-empty strings and not duplicates.
pub(crate) fn string_array(name: &str, values: &[String]) -> AssertResult<()> {
    if values.is_empty() {
        return Err(format!("no {name} found").into());
    }

    let mut parsed = IndexSet::new();
    for value in values {
        if value.is_empty() {
            return Err(format!("{name} has an empty string").into());
        }

        if parsed.contains(value) {
            return Err(format!("{name} contains a duplicate {value}").into());
        }

        parsed.insert(value);
    }

    Ok(())
}

/// account_array ensures all [`AccountIdentifier`] in an array
/// are valid and not duplicates.
pub(crate) fn account_array(arr_name: &str, arr: &[AccountIdentifier]) -> AssertResult<()> {
    if arr.len() == 0 {
        return Err(format!("no {} found", arr_name).into());
    }

    let mut parsed = IndexSet::new();
    for s in arr {
        account_identifier(Some(s))
            .map_err(|e| format!("{arr_name} has an invalid account identifier"))?;
        if parsed.contains(hash(s)) {
            return Err(format!("{arr_name} contains a duplicate {s:?}").into());
        }

        parsed.insert(hash(s));
    }

    Ok(())
}

/// bytes_array_zero returns a boolean indicating if
/// all elements in an array are 0.
fn bytes_array_zero(arr: &[u8]) -> bool {
    arr.iter().all(|b| b == 0)
}

// TODO move this file to types module when it exists
pub(crate) fn hash_bytes(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();

    format!("{hash:x}")
}

// TODO move this file to types module when it exists
pub(crate) fn hash<T: Debug + Serialize + Sortable>(hashable: &T) -> String {
    let sorted = hashable.sort();

    let json = match serde_json::to_string(&sorted) {
        Ok(json) => json,
        Err(e) => panic!("{e}: unable to jsonify {hashable:?}"),
    };

    hash_bytes(json)
}
