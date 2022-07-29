use super::*;

#[test]
fn test_error_map() {
    let tests = [
        CustomAsserterTest {
            name: "matching error",
            payload: Some(MentatError {
                status_code: 0,
                code: 10,
                message: "error 10".to_string(),
                description: None,
                retriable: true,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
            name: "empty description",
            payload: Some(MentatError {
                status_code: 0,
                code: 10,
                message: "error 10".to_string(),
                description: Some(String::new()),
                retriable: true,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            }),
            extras: (),
            err: Some(ErrorError::DescriptionEmpty.into()),
        },
        CustomAsserterTest {
            name: "negative error",
            payload: Some(MentatError {
                status_code: 0,
                code: -1,
                message: "error 10".to_string(),
                description: None,
                retriable: true,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            }),
            extras: (),
            err: Some(ErrorError::CodeIsNeg.into()),
        },
        CustomAsserterTest {
            name: "retriable error",
            payload: Some(MentatError {
                status_code: 0,
                code: 10,
                message: "error 10".to_string(),
                description: None,
                retriable: false,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            }),
            extras: (),
            err: Some(ErrorError::RetriableMismatch.into()),
        },
        CustomAsserterTest {
            name: "code mismatch",
            payload: Some(MentatError {
                status_code: 0,
                code: 20,
                message: "error 20".to_string(),
                description: None,
                retriable: false,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            }),
            extras: (),
            err: Some(ErrorError::UnexpectedCode.into()),
        },
        CustomAsserterTest {
            name: "code mismatch",
            payload: Some(MentatError {
                status_code: 0,
                code: 10,
                message: "error 11".to_string(),
                description: None,
                retriable: true,
                details: indexmap!(
                  "hello".to_string() => "goodbye".into()
                ),
            }),
            extras: (),
            err: Some(ErrorError::MessageMismatch.into()),
        },
    ];

    let asserter = |_: &()| {
        Asserter::new_client_with_responses(
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
        .unwrap()
    };

    CustomAsserterTest::custom_asserter_tests(&tests, asserter, Asserter::error);
}
