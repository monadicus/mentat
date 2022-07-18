use indexmap::{indexmap, IndexMap};

use crate::{
    asserter::{
        errors::{AsserterError, NetworkError},
        network::{allow, network_identifier, version},
    },
    types::{
        Allow, BalanceExemption, Currency, ExemptionType, MentatError, NetworkIdentifier,
        OperationStatus, SubNetworkIdentifier, Version,
    },
};

struct NetworkIdentTest {
    network: NetworkIdentifier,
    err: Option<AsserterError>,
}

#[test]
fn test_network_identifier() {
    let tests: IndexMap<&str, NetworkIdentTest> = indexmap!(
      "valid network" => NetworkIdentTest {
        network: NetworkIdentifier {
          blockchain: "bitcoin".into(),
          network: "mainnet".into(),
          sub_network_identifier: Default::default()
        },
        err: None,
      },
      // TODO allow None network
      // "nil network" => NetworkIdentTest {
      //   network: None,
      //   err: Some(NetworkError::NetworkIdentifierIsNil.into()),
      // },
      "invalid blockchain" => NetworkIdentTest {
        network: NetworkIdentifier {
          blockchain: Default::default(),
          network: "mainnet".into(),
          sub_network_identifier: Default::default()
        },
        err: Some(NetworkError::NetworkIdentifierBlockchainMissing.into()),
      },
      "invalid network" => NetworkIdentTest {
        network: NetworkIdentifier {
          blockchain: "bitcoin".into(),
          network: Default::default(),
          sub_network_identifier: Default::default()
        },
        err: Some(NetworkError::NetworkIdentifierNetworkMissing.into()),
      },
      "valid sub_network" => NetworkIdentTest {
        network: NetworkIdentifier {
          blockchain: "bitcoin".into(),
          network: "mainnet".into(),
          sub_network_identifier: Some(SubNetworkIdentifier { network: "shard 1".into(), metadata: Default::default() })
        },
        err: None,
      },
      "invalid sub_network" => NetworkIdentTest {
        network: NetworkIdentifier {
          blockchain: "bitcoin".into(),
          network: "mainnet".into(),
          sub_network_identifier: Some(Default::default())
        },
        err: Some(NetworkError::SubNetworkIdentifierInvalid.into()),
      },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let res = network_identifier(&test.network);
        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    });
}

struct VerTest {
    ver: Version,
    err: Option<AsserterError>,
}

#[test]
fn test_version() {
    let middleware_version = Some("1.2".to_string());
    let invalid_middleware_version = Some(String::new());
    let rosetta_version = "1.4.0".to_string();
    let node_version = "1.0".to_string();

    let tests: IndexMap<&str, VerTest> = indexmap!(
      "valid version" => VerTest {
        ver: Version {
          rosetta_version: rosetta_version.clone(),
          node_version: node_version.clone(),
          ..Default::default()
        },
        err: None
      },
      "valid version with middleware" => VerTest {
        ver: Version {
          rosetta_version: rosetta_version.clone(),
          node_version: node_version.clone(),
          middleware_version,
          ..Default::default()
        },
        err: None
      },
      "old RosettaVersion" => VerTest {
        ver: Version {
          rosetta_version: "1.2.0".to_string(),
          node_version: node_version.clone(),
          ..Default::default()
        },
        err: None
      },
      // TODO allow None Version
      // "nil version" => VerTest {
      //   ver: None,
      //   err: Some(NetworkError::VersionIsNil.into()),
      // },
      "invalid NodeVersion" => VerTest {
        ver: Version {
          rosetta_version: rosetta_version.clone(),
          node_version: String::new(),
          ..Default::default()
        },
        err: Some(NetworkError::VersionNodeVersionMissing.into()),
      },
      "invalid MiddlewareVersion" => VerTest {
        ver: Version {
          rosetta_version,
          node_version,
          middleware_version: invalid_middleware_version,
          ..Default::default()
        },
        err: Some(NetworkError::VersionMiddlewareVersionMissing.into()),
      },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let res = version(&test.ver);
        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    });
}

