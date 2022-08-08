use super::*;

#[test]
fn test_error_map() {
    let tests = vec![
        TestCase {
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
            result: None,
        },
        TestCase {
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
            result: Some(ErrorError::DescriptionEmpty.into()),
        },
        TestCase {
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
            result: Some(ErrorError::CodeIsNeg.into()),
        },
        TestCase {
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
            result: Some(ErrorError::RetriableMismatch.into()),
        },
        TestCase {
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
            result: Some(ErrorError::UnexpectedCode.into()),
        },
        TestCase {
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
            result: Some(ErrorError::MessageMismatch.into()),
        },
    ];

    let asserter = Asserter::new_client_with_responses(
        Some(NetworkIdentifier {
            blockchain: "hello".into(),
            network: "world".into(),
            sub_network_identifier: None,
        }),
        Some(NullableNetworkStatusResponse {
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
        }),
        Some(NullableNetworkOptionsResponse {
            version: Some(Version {
                rosetta_version: "1.4.0".to_string(),
                node_version: "1.0".to_string(),
                middleware_version: None,
                metadata: Default::default(),
            }),
            allow: Some(NullableAllow {
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
        }),
        None,
    )
    .unwrap();

    TestCase::run_err_match(tests, |t| asserter.error(Some(&t)));
}
