use std::error::Error;

use indexmap::{IndexMap, IndexSet};
use serde::{Serialize, Deserialize};

use crate::identifiers::{NetworkIdentifier, BlockIdentifier};


type ChainType = String;

static ACCOUNT: ChainType = "account";
static UTXO: ChainType = "utxo";

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Operation {
	count: usize,
	should_balance: bool, 
}


#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ValidationOperation {
	name: String,
	operation: Operation, 
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Validations {
	enabled: bool,
	related_ops_exists: bool,
	chain_type: ChainType,
	payment: ValidationOperation,
	Fee: ValidationOperation,
}

pub(crate) struct Asserter {
	// For response assertion.
	network: NetworkIdentifier,
	operation_types: Vec<String>,
	operation_status_map: IndexMap<String, bool>,
	error_type_map: IndexMap<i32, Box<dyn Error>>,
	genesis_block: BlockIdentifier,
	timestamp_start_index: i64,
	
	// For request assertion
	historical_balance_lookup: bool,
	supported_networks: Vec<NetworkIdentifier>,
	call_methods: IndexSet<String>,
	mempool_coins: bool,
	validations: Validations,
}