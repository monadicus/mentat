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
    network_status: Option<NetworkStatusResponse>,
    network_options: Option<NetworkOptionsResponse>,
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
    let valid_network_status = Some(NetworkStatusResponse {
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
    let valid_network_status_sync_status = Some(NetworkStatusResponse {
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
    let invalid_network_status = Some(NetworkStatusResponse {
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
    let invalid_network_status_sync_status = Some(NetworkStatusResponse {
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
    let valid_network_options = Some(NetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(Allow {
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
    let valid_network_options_with_start_index = Some(NetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(Allow {
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
    let invalid_network_options = Some(NetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(Allow {
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
    let duplicate_statuses = Some(NetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(Allow {
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
    let duplicate_types = Some(NetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(Allow {
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
    let negative_start_index = Some(NetworkOptionsResponse {
        version: Some(Version {
            rosetta_version: "1.4.0".into(),
            node_version: "1.0".into(),
            ..Default::default()
        }),
        allow: Some(Allow {
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
        AsserterTest {
            name: "valid responses",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: valid_network_options.clone(),
                validation_file_path: None,
                skip_load_test: false,
            }),
            err: None,
        },
        AsserterTest {
            name: "valid responses (with sync status)",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status_sync_status,
                network_options: valid_network_options.clone(),
                validation_file_path: None,
                skip_load_test: false,
            }),
            err: None,
        },
        AsserterTest {
            name: "valid responses (with start index)",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: valid_network_options_with_start_index,
                validation_file_path: None,
                skip_load_test: false,
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid network status",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: invalid_network_status,
                network_options: valid_network_options.clone(),
                validation_file_path: None,
                skip_load_test: false,
            }),
            err: Some("BlockIdentifier is Nil".into()),
        },
        AsserterTest {
            name: "invalid network status (with sync status)",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: invalid_network_status_sync_status,
                network_options: valid_network_options.clone(),
                validation_file_path: None,
                skip_load_test: true,
            }),
            err: Some("SyncStatus.CurrentIndex is negative".into()),
        },
        AsserterTest {
            name: "invalid network options",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: invalid_network_options,
                validation_file_path: None,
                skip_load_test: false,
            }),
            err: Some("no Allow.OperationStatuses found".into()),
        },
        AsserterTest {
            name: "duplicate operation statuses",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: duplicate_statuses,
                validation_file_path: None,
                skip_load_test: false,
            }),
            err: Some("Allow.OperationStatuses contains a duplicate Success".into()),
        },
        AsserterTest {
            name: "duplicate operation types",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: duplicate_types,
                validation_file_path: None,
                skip_load_test: false,
            }),
            err: Some("Allow.OperationTypes contains a duplicate Transfer".into()),
        },
        AsserterTest {
            name: "invalid start index",
            payload: Some(TestNewExtras {
                network: valid_network.clone(),
                network_status: valid_network_status.clone(),
                network_options: negative_start_index,
                validation_file_path: None,
                skip_load_test: false,
            }),
            err: Some("TimestampStartIndex is invalid: -1".into()),
        },
    ];

    // TODO make use of test framework
    tests.into_iter().for_each(|test| {
        println!("{}: ", test.name);
        let payload = test.payload.unwrap();
        let res = Asserter::new_client_with_responses(
            payload.network.clone(),
            payload.network_status.clone(),
            payload.network_options.clone(),
            payload.validation_file_path.as_deref(),
        );

        if test.err.is_some() {
            assert_correct(&test.err, &res);
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
                        .unwrap()
                        .allow
                        .unwrap()
                        .timestamp_start_index,
                    config.allowed_timestamp_start_index
                );
            } else {
                assert_eq!(
                    Some(
                        payload
                            .network_status
                            .unwrap()
                            .genesis_block_identifier
                            .unwrap()
                            .index
                            + 1
                    ),
                    config.allowed_timestamp_start_index
                )
            }
        }
    });

    let tmp_file = temp_dir().join("test.json");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&tmp_file)
        .unwrap();
    file.write_all(b"blah").unwrap();

    println!("non-existent file: ");
    Configuration::new_client_with_file(Path::new("blah")).unwrap_err();

    println!("file not formatted correctly: ");
    Configuration::new_client_with_file(&tmp_file).unwrap_err();

    println!("default no validation file: ");
    let asserter = Asserter::new_client_with_responses(
        valid_network.clone(),
        valid_network_status.clone(),
        valid_network_options.clone(),
        Some(Path::new("")),
    )
    .unwrap();
    assert!(!asserter.validations.enabled);

    println!("non existent validation file: ");
    Asserter::new_client_with_responses(
        valid_network.clone(),
        valid_network_status.clone(),
        valid_network_options.clone(),
        Some(Path::new("blah")),
    )
    .unwrap_err();

    println!("wrong format of validation file: ");
    Asserter::new_client_with_responses(
        valid_network,
        valid_network_status,
        valid_network_options,
        Some(&tmp_file),
    )
    .unwrap_err();
}