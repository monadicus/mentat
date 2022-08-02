use super::*;

#[test]
fn test_network_identifier() {
    let tests = [
        AsserterTest {
            name: "valid network",
            payload: Some(NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: "mainnet".into(),
                sub_network_identifier: Default::default(),
            }),
            err: None,
        },
        AsserterTest {
            name: "nil network",
            payload: None,
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterTest {
            name: "invalid blockchain",
            payload: Some(NetworkIdentifier {
                blockchain: Default::default(),
                network: "mainnet".into(),
                sub_network_identifier: Default::default(),
            }),
            err: Some(NetworkError::NetworkIdentifierBlockchainMissing.into()),
        },
        AsserterTest {
            name: "invalid network",
            payload: Some(NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: Default::default(),
                sub_network_identifier: Default::default(),
            }),
            err: Some(NetworkError::NetworkIdentifierNetworkMissing.into()),
        },
        AsserterTest {
            name: "valid sub_network",
            payload: Some(NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: "mainnet".into(),
                sub_network_identifier: Some(SubNetworkIdentifier {
                    network: "shard 1".into(),
                    metadata: Default::default(),
                }),
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid sub_network",
            payload: Some(NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: "mainnet".into(),
                sub_network_identifier: Some(Default::default()),
            }),
            err: Some(NetworkError::SubNetworkIdentifierInvalid.into()),
        },
    ];

    AsserterTest::run(&tests, network_identifier);
}

