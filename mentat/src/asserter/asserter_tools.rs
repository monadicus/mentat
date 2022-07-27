//! The asserter contains tools and methods to help validate the other types.
use std::path::PathBuf;

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use super::{
    block::block_identifier,
    errors::{AssertResult, AsserterError, BlockError, NetworkError, ServerError},
    network::{
        network_identifier, network_options_response, network_status_response, operation_statuses,
        operation_types,
    },
    server::supported_networks,
    BlockIdentifier,
    MentatError,
    NetworkIdentifier,
    NetworkOptionsResponse,
    NetworkStatusResponse,
    OperationStatus,
    DATA_DIR,
};

/// A static string representing account type data.
pub(crate) const ACCOUNT: &str = "account";
// pub(crate) const UTXO: &str = "utxo";

/// The `AsserterOperation` data helps validate data.
#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(clippy::missing_docs_in_private_items)]
pub(crate) struct AsserterOperation {
    pub(crate) count: i64,
    pub(crate) should_balance: bool,
}

/// The `ValidationOperation` data helps validate data.
#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(clippy::missing_docs_in_private_items)]
pub(crate) struct ValidationOperation {
    pub(crate) name: String,
    pub(crate) operation: AsserterOperation,
}

/// Validations is used to define stricter validations
/// on the transaction. Fore more details please refer to
/// https://github.com/coinbase/rosetta-sdk-go/tree/master/asserter#readme
#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(clippy::missing_docs_in_private_items)]
#[serde(default)]
pub(crate) struct Validations {
    pub(crate) enabled: bool,
    pub(crate) related_ops_exists: bool,
    pub(crate) chain_type: String,
    pub(crate) payment: ValidationOperation,
    pub(crate) fee: ValidationOperation,
}

impl Validations {
    /// Creates a new `Validations` struct given a config file.
    pub(crate) fn get_validation_config(
        validation_file_path: Option<&PathBuf>,
    ) -> Result<Self, String> {
        if let Some(path) = validation_file_path {
            // TODO handle these unwraps
            let content = DATA_DIR.get_file(path).unwrap();
            let mut config: Self = serde_json::from_str(content.contents_utf8().unwrap()).unwrap();
            return Ok(config);
        }

        Ok(Self::default())
    }
}

/// For response assertion.
#[derive(Debug)]
#[allow(clippy::missing_docs_in_private_items)]
pub(crate) struct ResponseAsserter {
    pub(crate) network: NetworkIdentifier,
    pub(crate) operation_status_map: IndexMap<String, bool>,
    pub(crate) error_type_map: IndexMap<i32, MentatError>,
    pub(crate) genesis_block: BlockIdentifier,
    pub(crate) timestamp_start_index: i64,
}

/// For request assertion.
#[derive(Debug)]
#[allow(clippy::missing_docs_in_private_items)]
pub(crate) struct RequestAsserter {
    pub(crate) historical_balance_lookup: bool,
    pub(crate) supported_networks: Vec<NetworkIdentifier>,
    pub(crate) call_methods: IndexSet<String>,
    pub(crate) mempool_coins: bool,
}
/// Asserter contains all logic to perform static
/// validation on Rosetta Server responses.
#[derive(Debug)]
#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::large_enum_variant)]
pub struct Asserter {
    pub(crate) operation_types: Vec<String>,
    pub(crate) request: Option<RequestAsserter>,
    pub(crate) response: Option<ResponseAsserter>,
    pub(crate) validations: Validations,
}

impl Asserter {
    /// `new_server` constructs a new [`Asserter`] for use in the
    /// server package.
    pub(crate) fn new_server(
        supported_operation_types: Vec<String>,
        historical_balance_lookup: bool,
        supp_networks: Vec<NetworkIdentifier>,
        call_methods: Vec<String>,
        mempool_coins: bool,
        validation_file_path: Option<&PathBuf>,
    ) -> AssertResult<Self> {
        operation_types(&supported_operation_types)?;
        supported_networks(&supp_networks.iter().cloned().map(Some).collect::<Vec<_>>())?;

        let validations = Validations::get_validation_config(validation_file_path)?;
        let mut call_map: IndexSet<String> = IndexSet::new();
        for method in call_methods {
            if method.is_empty() {
                Err(ServerError::CallMethodEmpty)?
            } else if call_map.contains(&method) {
                Err(format!("{}: {method}", ServerError::CallMethodDuplicate))?
            } else {
                call_map.insert(method);
            }
        }

        Ok(Self {
            operation_types: supported_operation_types,
            request: Some(RequestAsserter {
                historical_balance_lookup,
                supported_networks: supp_networks,
                call_methods: call_map,
                mempool_coins,
            }),
            response: None,
            validations,
        })
    }

