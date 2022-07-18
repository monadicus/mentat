use indexmap::{indexmap, IndexMap};

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

struct ErrMapTest {
    err: MentatError,
    expected_err: Option<ErrorError>,
}

#[test]
fn test_error_map() {
    let tests: IndexMap<&str, ErrMapTest> = indexmap!(
      "matching error" => ErrMapTest {
        err: MentatError {
          status_code: 0,
          code: 10,
          message: "error 10".to_string(),
          description: None,
          retriable: true,
          details: indexmap!(
            "hello".to_string() => "goodbye".into()
          )
        },
        expected_err: None
      },
      "empty description" => ErrMapTest {
        err: MentatError {
          status_code: 0,
          code: 10,
          message: "error 10".to_string(),
          description: Some(String::new()),
          retriable: true,
          details: indexmap!(
            "hello".to_string() => "goodbye".into()
          )
        },
        expected_err: Some(ErrorError::DescriptionEmpty),
      },
      // TODO make code an i32
      // "negative error" => ErrMapTest {
      //   err: MentatError {
      //     status_code: 0,
      //     code: -1,
      //     message: "error 10".to_string(),
      //     description: None,
      //     retriable: true,
      //     details: indexmap!(
      //       "hello".to_string() => "goodbye".into()
      //     )
      //   },
      //   expected_err: Some(ErrorError::CodeIsNeg),
      // },
      "retriable error" => ErrMapTest {
        err: MentatError {
          status_code: 0,
          code: 10,
          message: "error 10".to_string(),
          description: None,
          retriable: false,
          details: indexmap!(
            "hello".to_string() => "goodbye".into()
          )
        },
        expected_err: Some(ErrorError::RetriableMismatch),
      },
      "code mismatch" => ErrMapTest {
        err: MentatError {
          status_code: 0,
          code: 20,
          message: "error 20".to_string(),
          description: None,
          retriable: false,
          details: indexmap!(
            "hello".to_string() => "goodbye".into()
          )
        },
        expected_err: Some(ErrorError::UnexpectedCode),
      },
      "code mismatch" => ErrMapTest {
        err: MentatError {
          status_code: 0,
          code: 10,
          message: "error 11".to_string(),
          description: None,
          retriable: true,
          details: indexmap!(
            "hello".to_string() => "goodbye".into()
          )
        },
        expected_err: Some(ErrorError::MessageMismatch),
      },
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let asserter = Asserter::new_client_with_responses(
            NetworkIdentifier {
                blockchain: "hello".to_string(),
                network: "world".to_string(),
                sub_network_identifier: None,
            },
            NetworkStatusResponse {
                current_block_identifier: BlockIdentifier {
                    index: 100,
                    hash: "block 100".to_string(),
                },
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                genesis_block_identifier: BlockIdentifier {
                    index: 0,
                    hash: "block 0".to_string(),
                },
                oldest_block_identifier: None,
                sync_status: None,
                peers: vec![Peer {
                    peer_id: "peer 1".to_string(),
                    metadata: Default::default(),
                }],
            },
            NetworkOptionsResponse {
                version: Version {
                    rosetta_version: "1.4.0".to_string(),
                    node_version: "1.0".to_string(),
                    middleware_version: None,
                    metadata: Default::default(),
                },
                allow: Allow {
                    errors: vec![
                        MentatError {
                            status_code: 0,
                            code: 10,
                            message: "error 10".to_string(),
                            description: None,
                            retriable: true,
                            details: Default::default(),
                        },
                        MentatError {
                            status_code: 0,
                            code: 1,
                            message: "error 1".to_string(),
                            description: None,
                            retriable: false,
                            details: Default::default(),
                        },
                    ],
                    operation_statuses: vec![
                        OperationStatus {
                            status: "SUCCESS".to_string(),
                            successful: true,
                        },
                        OperationStatus {
                            status: "FAILURE".to_string(),
                            successful: false,
                        },
                    ],
                    operation_types: vec!["PAYMENT".to_string()],
                    ..Default::default()
                },
            },
            // TODO make this optional???
            Default::default(),
        );
        assert!(asserter.is_err());

        // let resp = asserter.unwrap().error(&test.err);
    });
}
