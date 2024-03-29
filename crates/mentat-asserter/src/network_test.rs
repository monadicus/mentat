use super::*;

#[test]
fn test_network_identifier() {
    let tests = vec![
        TestCase {
            name: "valid network",
            payload: Some(NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: "mainnet".into(),
                sub_network_identifier: Default::default(),
            }),
            criteria: None,
        },
        TestCase {
            name: "nil network",
            payload: None,
            criteria: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        TestCase {
            name: "invalid blockchain",
            payload: Some(NetworkIdentifier {
                blockchain: Default::default(),
                network: "mainnet".into(),
                sub_network_identifier: Default::default(),
            }),
            criteria: Some(NetworkError::NetworkIdentifierBlockchainMissing.into()),
        },
        TestCase {
            name: "invalid network",
            payload: Some(NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: Default::default(),
                sub_network_identifier: Default::default(),
            }),
            criteria: Some(NetworkError::NetworkIdentifierNetworkMissing.into()),
        },
        TestCase {
            name: "valid sub_network",
            payload: Some(NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: "mainnet".into(),
                sub_network_identifier: Some(SubNetworkIdentifier {
                    network: "shard 1".into(),
                    metadata: Default::default(),
                }),
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid sub_network",
            payload: Some(NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: "mainnet".into(),
                sub_network_identifier: Some(Default::default()),
            }),
            criteria: Some(NetworkError::SubNetworkIdentifierInvalid.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| network_identifier(t.as_ref()));
}

#[test]
fn test_version() {
    let middleware_version = Some("1.2".to_string());
    let invalid_middleware_version = Some(String::new());
    let rosetta_version = "1.4.0".to_string();
    let node_version = "1.0".to_string();

    let tests = vec![
        TestCase {
            name: "valid version",
            payload: Some(Version {
                rosetta_version: rosetta_version.clone(),
                node_version: node_version.clone(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid version with middleware",
            payload: Some(Version {
                rosetta_version: rosetta_version.clone(),
                node_version: node_version.clone(),
                middleware_version,
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "old RosettaVersion",
            payload: Some(Version {
                rosetta_version: "1.2.0".to_string(),
                node_version: node_version.clone(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "nil version",
            payload: None,
            criteria: Some(NetworkError::VersionIsNil.into()),
        },
        TestCase {
            name: "invalid NodeVersion",
            payload: Some(Version {
                rosetta_version: rosetta_version.clone(),
                node_version: String::new(),
                ..Default::default()
            }),
            criteria: Some(NetworkError::VersionNodeVersionMissing.into()),
        },
        TestCase {
            name: "invalid MiddlewareVersion",
            payload: Some(Version {
                rosetta_version,
                node_version,
                middleware_version: invalid_middleware_version,
                ..Default::default()
            }),
            criteria: Some(NetworkError::VersionMiddlewareVersionMissing.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| version(t.as_ref()));
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
    let balance_exemptions = vec![Some(UncheckedBalanceExemption {
        sub_account_address: None,
        currency: Some(UncheckedCurrency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        exemption_type: UncheckedExemptionType::DYNAMIC.into(),
    })];
    let neg_index = Some(-1);
    let index = Some(100);

    let tests = vec![
        TestCase {
            name: "valid Allow",
            payload: Some(UncheckedAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid Allow with call methods and exemptions",
            payload: Some(UncheckedAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods: call_methods.clone(),
                balance_exemptions: balance_exemptions.clone(),
                historical_balance_lookup: true,
                timestamp_start_index: index,
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid Allow with exemptions and no historical",
            payload: Some(UncheckedAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods,
                balance_exemptions: balance_exemptions.clone(),
                ..Default::default()
            }),
            criteria: Some(NetworkError::BalanceExemptionNoHistoricalLookup.into()),
        },
        TestCase {
            name: "invalid timestamp start index",
            payload: Some(UncheckedAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                timestamp_start_index: neg_index,
                ..Default::default()
            }),
            criteria: Some(NetworkError::TimestampStartIndexInvalid.into()),
        },
        TestCase {
            name: "nil Allow",
            payload: None,
            criteria: Some(NetworkError::AllowIsNil.into()),
        },
        TestCase {
            name: "no OperationStatuses",
            payload: Some(UncheckedAllow {
                operation_types: operation_types.clone(),
                ..Default::default()
            }),
            criteria: Some(NetworkError::NoAllowedOperationStatuses.into()),
        },
        TestCase {
            name: "no successful OperationStatuses",
            payload: Some(UncheckedAllow {
                operation_statuses: vec![operation_statuses[1].clone()],
                operation_types: operation_types.clone(),
                ..Default::default()
            }),
            criteria: Some(NetworkError::NoSuccessfulAllowedOperationStatuses.into()),
        },
        TestCase {
            name: "no OperationTypes",
            payload: Some(UncheckedAllow {
                operation_statuses: operation_statuses.clone(),
                ..Default::default()
            }),
            criteria: Some(UtilError::StringArrayEmpty.into()),
        },
        TestCase {
            name: "duplicate call methods",
            payload: Some(UncheckedAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods: vec!["call".into(), "call".into()],
                balance_exemptions,
                ..Default::default()
            }),
            criteria: Some(AsserterError::from(
                "Allow.CallMethods contains a duplicate call".to_string(),
            )),
        },
        TestCase {
            name: "empty exemption",
            payload: Some(UncheckedAllow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods: vec!["call".into()],
                balance_exemptions: vec![Some(UncheckedBalanceExemption {
                    sub_account_address: None,
                    currency: None,
                    exemption_type: UncheckedExemptionType::DYNAMIC.into(),
                })],
                ..Default::default()
            }),
            criteria: Some(NetworkError::BalanceExemptionMissingSubject.into()),
        },
        TestCase {
            name: "invalid exemption type",
            payload: Some(UncheckedAllow {
                operation_statuses,
                operation_types,
                call_methods: vec!["call".into()],
                balance_exemptions: vec![Some(UncheckedBalanceExemption {
                    sub_account_address: None,
                    currency: None,
                    exemption_type: "test".into(),
                })],
                ..Default::default()
            }),
            criteria: Some(NetworkError::BalanceExemptionTypeInvalid.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| allow(t.as_ref()));
}

#[test]
fn test_error() {
    let tests = vec![
        TestCase {
            name: "valid error",
            payload: Some(UncheckedMentatError {
                code: 12,
                message: "signature invalid".into(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "nil error",
            payload: None,
            criteria: Some(ErrorError::IsNil.into()),
        },
        TestCase {
            name: "negative code",
            payload: Some(UncheckedMentatError {
                code: -1,
                message: "signature invalid".into(),
                ..Default::default()
            }),
            criteria: Some(ErrorError::CodeIsNeg.into()),
        },
        TestCase {
            name: "empty message",
            payload: Some(UncheckedMentatError {
                code: 0,
                message: String::new(),
                ..Default::default()
            }),
            criteria: Some(ErrorError::MessageMissing.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| error(t.as_ref()));
}

#[test]
fn test_errors() {
    let tests = vec![
        TestCase {
            name: "valid errors",
            payload: vec![
                Some(UncheckedMentatError {
                    code: 0,
                    message: "error 1".into(),
                    ..Default::default()
                }),
                Some(UncheckedMentatError {
                    code: 2,
                    message: "error 2".into(),
                    ..Default::default()
                }),
            ],
            criteria: None,
        },
        TestCase {
            name: "details populated",
            payload: vec![
                Some(UncheckedMentatError {
                    code: 0,
                    message: "error 1".into(),
                    details: indexmap!(
                      "hello".to_string() => "goodbye".into()
                    ),
                    ..Default::default()
                }),
                Some(UncheckedMentatError {
                    code: 1,
                    message: "error 2".into(),
                    ..Default::default()
                }),
            ],
            criteria: Some(NetworkError::ErrorDetailsPopulated.into()),
        },
        TestCase {
            name: "duplicate error codes",
            payload: vec![
                Some(UncheckedMentatError {
                    code: 0,
                    message: "error 1".into(),
                    ..Default::default()
                }),
                Some(UncheckedMentatError {
                    code: 0,
                    message: "error 2".into(),
                    ..Default::default()
                }),
            ],
            criteria: Some(NetworkError::ErrorCodeUsedMultipleTimes.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| errors(&t));
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

    let tests = vec![
        TestCase {
            name: "valid network list",
            payload: Some(UncheckedNetworkListResponse {
                network_identifiers: vec![network_1, network_1_sub.clone(), network_2],
            }),
            criteria: None,
        },
        TestCase {
            name: "nil network list",
            payload: None,
            criteria: Some(NetworkError::NetworkListResponseIsNil.into()),
        },
        TestCase {
            name: "network list duplicate",
            payload: Some(UncheckedNetworkListResponse {
                network_identifiers: vec![network_1_sub.clone(), network_1_sub],
            }),
            criteria: Some(NetworkError::NetworkListResponseNetworksContainsDuplicates.into()),
        },
        TestCase {
            name: "invalid network",
            payload: Some(UncheckedNetworkListResponse {
                network_identifiers: vec![network_3.clone()],
            }),
            criteria: Some(
                format!(
                    "network identifier {network_3:?} is invalid: {}",
                    NetworkError::NetworkIdentifierBlockchainMissing
                )
                .into(),
            ),
        },
    ];

    TestCase::run_err_match(tests, |t| network_list_response(t.as_ref()));
}