    /// `new_client_with_options` constructs a new [`Asserter`] using the
    /// provided arguments instead of using a NetworkStatusResponse and a
    /// NetworkOptionsResponse.
    fn new_client_with_options(
        network: Option<NetworkIdentifier>,
        genesis_block: Option<BlockIdentifier>,
        operation_types_: Vec<String>,
        operation_stats: Vec<Option<OperationStatus>>,
        errors: Vec<Option<MentatError>>,
        timestamp_start_index: Option<i64>,
        validations: Validations,
    ) -> AssertResult<Self> {
        network_identifier(network.as_ref())?;
        block_identifier(genesis_block.as_ref())?;
        let genesis_block = genesis_block.unwrap();
        operation_statuses(&operation_stats)?;
        operation_types(&operation_types_)?;

        // TimestampStartIndex defaults to genesisIndex + 1 (this
        // avoid breaking existing clients using < v1.4.6).
        // safe to unwrap.
        let mut parsed_timestamp_start_index = genesis_block.index + 1;
        if let Some(tsi) = timestamp_start_index {
            if tsi < 0 {
                Err(AsserterError::from(format!(
                    "{}: {tsi}",
                    NetworkError::TimestampStartIndexInvalid
                )))?;
            }

            parsed_timestamp_start_index = tsi;
        }

        // TODO these unwraps are not safe see operation_statuses fn.
        let operation_status_map = operation_stats
            .into_iter()
            .map(|status| (status.clone().unwrap().status, status.unwrap().successful))
            .collect();

        let error_type_map = errors
            .into_iter()
            .map(|err| {
                // Safe to unwrap
                let err = err.unwrap();
                (err.code, err)
            })
            .collect();

        Ok(Self {
            operation_types: operation_types_,
            request: None,
            response: Some(ResponseAsserter {
                // safe to unwrap.
                network: network.unwrap(),
                genesis_block,
                timestamp_start_index: parsed_timestamp_start_index as i64,
                operation_status_map,
                error_type_map,
            }),
            validations,
        })
    }

    /// NewClientWithResponses constructs a new Asserter
    /// from a NetworkStatusResponse and
    /// NetworkOptionsResponse.
    pub(crate) fn new_client_with_responses(
        network: Option<NetworkIdentifier>,
        status: Option<NullableNetworkStatusResponse>,
        options: Option<NullableNetworkOptionsResponse>,
        validation_file_path: Option<&PathBuf>,
    ) -> AssertResult<Self> {
        network_identifier(network.as_ref())?;
        network_status_response(status.as_ref())?;
        network_options_response(options.as_ref())?;
        // safe to unwrap.
        let allow = options.unwrap().allow.unwrap();

        let validations = Validations::get_validation_config(validation_file_path)?;

        Self::new_client_with_options(
            network,
            // this is safe to unwrap.
            status.unwrap().genesis_block_identifier,
            allow.operation_types,
            allow.operation_statuses,
            allow.errors,
            allow.timestamp_start_index,
            validations,
        )
    }

    /// ClientConfiguration returns all variables currently set in an Asserter.
    /// This function will error if it is called on an uninitialized asserter.
    pub(crate) fn client_configuration(&self) -> AssertResult<Configuration> {
        let asserter = self
            .response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;

        let mut allowed_operation_statuses = Vec::new();
        for (status, successful) in asserter.operation_status_map.iter() {
            allowed_operation_statuses.push(OperationStatus {
                status: status.clone(),
                successful: *successful,
            });
        }

        Ok(Configuration {
            network_identifier: Some(asserter.network.clone()),
            genesis_block_identifier: Some(asserter.genesis_block.clone()),
            allowed_operation_types: self.operation_types.clone(),
            allowed_operation_statuses: allowed_operation_statuses.into_iter().map(Some).collect(),
            allowed_errors: Vec::new(),
            allowed_timestamp_start_index: Some(asserter.timestamp_start_index),
        })
    }

    /// Says whether a given operation was successful or not.
    pub(crate) fn operation_successful(
        &self,
        operation: Option<&crate::types::NullableOperation>,
    ) -> AssertResult<bool> {
        let asserter = self
            .response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;

        let operation = operation.unwrap();
        if operation.status.is_none() || operation.status.as_ref().unwrap().is_empty() {
            Err(BlockError::OperationStatusMissing)?;
        }

        if let Some(val) = asserter
            .operation_status_map
            .get(operation.status.as_ref().unwrap())
        {
            Ok(*val)
        } else {
            Err(AsserterError::from(format!(
                "{} not found",
                operation.status.as_ref().unwrap()
            )))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::missing_docs_in_private_items)]
pub(crate) struct Configuration {
    pub(crate) network_identifier: Option<NetworkIdentifier>,
    pub(crate) genesis_block_identifier: Option<BlockIdentifier>,
    pub(crate) allowed_operation_types: Vec<String>,
    pub(crate) allowed_operation_statuses: Vec<Option<OperationStatus>>,
    pub(crate) allowed_errors: Vec<Option<MentatError>>,
    pub(crate) allowed_timestamp_start_index: Option<i64>,
}

impl Configuration {
    /// NewClientWithFile constructs a new Asserter using a specification
    /// file instead of responses. This can be useful for running reliable
    /// systems that error when updates to the server (more error types,
    /// more operations, etc.) significantly change how to parse the chain.
    /// The filePath provided is parsed relative to the current directory.
    pub(crate) fn new_client_with_file(path: PathBuf) -> AssertResult<Asserter> {
        // TODO handle these unwraps
        let content = std::fs::File::open(path).unwrap();
        let config: Self = serde_json::from_reader(content).unwrap();

        Asserter::new_client_with_options(
            config.network_identifier,
            config.genesis_block_identifier,
            config.allowed_operation_types,
            config.allowed_operation_statuses,
            config.allowed_errors,
            config.allowed_timestamp_start_index,
            Validations::default(),
        )
    }
}
