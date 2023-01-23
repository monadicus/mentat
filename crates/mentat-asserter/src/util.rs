//! Utility functions to make validations easier.

use super::*;

/// `string_array` ensures all strings in an array
/// are non-empty strings and not duplicates.
pub(crate) fn string_array(name: &str, values: &[String]) -> AssertResult<()> {
    if values.is_empty() {
        Err(format!(
            "string array {name} is empty: {}",
            UtilError::StringArrayEmpty
        ))?;
    }

    let mut parsed = IndexSet::new();
    for value in values {
        if value.is_empty() {
            Err(format!(
                "string array {name} has an empty string: {}",
                UtilError::StringArrayEmptyString
            ))?;
        }

        if !parsed.insert(value) {
            Err(format!(
                "string array {name} contains a duplicate {value}: {}",
                UtilError::StringArrayDuplicateString
            ))?;
        }
    }

    Ok(())
}

/// `account_array` ensures all [`AccountIdentifier`] in an array
/// are valid and not duplicates.
pub(crate) fn account_array(arr_name: &str, arr: &[Option<AccountIdentifier>]) -> AssertResult<()> {
    if arr.is_empty() {
        Err(format!(
            "account array {arr_name} is empty: {}",
            UtilError::AccountArrayEmpty
        ))?;
    }

    let mut parsed = IndexSet::new();
    for s in arr {
        account_identifier(s.as_ref()).map_err(|_e| {
            format!(
                "account array {arr_name} has an invalid account identifier: {}",
                UtilError::AccountArrayInvalidAccount
            )
        })?;
        let key = hash(s.as_ref());
        if parsed.contains(&key) {
            Err(format!(
                "account array {arr_name} contains a duplicate account identifier {s:?}: {}",
                UtilError::AccountArrayDuplicateAccount
            ))?;
        }

        parsed.insert(key);
    }

    Ok(())
}

/// `bytes_array_zero` returns a boolean indicating if
/// all elements in an array are 0.
pub fn bytes_array_zero(arr: &[u8]) -> bool {
    arr.iter().all(|b| b == &0)
}
