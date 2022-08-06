use std::{
    env::temp_dir,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use super::*;

#[derive(Default)]
struct TestNewExtras {
    network: Option<NetworkIdentifier>,
    network_status: Option<NullableNetworkStatusResponse>,
    network_options: Option<NullableNetworkOptionsResponse>,
    validation_file_path: Option<PathBuf>,
    skip_load_test: bool,
}

#[test]
fn test_new() {
    let valid_network = Some(NetworkIdentifier {
        blockchain: "hello".into(),
        network: "world".into(),
        sub_network_identifier: Default::default(),
    });
    let valid_network_status = Some(NullableNetworkStatusResponse {
        genesis_block_identifier: Some(BlockIdentifier {
            index: 0,
            hash: "block 0".into(),
        }),
        current_block_identifier: Some(BlockIdentifier {
            index: 100,
            hash: "block 100".into(),
        }),
        current_block_timestamp: MIN_UNIX_EPOCH + 1,
        peers: vec![Some(Peer {
            peer_id: "peer 1".into(),
            metadata: Default::default(),
        })],
        ..Default::default()
    });
    let valid_network_status_sync_status = Some(NullableNetworkStatusResponse {
        genesis_block_identifier: Some(BlockIdentifier {
            index: 0,
            hash: "block 0".into(),
        }),
        current_block_identifier: Some(BlockIdentifier {
            index: 100,
            hash: "block 100".into(),
        }),
        current_block_timestamp: MIN_UNIX_EPOCH + 1,
        peers: vec![Some(Peer {
            peer_id: "peer 1".into(),
            metadata: Default::default(),
        })],
        sync_status: Some(SyncStatus {
            current_index: Some(100),
            stage: Some("pre-sync".into()),
            ..Default::default()
        }),
        oldest_block_identifier: None,
    });
    let invalid_network_status = Some(NullableNetworkStatusResponse {
        current_block_identifier: Some(BlockIdentifier {
            index: 100,
            hash: "block 100".into(),
        }),
        current_block_timestamp: MIN_UNIX_EPOCH + 1,
        peers: vec![Some(Peer {
            peer_id: "peer 1".into(),
            metadata: Default::default(),
        })],
        ..Default::default()
    });
    let invalid_network_status_sync_status = Some(NullableNetworkStatusResponse {
        genesis_block_identifier: Some(BlockIdentifier {
            index: 0,
            hash: "block 0".into(),
        }),
        current_block_identifier: Some(BlockIdentifier {
            index: 100,
            hash: "block 100".into(),
        }),
        current_block_timestamp: MIN_UNIX_EPOCH + 1,
        peers: vec![Some(Peer {
            peer_id: "peer 1".into(),
            metadata: Default::default(),
        })],
        sync_status: Some(SyncStatus {
            current_index: Some(-100),
            stage: Some("pre-sync".into()),
            ..Default::default()
        }),
        oldest_block_identifier: None,
    });
    let valid_network_options = Some(NullableNetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(NullableAllow {
            operation_statuses: vec![Some(OperationStatus {
                status: "Success".into(),
                successful: true,
            })],
            operation_types: vec!["Transfer".to_string()],
            errors: vec![Some(MentatError {
                status_code: 0,
                code: 1,
                message: "error".into(),
                retriable: true,
                ..Default::default()
            })],
            historical_balance_lookup: true,
            ..Default::default()
        }),
    });
    let valid_network_options_with_start_index = Some(NullableNetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(NullableAllow {
            operation_statuses: vec![Some(OperationStatus {
                status: "Success".into(),
                successful: true,
            })],
            operation_types: vec!["Transfer".to_string()],
            errors: vec![Some(MentatError {
                status_code: 0,
                code: 1,
                message: "error".into(),
                retriable: true,
                ..Default::default()
            })],
            historical_balance_lookup: true,
            timestamp_start_index: Some(10),
            ..Default::default()
        }),
    });
    let invalid_network_options = Some(NullableNetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(NullableAllow {
            operation_types: vec!["Transfer".to_string()],
            errors: vec![Some(MentatError {
                status_code: 0,
                code: 1,
                message: "error".into(),
                retriable: true,
                ..Default::default()
            })],
            ..Default::default()
        }),
    });
    let duplicate_statuses = Some(NullableNetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(NullableAllow {
            operation_statuses: vec![
                Some(OperationStatus {
                    status: "Success".into(),
                    successful: true,
                }),
                Some(OperationStatus {
                    status: "Success".into(),
                    successful: false,
                }),
            ],
            operation_types: vec!["Transfer".to_string()],
            errors: vec![Some(MentatError {
                status_code: 0,
                code: 1,
                message: "error".into(),
                retriable: true,
                ..Default::default()
            })],
            ..Default::default()
        }),
    });
    let duplicate_types = Some(NullableNetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(NullableAllow {
            operation_statuses: vec![Some(OperationStatus {
                status: "Success".into(),
                successful: true,
            })],
            operation_types: vec!["Transfer".to_string(), "Transfer".to_string()],
            errors: vec![Some(MentatError {
                status_code: 0,
                code: 1,
                message: "error".into(),
                retriable: true,
                ..Default::default()
            })],
            ..Default::default()
        }),
    });
    let negative_start_index = Some(NullableNetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(NullableAllow {
            operation_statuses: vec![Some(OperationStatus {
                status: "Success".into(),
                successful: true,
            })],
            operation_types: vec!["Transfer".to_string()],
            errors: vec![Some(MentatError {
                status_code: 0,
                code: 1,
                message: "error".into(),
                retriable: true,
                ..Default::default()
            })],
            historical_balance_lookup: true,
            timestamp_start_index: Some(-1),
            ..Default::default()
        }),
    });

    let tests = [
        FnTest {
            name: "valid responses",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: valid_network_options.clone(),
                validation_file_path: None,
                skip_load_test: false,
            }),
            result: None,
        },
        FnTest {
            name: "valid responses (with sync status)",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status_sync_status,
                network_options: valid_network_options.clone(),
                validation_file_path: None,
                skip_load_test: false,
            }),
            result: None,
        },
        FnTest {
            name: "valid responses (with start index)",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: valid_network_options_with_start_index,
                validation_file_path: None,
                skip_load_test: false,
            }),
            result: None,
        },
        FnTest {
            name: "invalid network status",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: invalid_network_status,
                network_options: valid_network_options.clone(),
                validation_file_path: None,
                skip_load_test: false,
            }),
            result: Some("BlockIdentifier is nil".into()),
        },
        FnTest {
            name: "invalid network status (with sync status)",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: invalid_network_status_sync_status,
                network_options: valid_network_options.clone(),
                validation_file_path: None,
                skip_load_test: true,
            }),
            result: Some("SyncStatus.CurrentIndex is negative".into()),
        },
        FnTest {
            name: "invalid network options",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: invalid_network_options,
                validation_file_path: None,
                skip_load_test: false,
            }),
            result: Some("no Allow.OperationStatuses found".into()),
        },
        FnTest {
            name: "duplicate operation statuses",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: duplicate_statuses,
                validation_file_path: None,
                skip_load_test: false,
            }),
            result: Some("Allow.OperationStatuses contains a duplicate Success".into()),
        },
        FnTest {
            name: "duplicate operation types",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: duplicate_types,
                validation_file_path: None,
                skip_load_test: false,
            }),
            result: Some("Allow.OperationTypes contains a duplicate Transfer".into()),
        },
        FnTest {
            name: "invalid start index",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: negative_start_index,
                validation_file_path: None,
                skip_load_test: false,
            }),
            result: Some("TimestampStartIndex is invalid: -1".into()),
        },
    ];

    // TODO make use of test framework
    for test in tests {
        print!("{}: ", test.name);
        let payload = test.payload.unwrap();
        let res = Asserter::new_client_with_responses(
            payload.network.clone(),
            payload.network_status.clone(),
            payload.network_options.clone(),
            payload.validation_file_path.as_ref(),
        );

        if test.result.is_some() {
            assert!(check_err_match(&test.result, &res));
        } else {
            let asserter = res.unwrap();

            let config = asserter.client_configuration().unwrap();
            assert_eq!(payload.network, config.network_identifier);
            assert_eq!(
                payload
                    .network_status
                    .as_ref()
                    .unwrap()
                    .genesis_block_identifier,
                config.genesis_block_identifier
            );
            assert_eq!(
                payload
                    .network_options
                    .as_ref()
                    .unwrap()
                    .allow
                    .as_ref()
                    .unwrap()
                    .operation_types,
                config.allowed_operation_types
            );
            assert_eq!(
                payload
                    .network_options
                    .as_ref()
                    .unwrap()
                    .allow
                    .as_ref()
                    .unwrap()
                    .operation_statuses,
                config.allowed_operation_statuses
            );
            assert_eq!(
                payload
                    .network_options
                    .as_ref()
                    .unwrap()
                    .allow
                    .as_ref()
                    .unwrap()
                    .errors,
                config.allowed_errors
            );

            if payload
                .network_options
                .as_ref()
                .unwrap()
                .allow
                .as_ref()
                .unwrap()
                .timestamp_start_index
                .is_some()
            {
                assert_eq!(
                    payload
                        .network_options
                        .as_ref()
                        .unwrap()
                        .allow
                        .as_ref()
                        .unwrap()
                        .timestamp_start_index,
                    config.allowed_timestamp_start_index
                );
            } else {
                assert_eq!(
                    Some(
                        payload
                            .network_status
                            .as_ref()
                            .unwrap()
                            .genesis_block_identifier
                            .as_ref()
                            .unwrap()
                            .index
                            + 1
                    ),
                    config.allowed_timestamp_start_index
                )
            }
            println!("ok!");
        }

        if !payload.skip_load_test {
            print!("{} with file: ", test.name);

            let allow = payload.network_options.unwrap().allow.unwrap();
            let genesis_block_identifier = payload.network_status.unwrap().genesis_block_identifier;
            let allowed_timestamp_start_index = if allow.timestamp_start_index.is_some() {
                allow.timestamp_start_index
            } else {
                genesis_block_identifier.as_ref().map(|t| t.index)
            };
            let file_config = Configuration {
                network_identifier: payload.network.clone(),
                allowed_timestamp_start_index,
                genesis_block_identifier: genesis_block_identifier.clone(),
                allowed_operation_types: allow.operation_types.clone(),
                allowed_operation_statuses: allow.operation_statuses.clone(),
                allowed_errors: allow.errors.clone(),
            };

            let tmp_file_path = temp_dir().join("test.json");
            let mut tmp_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&tmp_file_path)
                .unwrap();

            let data = serde_json::to_string_pretty(&file_config).unwrap();

            tmp_file.write_all(data.as_bytes()).unwrap();

            let asserter = Configuration::new_client_with_file(&tmp_file_path);

            if let Some(e) = test.result {
                let err = asserter.unwrap_err();
                assert!(check_err_match::<(), _>(&Some(err), &Err(e)))
            } else {
                let configuration = asserter.unwrap().client_configuration().unwrap();
                assert_eq!(payload.network, configuration.network_identifier);
                assert_eq!(
                    genesis_block_identifier,
                    configuration.genesis_block_identifier
                );
                assert_eq!(allow.operation_types, configuration.allowed_operation_types);
                assert_eq!(
                    allow.operation_statuses,
                    configuration.allowed_operation_statuses
                );
                assert_eq!(allow.errors, configuration.allowed_errors);
                assert_eq!(
                    allowed_timestamp_start_index,
                    configuration.allowed_timestamp_start_index
                );
                println!("ok!");
            }
        }
    }

    let tmp_file_path = temp_dir().join("test.json");
    let mut tmp_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&tmp_file_path)
        .unwrap();
    tmp_file.write_all(b"blah").unwrap();

    println!("non-existent file: ");
    Configuration::new_client_with_file(Path::new("blah")).unwrap_err();

    println!("file not formatted correctly: ");
    Configuration::new_client_with_file(&tmp_file_path).unwrap_err();

    println!("default no validation file: ");
    let asserter = Asserter::new_client_with_responses(
        valid_network.clone(),
        valid_network_status.clone(),
        valid_network_options.clone(),
        None,
    )
    .unwrap();
    assert!(!asserter.validations.enabled);

    println!("non existent validation file: ");
    Asserter::new_client_with_responses(
        valid_network.clone(),
        valid_network_status.clone(),
        valid_network_options.clone(),
        Some(&PathBuf::from("blah")),
    )
    .unwrap_err();

    println!("wrong format of validation file: ");
    Asserter::new_client_with_responses(
        valid_network,
        valid_network_status,
        valid_network_options,
        Some(&tmp_file_path),
    )
    .unwrap_err();
}
