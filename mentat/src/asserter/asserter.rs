use std::path::Path;

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use super::{
    block::block_identifier,
    errors::{AssertResult, NetworkError, ServerError},
    network::{network_identifier, operation_statuses, operation_types},
    server::supported_networks,
};
use crate::{
    identifiers::{BlockIdentifier, NetworkIdentifier},
    misc::OperationStatus,
    responses::{NetworkOptionsResponse, NetworkStatusResponse},
};

pub(crate) const ACCOUNT: &str = "account";
pub(crate) const UTXO: &str = "utxo";

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Operation {
    pub(crate) count: i64,
    pub(crate) should_balance: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ValidationOperation {
    pub(crate) name: String,
    pub(crate) operation: Operation,
}

/// Validations is used to define stricter validations
/// on the transaction. Fore more details please refer to
/// https://github.com/coinbase/rosetta-sdk-go/tree/master/asserter#readme
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Validations {
    pub(crate) enabled: bool,
    pub(crate) related_ops_exists: bool,
    pub(crate) chain_type: String,
    pub(crate) payment: ValidationOperation,
    pub(crate) fee: ValidationOperation,
}

impl Validations {
    pub(crate) fn get_validation_config(validation_file_path: &Path) -> Result<Self, String> {
        todo!()
    }
}

/// For response assertion.
#[derive(Debug)]
pub(crate) struct ResponseAsserter {
    pub(crate) network: NetworkIdentifier,
    pub(crate) operation_types: Vec<String>,
    pub(crate) operation_status_map: IndexMap<String, bool>,
    pub(crate) error_type_map: IndexMap<i32, String>,
    pub(crate) genesis_block: BlockIdentifier,
    pub(crate) timestamp_start_index: i64,
    pub(crate) validations: Validations,
}

impl ResponseAsserter {
    /// ClientConfiguration returns all variables currently set in an Asserter.
    /// This function will error if it is called on an uninitialized asserter.
    pub(crate) fn client_configuration(&self) -> AssertResult<Configuration> {
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
    ) -> AssertResult<Self> {
        network_identifier(&network)?;
        block_identifier(&genesis_block)?;
        operation_statuses(&operation_stats)?;
        operation_types(&operation_types_)?;

        // TimestampStartIndex defaults to genesisIndex + 1 (this
        // avoid breaking existing clients using < v1.4.6).
        let parsed_timestamp_start_index = genesis_block.index + 1;
        // TODO if timestampindex nil set parsedtimestampindex = timestampstartindex

        if timestamp_start_index < 0 {
            return Err(format!(
                "{}: {timestamp_start_index}",
                NetworkError::TimestampStartIndexInvalid
            )
            .into());
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
    pub(crate) operation_types: Vec<String>,
    pub(crate) historical_balance_lookup: bool,
    pub(crate) supported_networks: Vec<NetworkIdentifier>,
    pub(crate) call_methods: IndexSet<String>,
    pub(crate) mempool_coins: bool,
    pub(crate) validations: Validations,
}

impl RequestAsserter {
    pub(crate) fn new_server(
        supported_operation_types: Vec<String>,
        historical_balance_lookup: bool,
        supp_networks: Vec<NetworkIdentifier>,
        call_methods: Vec<String>,
        mempool_coins: bool,
        validation_file_path: &Path,
    ) -> AssertResult<Self> {
        operation_types(&supported_operation_types)?;
        supported_networks(&supp_networks)?;

        let validations = Validations::get_validation_config(validation_file_path)?;
        let mut call_map: IndexSet<String> = IndexSet::new();
        for method in call_methods {
            if method.is_empty() {
                return Err(ServerError::CallMethodEmpty.into());
            } else if call_map.contains(&method) {
                Err(format!("{}: {method}", ServerError::CallMethodDuplicate))?;
            } else {
                call_map.insert(method);
            }
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
        validation_file_path: &Path,
    ) -> AssertResult<Self> {
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
        validation_file_path: &Path,
    ) -> AssertResult<Self> {
        network_identifier(&network)?;

        todo!()
    }

    pub(crate) fn new_with_file(file_path: String) -> AssertResult<Self> {
        todo!()
    }

    pub(crate) fn operation_successful(&self, operation: &Operation) -> AssertResult<bool> {
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
