use indexmap::indexmap;

use super::test_utils::AsserterTest;
use crate::{
    asserter::{
        errors::{AsserterError, ErrorError, NetworkError},
        network::{allow, error, errors, network_identifier, network_list_response, version},
    },
    types::{
        Allow, BalanceExemption, Currency, ExemptionType, MentatError, NetworkIdentifier,
        NetworkListResponse, OperationStatus, SubNetworkIdentifier, Version,
    },
};

#[test]
fn test_network_identifier() {
    let tests = [
        AsserterTest {
            name: "valid network",
            payload: NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: "mainnet".into(),
                sub_network_identifier: Default::default(),
            },
            err: None,
        },
        // TODO allow None network
        // "nil network" => NetworkIdentTest {
        //   network: None,
        //   err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        // },
        AsserterTest {
            name: "invalid blockchain",
            payload: NetworkIdentifier {
                blockchain: Default::default(),
                network: "mainnet".into(),
                sub_network_identifier: Default::default(),
            },
            err: Some(NetworkError::NetworkIdentifierBlockchainMissing.into()),
        },
        AsserterTest {
            name: "invalid network",
            payload: NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: Default::default(),
                sub_network_identifier: Default::default(),
            },
            err: Some(NetworkError::NetworkIdentifierNetworkMissing.into()),
        },
        AsserterTest {
            name: "valid sub_network",
            payload: NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: "mainnet".into(),
                sub_network_identifier: Some(SubNetworkIdentifier {
                    network: "shard 1".into(),
                    metadata: Default::default(),
                }),
            },
            err: None,
        },
        AsserterTest {
            name: "invalid sub_network",
            payload: NetworkIdentifier {
                blockchain: "bitcoin".into(),
                network: "mainnet".into(),
                sub_network_identifier: Some(Default::default()),
            },
            err: Some(NetworkError::SubNetworkIdentifierInvalid.into()),
        },
    ];

    AsserterTest::non_asserter_tests(&tests, network_identifier);
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
            payload: Version {
                rosetta_version: rosetta_version.clone(),
                node_version: node_version.clone(),
                ..Default::default()
            },
            err: None,
        },
        AsserterTest {
            name: "valid version with middleware",
            payload: Version {
                rosetta_version: rosetta_version.clone(),
                node_version: node_version.clone(),
                middleware_version,
                ..Default::default()
            },
            err: None,
        },
        AsserterTest {
            name: "old RosettaVersion",
            payload: Version {
                rosetta_version: "1.2.0".to_string(),
                node_version: node_version.clone(),
                ..Default::default()
            },
            err: None,
        },
        // TODO allow None Version
        // "nil version" => ServerTest {
        //   request: None,
        //   err: Some(NetworkError::VersionIsNil.into()),
        // },
        AsserterTest {
            name: "invalid NodeVersion",
            payload: Version {
                rosetta_version: rosetta_version.clone(),
                node_version: String::new(),
                ..Default::default()
            },
            err: Some(NetworkError::VersionNodeVersionMissing.into()),
        },
        AsserterTest {
            name: "invalid MiddlewareVersion",
            payload: Version {
                rosetta_version,
                node_version,
                middleware_version: invalid_middleware_version,
                ..Default::default()
            },
            err: Some(NetworkError::VersionMiddlewareVersionMissing.into()),
        },
    ];

    AsserterTest::non_asserter_tests(&tests, version);
}

#[test]
fn test_allow() {
    let operation_statuses = vec![
        OperationStatus {
            status: "SUCCESS".to_string(),
            successful: true,
        },
        OperationStatus {
            status: "FAILURE".to_string(),
            successful: false,
        },
    ];
    let operation_types = vec!["PAYMENT".to_string()];
    let call_methods = Some(vec!["call".to_string()]);
    let balance_exemptions = Some(vec![BalanceExemption {
        sub_account_address: None,
        currency: Some(Currency {
            symbol: "BTC".to_string(),
            decimals: 8,
            metadata: Default::default(),
        }),
        exemption_type: Some(ExemptionType::Dynamic),
    }]);
    let neg_index = Some(-1);
    let index = Some(100);

    let tests = [
        AsserterTest {
            name: "valid Allow",
            payload: Allow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                ..Default::default()
            },
            err: None,
        },
        AsserterTest {
            name: "valid Allow with call methods and exemptions",
            payload: Allow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods: call_methods.clone(),
                balance_exemptions: balance_exemptions.clone(),
                historical_balance_lookup: true,
                timestamp_start_index: index,
                ..Default::default()
            },
            err: None,
        },
        AsserterTest {
            name: "valid Allow with exemptions and no historical",
            payload: Allow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods,
                balance_exemptions: balance_exemptions.clone(),
                ..Default::default()
            },
            err: Some(NetworkError::TimestampStartIndexInvalid.into()),
        },
        // TODO make timestamp start index an i64
        // "invalid timestamp start index" => ServerTest {
        //   request: Allow {
        //     operation_statuses: operation_statuses.clone(),
        //     operation_types: operation_types.clone(),
        //     timestamp_start_index: neg_index,
        //     ..Default::default()
        //   },
        // err: Some(NetworkError::TimestampStartIndexInvalid.into())
        // },
        // TODO allow None Allow
        // "nil Allow" => ServerTest {
        //   request: None,
        //   err: Some(NetworkError::AllowIsNil.into())
        // },
        AsserterTest {
            name: "no OperationStatuses",
            payload: Allow {
                operation_types: operation_types.clone(),
                ..Default::default()
            },
            err: Some(NetworkError::NoAllowedOperationStatuses.into()),
        },
        AsserterTest {
            name: "no successful OperationStatuses",
            payload: Allow {
                operation_statuses: vec![operation_statuses[1].clone()],
                operation_types: operation_types.clone(),
                ..Default::default()
            },
            err: Some(NetworkError::NoSuccessfulAllowedOperationStatuses.into()),
        },
        AsserterTest {
            name: "no OperationTypes",
            payload: Allow {
                operation_statuses: operation_statuses.clone(),
                ..Default::default()
            },
            err: Some(AsserterError::from(
                "no Allow.OperationTypes found".to_string(),
            )),
        },
        AsserterTest {
            name: "duplicate call methods",
            payload: Allow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods: Some(vec!["call".into(), "call".into()]),
                balance_exemptions,
                ..Default::default()
            },
            err: Some(AsserterError::from(
                "Allow.CallMethods contains a duplicate call".to_string(),
            )),
        },
        AsserterTest {
            name: "empty exemption",
            payload: Allow {
                operation_statuses: operation_statuses.clone(),
                operation_types: operation_types.clone(),
                call_methods: Some(vec!["call".into()]),
                balance_exemptions: Some(Vec::new()),
                ..Default::default()
            },
            err: Some(NetworkError::BalanceExemptionMissingSubject.into()),
        },
        AsserterTest {
            name: "empty exemption",
            payload: Allow {
                operation_statuses,
                operation_types,
                call_methods: Some(vec!["call".into()]),
                balance_exemptions: Some(Vec::new()),
                ..Default::default()
            },
            err: Some(NetworkError::NoAllowedOperationStatuses.into()),
        },
    ];

    AsserterTest::non_asserter_tests(&tests, allow);
}