#[test]
fn test_version() {
    let middleware_version = Some("1.2".to_string());
    let invalid_middleware_version = Some(String::new());
    let rosetta_version = "1.4.0".to_string();
    let node_version = "1.0".to_string();

    let tests = [
        AsserterTest {
            name: "valid version",
            payload: Some(Version {
                rosetta_version: rosetta_version.clone(),
                node_version: node_version.clone(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "valid version with middleware",
            payload: Some(Version {
                rosetta_version: rosetta_version.clone(),
                node_version: node_version.clone(),
                middleware_version,
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "old RosettaVersion",
            payload: Some(Version {
                rosetta_version: "1.2.0".to_string(),
                node_version: node_version.clone(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "nil version",
            payload: None,
            err: Some(NetworkError::VersionIsNil.into()),
        },
        AsserterTest {
            name: "invalid NodeVersion",
            payload: Some(Version {
                rosetta_version: rosetta_version.clone(),
                node_version: String::new(),
                ..Default::default()
            }),
            err: Some(NetworkError::VersionNodeVersionMissing.into()),
        },
        AsserterTest {
            name: "invalid MiddlewareVersion",
            payload: Some(Version {
                rosetta_version,
                node_version,
                middleware_version: invalid_middleware_version,
                ..Default::default()
            }),
            err: Some(NetworkError::VersionMiddlewareVersionMissing.into()),
        },
    ];

    AsserterTest::run(&tests, version);
}

#[test]
fn test_allow() {
    let operation_statuses = vec![
        Some(OperationStatus {
            status: "SUCCESS".to_string(),
            successful: true,
        }),
        Some(OperationStatus {
            status: "FAILURE".to_string(),
            successful: false,
        }),
    ];
    let operation_types = vec!["PAYMENT".to_string()];
    let call_methods = vec!["call".to_string()];
    let balance_exemptions = vec![Some(NullableBalanceExemption {
        sub_account_address: None,
        currency: Some(NullableCurrency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        exemption_type: NullableExemptionType::DYNAMIC.into(),
    })];
    let neg_index = Some(-1);
    let index = Some(100);

    let tests = [
        AsserterTest {
            name: "valid Allow",
            payload: Some(NullableAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "valid Allow with call methods and exemptions",
            payload: Some(NullableAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods: call_methods.clone(),
                balance_exemptions: balance_exemptions.clone(),
                historical_balance_lookup: true,
                timestamp_start_index: index,
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "valid Allow with exemptions and no historical",
            payload: Some(NullableAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods,
                balance_exemptions: balance_exemptions.clone(),
                ..Default::default()
            }),
            err: Some(NetworkError::BalanceExemptionNoHistoricalLookup.into()),
        },
        AsserterTest {
            name: "invalid timestamp start index",
            payload: Some(NullableAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                timestamp_start_index: neg_index,
                ..Default::default()
            }),
            err: Some(NetworkError::TimestampStartIndexInvalid.into()),
        },
        AsserterTest {
            name: "nil Allow",
            payload: None,
            err: Some(NetworkError::AllowIsNil.into()),
        },
        AsserterTest {
            name: "no OperationStatuses",
            payload: Some(NullableAllow {
                operation_types: operation_types.clone(),
                ..Default::default()
            }),
            err: Some(NetworkError::NoAllowedOperationStatuses.into()),
        },
        AsserterTest {
            name: "no successful OperationStatuses",
            payload: Some(NullableAllow {
                operation_statuses: vec![operation_statuses[1].clone()],
                operation_types: operation_types.clone(),
                ..Default::default()
            }),
            err: Some(NetworkError::NoSuccessfulAllowedOperationStatuses.into()),
        },
        AsserterTest {
            name: "no OperationTypes",
            payload: Some(NullableAllow {
                operation_statuses: operation_statuses.clone(),
                ..Default::default()
            }),
            err: Some(AsserterError::from(
                "no Allow.OperationTypes found".to_string(),
            )),
        },
        AsserterTest {
            name: "duplicate call methods",
            payload: Some(NullableAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods: vec!["call".into(), "call".into()],
                balance_exemptions,
                ..Default::default()
            }),
            err: Some(AsserterError::from(
                "Allow.CallMethods contains a duplicate call".to_string(),
            )),
        },
        AsserterTest {
            name: "empty exemption",
            payload: Some(NullableAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods: vec!["call".into()],
                balance_exemptions: vec![Some(NullableBalanceExemption {
                    sub_account_address: None,
                    currency: None,
                    exemption_type: NullableExemptionType::DYNAMIC.into(),
                })],
                ..Default::default()
            }),
            err: Some(NetworkError::BalanceExemptionMissingSubject.into()),
        },
        AsserterTest {
            name: "invalid exemption type",
            payload: Some(NullableAllow {
                operation_statuses,
                operation_types,
                call_methods: vec!["call".into()],
                balance_exemptions: vec![Some(NullableBalanceExemption {
                    sub_account_address: None,
                    currency: None,
                    exemption_type: "test".into(),
                })],
                ..Default::default()
            }),
            err: Some(NetworkError::BalanceExemptionTypeInvalid.into()),
        },
    ];

    AsserterTest::run(&tests, allow);
}

#[test]
fn test_error() {
    let tests = [
        AsserterTest {
            name: "valid error",
            payload: Some(MentatError {
                code: 12,
                message: "signature invalid".into(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "nil error",
            payload: None,
            err: Some(ErrorError::IsNil.into()),
        },
        AsserterTest {
            name: "negative code",
            payload: Some(MentatError {
                code: -1,
                message: "signature invalid".into(),
                ..Default::default()
            }),
            err: Some(ErrorError::CodeIsNeg.into()),
        },
        AsserterTest {
            name: "empty message",
            payload: Some(MentatError {
                code: 0,
                message: String::new(),
                ..Default::default()
            }),
            err: Some(ErrorError::MessageMissing.into()),
        },
    ];

    AsserterTest::run(&tests, error);
}

#[test]
fn test_errors() {
    let tests = [
        AsserterTest {
            name: "valid errors",
            payload: Some(vec![
                Some(MentatError {
                    code: 0,
                    message: "error 1".into(),
                    ..Default::default()
                }),
                Some(MentatError {
                    code: 2,
                    message: "error 2".into(),
                    ..Default::default()
                }),
            ]),
            err: None,
        },
        AsserterTest {
            name: "details populated",
            payload: Some(vec![
                Some(MentatError {
                    code: 0,
                    message: "error 1".into(),
                    details: indexmap!(
                      "hello".to_string() => "goodbye".into()
                    ),
                    ..Default::default()
                }),
                Some(MentatError {
                    code: 1,
                    message: "error 2".into(),
                    ..Default::default()
                }),
            ]),
            err: Some(NetworkError::ErrorDetailsPopulated.into()),
        },
        AsserterTest {
            name: "duplicate error codes",
            payload: Some(vec![
                Some(MentatError {
                    code: 0,
                    message: "error 1".into(),
                    ..Default::default()
                }),
                Some(MentatError {
                    code: 0,
                    message: "error 2".into(),
                    ..Default::default()
                }),
            ]),
            err: Some(NetworkError::ErrorCodeUsedMultipleTimes.into()),
        },
    ];

    AsserterTest::run(&tests, |t| errors(t.unwrap()));
}

#[test]
fn test_network_list_response() {
    let network_1 = Some(NetworkIdentifier {
        blockchain: "blockchain 1".into(),
        network: "network 1".into(),
        sub_network_identifier: None,
    });
    let network_1_sub = Some(NetworkIdentifier {
        sub_network_identifier: Some(SubNetworkIdentifier {
            network: "subnetwork".into(),
            metadata: Default::default(),
        }),
        ..network_1.clone().unwrap()
    });
    let network_2 = Some(NetworkIdentifier {
        blockchain: "blockchain 2".into(),
        network: "network 2".into(),
        sub_network_identifier: None,
    });
    let network_3 = Some(NetworkIdentifier {
        network: "network 2".into(),
        ..Default::default()
    });

    let tests = [
        AsserterTest {
            name: "valid network list",
            payload: Some(NullableNetworkListResponse {
                network_identifiers: vec![network_1, network_1_sub.clone(), network_2],
            }),
            err: None,
        },
        AsserterTest {
            name: "nil network list",
            payload: None,
            err: Some(NetworkError::NetworkListResponseIsNil.into()),
        },
        AsserterTest {
            name: "network list duplicate",
            payload: Some(NullableNetworkListResponse {
                network_identifiers: vec![network_1_sub.clone(), network_1_sub],
            }),
            err: Some(NetworkError::NetworkListResponseNetworksContainsDuplicates.into()),
        },
        AsserterTest {
            name: "invalid network",
            payload: Some(NullableNetworkListResponse {
                network_identifiers: vec![network_3],
            }),
            err: Some(NetworkError::NetworkIdentifierBlockchainMissing.into()),
        },
    ];

    AsserterTest::run(&tests, network_list_response);
}
