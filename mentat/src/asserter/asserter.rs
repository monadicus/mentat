use std::error::Error;

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::{
    identifiers::{BlockIdentifier, NetworkIdentifier},
    misc::OperationStatus,
    responses::{NetworkOptionsResponse, NetworkStatusResponse},
};

static ACCOUNT: &'static str = "account";
static UTXO: &'static str = "utxo";

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
    chain_type: String,
    payment: ValidationOperation,
    Fee: ValidationOperation,
}

impl Validations {
    pub(crate) fn get_calidation_config(validation_file_path: String) -> Result<Self, String> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct Asserter {
    // For response assertion.
    network: NetworkIdentifier,
    operation_types: Vec<String>,
    operation_status_map: IndexMap<String, bool>,
    error_type_map: IndexMap<i32, String>,
    genesis_block: BlockIdentifier,
    timestamp_start_index: i64,

    // For request assertion
    historical_balance_lookup: bool,
    supported_networks: Vec<NetworkIdentifier>,
    call_methods: IndexSet<String>,
    mempool_coins: bool,
    validations: Validations,
}

impl Asserter {
    pub(crate) fn new_server(
        supported_operation_types: Vec<String>,
        historical_balance_lookup: bool,
        supported_networks: Vec<NetworkIdentifier>,
        call_methods: Vec<String>,
        mempool_coins: bool,
        validation_file_path: String,
    ) -> Result<Self, String> {
        todo!()
    }

    pub(crate) fn new_client_with_options(
        network: NetworkIdentifier,
        genesis_block_identifier: BlockIdentifier,
        operation_types: Vec<String>,
        operation_statuses: Vec<OperationStatus>,
        errors: Vec<String>,
        timestamp_start_index: i64,
        validation_config: Validations,
    ) -> Result<Self, String> {
        todo!()
    }

    pub(crate) fn new_client_with_responses(
        network: NetworkIdentifier,
        status: NetworkStatusResponse,
        options: NetworkOptionsResponse,
        validation_file_path: String,
    ) -> Result<Self, String> {
        todo!()
    }

    pub(crate) fn new_with_file(file_path: String) -> Result<Self, String> {
        todo!()
    }

    pub(crate) fn client_configuration(&self) -> Result<Configuration, String> {
        Ok(Configuration {
            network_identifier: self.network.clone(),
            genesis_block_identifier: self.genesis_block.clone(),
            allowed_operation_types: self.operation_types.clone(),
            allowed_operation_statuses: todo!(),
            allowed_errors: todo!(),
            allowed_timestamp_start_index: self.timestamp_start_index,
        })
    }

    pub(crate) fn operation_successful(&self, operation: &Operation) -> Result<bool, String> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Configuration {
    network_identifier: NetworkIdentifier,
    genesis_block_identifier: BlockIdentifier,
    allowed_operation_types: Vec<String>,
    allowed_operation_statuses: Vec<OperationStatus>,
    allowed_errors: Vec<String>,
    allowed_timestamp_start_index: i64,
}
