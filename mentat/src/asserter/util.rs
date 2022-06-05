use std::fmt::Debug;

use indexmap::IndexSet;
use serde::Serialize;
use serde_json;
use sha2::{Digest, Sha256};

use crate::models::Sortable;

pub(crate) fn string_array(name: &str, values: &[String]) -> Result<(), String> {
    if values.is_empty() {
        return Err(format!("no {name} found"));
    }

    let mut parsed = IndexSet::new();
    for value in values {
        if value.is_empty() {
            return Err(format!("{name} has an empty string"));
        }

        if parsed.contains(value) {
            return Err(format!("{name} contains a duplicate {value}"));
        }

        parsed.insert(value);
    }

    Ok(())
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
