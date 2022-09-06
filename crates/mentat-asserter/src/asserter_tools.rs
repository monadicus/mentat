//! The asserter contains tools and methods to help validate the other types.
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use super::*;

/// A static string representing account type data.
pub(crate) const ACCOUNT: &str = "account";
// pub(crate) const UTXO: &str = "utxo";

/// The `AsserterOperation` data helps validate data.
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct AsserterOperation {
    pub count: isize,
    pub should_balance: bool,
}

/// The `ValidationOperation` data helps validate data.
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct ValidationOperation {
    pub name: String,
    pub operation: AsserterOperation,
}

/// Validations is used to define stricter validations
/// on the transaction. Fore more details please refer to
/// https://github.com/coinbase/rosetta-sdk-go/tree/master/asserter#readme
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
#[serde(default)]
pub struct Validations {
    pub enabled: bool,
    pub related_ops_exists: bool,
    pub chain_type: String,
    pub payment: ValidationOperation,
    pub fee: ValidationOperation,
}

impl Validations {
    /// Creates a new `Validations` struct given a config file.
    pub(crate) fn get_validation_config(
        validation_file_path: Option<&PathBuf>,
    ) -> Result<Self, String> {
        if let Some(path) = validation_file_path {
            let content = DATA_DIR
                .get_file(path)
                .ok_or_else(|| format!("failed to read file {}", path.display()))?;
            let config: Self = serde_json::from_str(content.contents_utf8().unwrap())
                .map_err(|e| format!("failed to deserialize contents of file {}: {e}", path.display()))?;
            return Ok(config);
        }

        Ok(Self::default())
    }
}

/// For response assertion.
#[derive(Debug, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub(crate) struct ResponseAsserter {
    pub(crate) network: NetworkIdentifier,
    pub(crate) operation_status_map: IndexMap<String, bool>,
    pub(crate) error_type_map: IndexMap<isize, UncheckedMentatError>,
    pub(crate) genesis_block: UncheckedBlockIdentifier,
    pub(crate) timestamp_start_index: usize,
}

