// TODO move this file to types module when it exists

use std::fmt::Debug;

use serde::Serialize;
use serde_json;
use sha2::{Sha256, Digest};

use crate::models::Sortable;

pub(crate) fn hash_bytes(data: String) -> String {
	let mut hasher = Sha256::new();
	hasher.update(data.as_bytes());
	let hash = hasher.finalize();

	format!("{hash:x}")
}

pub(crate) fn hash<T: Debug + Serialize + Sortable>(hashable: &T) -> String {
	let sorted = hashable.sort();

	let json = match serde_json::to_string(&sorted) {
		Ok(json) => json,
		Err(e) => panic!("{e}: unable to jsonify {hashable:?}"),	
	}; 

	hash_bytes(json)
}