struct AllowTest {
    allow: Allow,
    err: Option<AsserterError>,
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

    let tests: IndexMap<&str, AllowTest> = indexmap!(
      "valid Allow" => AllowTest {
        allow: Allow {
          operation_statuses: operation_statuses.clone(),
          operation_types: operation_types.clone(),
          ..Default::default()
        },
        err: None
      },
      "valid Allow with call methods and exemptions" => AllowTest {
        allow: Allow {
          operation_statuses: operation_statuses.clone(),
          operation_types: operation_types.clone(),
          call_methods: call_methods.clone(),
          balance_exemptions: balance_exemptions.clone(),
          historical_balance_lookup: true,
          timestamp_start_index: index,
          ..Default::default()
        },
        err: None
      },
      "valid Allow with exemptions and no historical" => AllowTest {
        allow: Allow {
          operation_statuses: operation_statuses.clone(),
          operation_types: operation_types.clone(),
          call_methods,
          balance_exemptions: balance_exemptions.clone(),
          ..Default::default()
        },
        err: Some(NetworkError::TimestampStartIndexInvalid.into())
      },
      // TODO make timestamp start index an i64
      // "invalid timestamp start index" => AllowTest {
      //   allow: Allow {
      //     operation_statuses: operation_statuses.clone(),
      //     operation_types: operation_types.clone(),
      //     timestamp_start_index: neg_index,
      //     ..Default::default()
      //   },
      // err: Some(NetworkError::TimestampStartIndexInvalid.into())
      // },
      // TODO allow None Allow
      // "nil Allow" => AllowTest {
      //   allow: None,
      //   err: Some(NetworkError::AllowIsNil.into())
      // },
      "no OperationStatuses" => AllowTest {
        allow: Allow {
          operation_types: operation_types.clone(),
          ..Default::default()
        },
        err: Some(NetworkError::NoAllowedOperationStatuses.into())
      },
      "no successful OperationStatuses" => AllowTest {
        allow: Allow {
          operation_statuses: vec![operation_statuses[1].clone()],
          operation_types: operation_types.clone(),
          ..Default::default()
        },
        err: Some(NetworkError::NoSuccessfulAllowedOperationStatuses.into())
      },
      "no OperationTypes" => AllowTest {
        allow: Allow {
          operation_statuses: operation_statuses.clone(),
          ..Default::default()
        },
        err: Some(AsserterError::from("no Allow.OperationTypes found".to_string()))
      },
      "duplicate call methods" => AllowTest {
        allow: Allow {
          operation_statuses: operation_statuses.clone(),
          operation_types: operation_types.clone(),
          call_methods: Some(vec!["call".into(), "call".into()]),
          balance_exemptions,
          ..Default::default()
        },
        err: Some(AsserterError::from("Allow.CallMethods contains a duplicate call".to_string()))
      },
      "empty exemption"=> AllowTest {
        allow: Allow {
          operation_statuses: operation_statuses.clone(),
          operation_types: operation_types.clone(),
          call_methods: Some(vec!["call".into()]),
          balance_exemptions: Some(Vec::new()),
          ..Default::default()
        },
        err: Some(NetworkError::BalanceExemptionMissingSubject.into())
      },
      "empty exemption"=> AllowTest {
        allow: Allow {
          operation_statuses,
          operation_types,
          call_methods: Some(vec!["call".into()]),
          balance_exemptions: Some(Vec::new()),
          ..Default::default()
        },
        err: Some(NetworkError::NoAllowedOperationStatuses.into())
      },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let res = allow(&test.allow);
        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    });
}

struct ErrorTest {
    rosetta_err: MentatError,
    err: Option<AsserterError>,
}

#[test]
fn test_error() {}