/// For request assertion.
#[derive(Debug, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub(crate) struct RequestAsserter {
    pub(crate) historical_balance_lookup: bool,
    pub(crate) supported_networks: Vec<NetworkIdentifier>,
    pub(crate) call_methods: IndexSet<String>,
    pub(crate) mempool_coins: bool,
}
/// Asserter contains all logic to perform static
/// validation on Rosetta Server responses.
#[derive(Debug, Clone, Default)]
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
    pub fn new_server(
        supported_operation_types: Vec<String>,
        historical_balance_lookup: bool,
        supp_networks: Vec<NetworkIdentifier>,
        call_methods: Vec<String>,
        mempool_coins: bool,
        validation_file_path: Option<&PathBuf>,
    ) -> AssertResult<Self> {
        operation_types(&supported_operation_types).map_err(|e| {
            format!(
                "operation types {:?} are invalid: {e}",
                supported_operation_types
            )
        })?;
        supported_networks(&supp_networks.iter().cloned().map(Some).collect::<Vec<_>>())
            .map_err(|e| format!("network identifiers {:?} are invalid: {e}", supp_networks))?;

        let validations = Validations::get_validation_config(validation_file_path)
            .map_err(|e| format!("config {:?} is invalid: {e}", validation_file_path))?;
        let mut call_map: IndexSet<String> = IndexSet::new();
        for method in call_methods {
            if method.is_empty() {
                Err(ServerError::CallMethodEmpty)?
            } else if call_map.contains(&method) {
                Err(format!(
                    "failed to call method {method}: {}",
                    ServerError::CallMethodDuplicate
                ))?
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
    pub fn new_client_with_options(
        network: Option<NetworkIdentifier>,
        genesis_block: Option<UncheckedBlockIdentifier>,
        operation_types_: Vec<String>,
        operation_stats: Vec<Option<OperationStatus>>,
        errors: Vec<Option<UncheckedMentatError>>,
        timestamp_start_index: Option<isize>,
        validations: Validations,
    ) -> AssertResult<Self> {
        network_identifier(network.as_ref())
            .map_err(|e| format!("network identifier {network:?} is invalid: {e}",))?;
        block_identifier(genesis_block.as_ref())
            .map_err(|e| format!("genesis block identifier {genesis_block:?} is invalid: {e}"))?;
        let genesis_block = genesis_block.unwrap();
        operation_statuses(&operation_stats)
            .map_err(|e| "operation statuses {operation_stats:?} are invalid: {e}")?;
        operation_types(&operation_types_)
            .map_err(|e| format!("operation types {operation_types_:?} are invalid: {e}"))?;

        // TimestampStartIndex defaults to genesisIndex + 1 (this
        // avoid breaking existing clients using < v1.4.6).
        // safe to unwrap.
        let unparsed_timestamp_start_index =
            timestamp_start_index.unwrap_or(genesis_block.index + 1);
        let parsed_timestamp_start_index =
            unparsed_timestamp_start_index.try_into().map_err(|_| {
                format!(
                    "failed to validate index {}: {}",
                    timestamp_start_index.unwrap(),
                    NetworkError::TimestampStartIndexInvalid,
                )
            })?;

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
                timestamp_start_index: parsed_timestamp_start_index,
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
        status: Option<UncheckedNetworkStatusResponse>,
        options: Option<UncheckedNetworkOptionsResponse>,
        validation_file_path: Option<&PathBuf>,
    ) -> AssertResult<Self> {
        network_identifier(network.as_ref())
            .map_err(|e| format!("network identifier {network:?} is invalid: {e}"))?;
        network_status_response(status.as_ref())
            .map_err(|e| format!("network status response {status:?} is invalid: {e}"))?;
        network_options_response(options.as_ref())
            .map_err(|e| format!("network options response {options:?} is invalid: {e}"))?;
        // safe to unwrap.
        let allow = options.unwrap().allow.unwrap();

        let validations = Validations::get_validation_config(validation_file_path)
            .map_err(|e| format!("config {validation_file_path:?} is invalid: {e}"))?;

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
            allowed_errors: vec![Some(MentatError::default_error().into())],
            allowed_timestamp_start_index: Some(asserter.timestamp_start_index as isize),
        })
    }

    /// Says whether a given operation was successful or not.
    pub fn operation_successful(&self, operation: &mentat_types::Operation) -> AssertResult<bool> {
        let asserter = self
            .response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;

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
                "operation status {} is not found",
                operation.status.as_ref().unwrap()
            )))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::missing_docs_in_private_items)]
pub(crate) struct Configuration {
    pub(crate) network_identifier: Option<NetworkIdentifier>,
    pub(crate) genesis_block_identifier: Option<UncheckedBlockIdentifier>,
    pub(crate) allowed_operation_types: Vec<String>,
    pub(crate) allowed_operation_statuses: Vec<Option<OperationStatus>>,
    pub(crate) allowed_errors: Vec<Option<UncheckedMentatError>>,
    pub(crate) allowed_timestamp_start_index: Option<isize>,
}

impl Configuration {
    /// NewClientWithFile constructs a new Asserter using a specification
    /// file instead of responses. This can be useful for running reliable
    /// systems that error when updates to the server (more error types,
    /// more operations, etc.) significantly change how to parse the chain.
    /// The filePath provided is parsed relative to the current directory.
    pub(crate) fn new_client_with_file(path: &Path) -> AssertResult<Asserter> {
        let content = File::open(path).map_err(|e| {
            AsserterError::StringError(format!("failed to read file {}: {e}", path.display()))
        })?;
        let config: Self = serde_json::from_reader(content).map_err(|e| {
            AsserterError::StringError(format!(
                "failed to deserialize contents of file `{}`: {e}",
                path.display(),
            ))
        })?;

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