#[test]
fn test_error() {
    let tests = [
        AsserterTest {
            name: "valid error",
            payload: MentatError {
                code: 12,
                message: "signature invalid".into(),
                ..Default::default()
            },
            err: None,
        },
        // TODO allow None Error
        // "nil error" => ServerTest {
        //   request: None,
        //   err: Some(ErrorError::IsNil.into()),
        // },
        // TODO change code to i32
        // "negative code" => ServerTest {
        //   request: MentatError {
        //     code: -1,
        //     message: "signature invalid".into(),
        //     ..Default::default()
        //   },
        //   err: Some(ErrorError::CodeIsNeg.into()),
        // },
        AsserterTest {
            name: "empty message",
            payload: MentatError {
                code: 0,
                message: String::new(),
                ..Default::default()
            },
            err: Some(ErrorError::MessageMissing.into()),
        },
    ];

    AsserterTest::non_asserter_tests(&tests, error);
}

#[test]
fn test_errors() {
    let tests = [
        AsserterTest {
            name: "valid errors",
            payload: vec![
                MentatError {
                    code: 0,
                    message: "error 1".into(),
                    ..Default::default()
                },
                MentatError {
                    code: 2,
                    message: "error 2".into(),
                    ..Default::default()
                },
            ],
            err: None,
        },
        AsserterTest {
            name: "details populated",
            payload: vec![
                MentatError {
                    code: 0,
                    message: "error 1".into(),
                    details: indexmap!(
                      "hello".to_string() => "goodbye".into()
                    ),
                    ..Default::default()
                },
                MentatError {
                    code: 1,
                    message: "error 2".into(),
                    ..Default::default()
                },
            ],
            err: Some(NetworkError::ErrorDetailsPopulated.into()),
        },
        AsserterTest {
            name: "duplicate error codes",
            payload: vec![
                MentatError {
                    code: 0,
                    message: "error 1".into(),
                    ..Default::default()
                },
                MentatError {
                    code: 0,
                    message: "error 2".into(),
                    ..Default::default()
                },
            ],
            err: Some(NetworkError::ErrorCodeUsedMultipleTimes.into()),
        },
    ];

    AsserterTest::non_asserter_tests(&tests, |data| errors(data.as_slice()));
}

#[test]
fn test_network_list_response() {
    let network_1 = NetworkIdentifier {
        blockchain: "blockchain 1".into(),
        network: "network 1".into(),
        sub_network_identifier: None,
    };
    let network_1_sub = NetworkIdentifier {
        sub_network_identifier: Some(SubNetworkIdentifier {
            network: "subnetwork".into(),
            metadata: Default::default(),
        }),
        ..network_1.clone()
    };
    let network_2 = NetworkIdentifier {
        blockchain: "blockchain 2".into(),
        network: "network 2".into(),
        sub_network_identifier: None,
    };
    let network_3 = NetworkIdentifier {
        network: "network 2".into(),
        ..Default::default()
    };

    let tests = [
        AsserterTest {
            name: "valid network list",
            payload: NetworkListResponse {
                network_identifiers: vec![network_1, network_1_sub.clone(), network_2],
            },
            err: None,
        },
        // TODO allow None NetworkListResponse
        // "nil network list" => ServerTest {
        //   request: None,
        //   err: Some(NetworkError::NetworkListResponseIsNil.into()),
        // },
        AsserterTest {
            name: "network list duplicate",
            payload: NetworkListResponse {
                network_identifiers: vec![network_1_sub.clone(), network_1_sub.clone()],
            },
            err: Some(NetworkError::NetworkListResponseNetworksContainsDuplicates.into()),
        },
        AsserterTest {
            name: "network list duplicate",
            payload: NetworkListResponse {
                network_identifiers: vec![network_1_sub, network_3],
            },
            err: Some(NetworkError::NetworkListResponseNetworksContainsDuplicates.into()),
        },
    ];

    AsserterTest::non_asserter_tests(&tests, network_list_response);
}
