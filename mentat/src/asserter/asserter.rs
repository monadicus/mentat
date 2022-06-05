use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use super::{
    block::block_identifier,
    network::{operation_statuses, operation_types, network_identifier},
    server::supported_networks,
};
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

/// Validations is used to define stricter validations
/// on the transaction. Fore more details please refer to
/// https://github.com/coinbase/rosetta-sdk-go/tree/master/asserter#readme
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

/// For response assertion.
#[derive(Debug)]
pub(crate) struct ResponseAsserter {
    network: NetworkIdentifier,
    operation_types: Vec<String>,
    operation_status_map: IndexMap<String, bool>,
    error_type_map: IndexMap<i32, String>,
    genesis_block: BlockIdentifier,
    timestamp_start_index: i64,
    validations: Validations,
}

impl ResponseAsserter {
    /// ClientConfiguration returns all variables currently set in an Asserter.
    /// This function will error if it is called on an uninitialized asserter.
    pub(crate) fn client_configuration(&self) -> Result<Configuration, String> {
        // TODO if self nil

        let mut allowed_operation_statuses = Vec::new();
        for (status, successful) in self.operation_status_map.iter() {
            allowed_operation_statuses.push(OperationStatus {
                status: status.clone(),
                successful: *successful,
            });
        }

        Ok(Configuration {
            network_identifier: self.network.clone(),
            genesis_block_identifier: self.genesis_block.clone(),
            allowed_operation_types: self.operation_types.clone(),
            allowed_operation_statuses,
            allowed_errors: todo!(),
            allowed_timestamp_start_index: self.timestamp_start_index,
        })
    }

    /// NewClientWithOptions constructs a new Asserter using the provided
    /// arguments instead of using a NetworkStatusResponse and a
    /// NetworkOptionsResponse.
    pub(crate) fn new_client_with_options(
        network: NetworkIdentifier,
        genesis_block: BlockIdentifier,
        operation_types_: Vec<String>,
        operation_stats: Vec<OperationStatus>,
        errors: Vec<String>,
        timestamp_start_index: i64,
        validations: Validations,
    ) -> Result<Self, String> {
        network_identifier(&network)?;
        block_identifier(&genesis_block)?;
        operation_statuses(&operation_stats)?;
        operation_types(&operation_types_)?;

        // TimestampStartIndex defaults to genesisIndex + 1 (this
        // avoid breaking existing clients using < v1.4.6).
        let parsed_timestamp_start_index = genesis_block.index + 1;
        // TODO if timestampindex nil set parsedtimestampindex = timestampstartindex

        if timestamp_start_index < 0 {
            return Err(format!("{}: {timestamp_start_index}", todo!()));
        }

        let operation_status_map = operation_stats
            .iter()
            .map(|status| (status.status.clone(), status.successful))
            .collect();

        let error_type_map = todo!();

        Ok(Self {
            network,
            operation_types: operation_types_,
            genesis_block,
            timestamp_start_index: parsed_timestamp_start_index as i64,
            validations,
            operation_status_map,
            error_type_map,
        })
    }
}

/// For response assertion.
#[derive(Debug)]
pub(crate) struct RequestAsserter {
    operation_types: Vec<String>,
    historical_balance_lookup: bool,
    supported_networks: Vec<NetworkIdentifier>,
    call_methods: IndexSet<String>,
    mempool_coins: bool,
    validations: Validations,
}

impl RequestAsserter {
    pub(crate) fn new_server(
        supported_operation_types: Vec<String>,
        historical_balance_lookup: bool,
        supp_networks: Vec<NetworkIdentifier>,
        call_methods: Vec<String>,
        mempool_coins: bool,
        validation_file_path: String,
    ) -> Result<Self, String> {
        operation_types(&supported_operation_types)?;
        supported_networks(&supp_networks)?;

        let validations = Validations::get_calidation_config(validation_file_path)?;
        let mut call_map: IndexSet<String> = IndexSet::new();
        for method in call_methods {
            if method.is_empty() {
                return Err(todo!());
            }

            if call_map.contains(&method) {
                return Err(format!("{}: {method}", todo!()));
            }

            call_map.insert(method);
        }

        Ok(Self {
            operation_types: supported_operation_types,
            historical_balance_lookup,
            supported_networks: supp_networks,
            call_methods: call_map,
            mempool_coins,
            validations,
        })
    }
}

/// Asserter contains all logic to perform static
/// validation on Rosetta Server responses.
#[derive(Debug)]
pub(crate) enum Asserter {
    Response(ResponseAsserter),
    Request(RequestAsserter),
}

impl Asserter {
    /// NewServer constructs a new Asserter for use in the
    /// server package.
    pub(crate) fn new_server(
        supported_operation_types: Vec<String>,
        historical_balance_lookup: bool,
        supp_networks: Vec<NetworkIdentifier>,
        call_methods: Vec<String>,
        mempool_coins: bool,
        validation_file_path: String,
    ) -> Result<Self, String> {
        Ok(Asserter::Request(RequestAsserter::new_server(
            supported_operation_types,
            historical_balance_lookup,
            supp_networks,
            call_methods,
            mempool_coins,
            validation_file_path,
        )?))
    }

	/// NewClientWithResponses constructs a new Asserter
	/// from a NetworkStatusResponse and
	/// NetworkOptionsResponse.
    pub(crate) fn new_client_with_responses(
        network: NetworkIdentifier,
        status: NetworkStatusResponse,
        options: NetworkOptionsResponse,
        validation_file_path: String,
    ) -> Result<Self, String> {
		network_identifier(&network)?;
		
        todo!()
    }

    pub(crate) fn new_with_file(file_path: String) -> Result<Self, String> {
        todo!()
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
