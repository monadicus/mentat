use indexmap::{indexmap, IndexMap};

use crate::{
    asserter::{
        errors::{AsserterError, NetworkError},
        network::{network_identifier, version},
    },
    types::{NetworkIdentifier, SubNetworkIdentifier, Version},
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
          blockchain: "bitcoin".to_string(),
          network: "mainnet".to_string(),
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
          blockchain: String::new(),
          network: "mainnet".to_string(),
          sub_network_identifier: Default::default()
        },
        err: Some(NetworkError::NetworkIdentifierBlockchainMissing.into()),
      },
      "invalid network" => NetworkIdentTest {
        network: NetworkIdentifier {
          blockchain: "bitcoin".to_string(),
          network: String::new(),
          sub_network_identifier: Default::default()
        },
        err: Some(NetworkError::NetworkIdentifierNetworkMissing.into()),
      },
      "valid sub_network" => NetworkIdentTest {
        network: NetworkIdentifier {
          blockchain: "bitcoin".to_string(),
          network: "mainnet".to_string(),
          sub_network_identifier: Some(SubNetworkIdentifier { network: "shard 1".to_string(), metadata: Default::default() })
        },
        err: None,
      },
      "invalid sub_network" => NetworkIdentTest {
        network: NetworkIdentifier {
          blockchain: "bitcoin".to_string(),
          network: "mainnet".to_string(),
          sub_network_identifier: Some(Default::default())
        },
        err: Some(NetworkError::SubNetworkIdentifierInvalid.into()),
      },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let res = network_identifier(&test.network);
        if let Err(err) = res {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
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
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}
