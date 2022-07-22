use indexmap::indexmap;

use super::test_utils::AsserterEqualityTest;
use crate::{
    asserter::{asserter_tools::Asserter, block::MIN_UNIX_EPOCH, errors::*},
    types::{
        Allow,
        BlockIdentifier,
        MentatError,
        NetworkIdentifier,
        NetworkOptionsResponse,
        NetworkStatusResponse,
        OperationStatus,
        Peer,
        Version,
    },
};

#[test]
fn test_error_map() {
    let tests = [
        AsserterEqualityTest {
            name: "matching error",
            payload: MentatError {
                status_code: 0,
                code: 10,
                message: "error 10".to_string(),
                description: None,
                retriable: true,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            },
            res: None,
        },
        AsserterEqualityTest {
            name: "empty description",
            payload: MentatError {
                status_code: 0,
                code: 10,
                message: "error 10".to_string(),
                description: Some(String::new()),
                retriable: true,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            },
            res: Some(ErrorError::DescriptionEmpty),
        },
        AsserterEqualityTest {
            name: "negative error",
            payload: MentatError {
                status_code: 0,
                code: -1,
                message: "error 10".to_string(),
                description: None,
                retriable: true,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            },
            res: Some(ErrorError::CodeIsNeg),
        },
        AsserterEqualityTest {
            name: "retriable error",
            payload: MentatError {
                status_code: 0,
                code: 10,
                message: "error 10".to_string(),
                description: None,
                retriable: false,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            },
            res: Some(ErrorError::RetriableMismatch),
        },
        AsserterEqualityTest {
            name: "code mismatch",
            payload: MentatError {
                status_code: 0,
                code: 20,
                message: "error 20".to_string(),
                description: None,
                retriable: false,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            },
            res: Some(ErrorError::UnexpectedCode),
        },
        AsserterEqualityTest {
            name: "code mismatch",
            payload: MentatError {
                status_code: 0,
                code: 10,
                message: "error 11".to_string(),
                description: None,
                retriable: true,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            },
            res: Some(ErrorError::MessageMismatch),
        },
    ];

    tests.into_iter().for_each(|test| {
        println!("test: {test}");

        let _asserter = Asserter::new_client_with_responses(
            NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                sub_network_identifier: None,
            },
            NetworkStatusResponse {
                current_block_identifier: Some(BlockIdentifier {
                    index: 100,
                    hash: "block 100".to_string(),
                }),
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: Some(BlockIdentifier {
                    index: 0,
                    hash: "block 0".to_string(),
                }),
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Some(Peer {
                    peer_id: "peer 1".to_string(),
                    metadata: Default::default(),
                })],
            },
            NetworkOptionsResponse {
                version: Some(Version {
                    rosetta_version: "1.4.0".to_string(),
                    node_version: "1.0".to_string(),
                    middleware_version: None,
                    metadata: Default::default(),
                }),
                allow: Some(Allow {
                    errors: vec![
                        Some(MentatError {
                            status_code: 0,
                            code: 10,
                            message: "error 10".to_string(),
                            description: None,
                            retriable: true,
                            details: Default::default(),
                        }),
                        Some(MentatError {
                            status_code: 0,
                            code: 1,
                            message: "error 1".to_string(),
                            description: None,
                            retriable: false,
                            details: Default::default(),
                        }),
                    ],
                    operation_statuses: vec![
                        Some(OperationStatus {
                            status: "SUCCESS".to_string(),
                            successful: true,
                        }),
                        Some(OperationStatus {
                            status: "FAILURE".to_string(),
                            successful: false,
                        }),
                    ],
                    operation_types: vec!["PAYMENT".to_string()],
                    ..Default::default()
                }),
            },
            // TODO make this optional???
            Default::default(),
        )
        .unwrap();

        // let resp = asserter.unwrap().error(&test.err);
        todo!()
    });
}
