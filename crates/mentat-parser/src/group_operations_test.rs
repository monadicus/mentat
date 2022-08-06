use indexmap::indexmap;

use super::*;

#[test]
fn test_sort_operation_groups() {
    let m: IndexMap<usize, OperationGroup> = indexmap!(
      2 => OperationGroup {
        operations: vec![
          Operation {
            operation_identifier: OperationIdentifier { index: 2, network_index: None },
            ..Default::default()
          }
        ],
        ..Default::default()
      },
      4 => OperationGroup {
        operations: vec![
          Operation {
            operation_identifier: OperationIdentifier { index: 4, network_index: None },
            ..Default::default()
          }
        ],
        ..Default::default()
      },
      0 => OperationGroup {
        operations: vec![
          Operation {
            operation_identifier: OperationIdentifier { index: 1, network_index: None },
            related_operations: vec![
              OperationIdentifier {index: 0, network_index: None },
            ],
            ..Default::default()
          },
          Operation {
            operation_identifier: OperationIdentifier { index: 3, network_index: None },
            related_operations: vec![
              OperationIdentifier {index: 1, network_index: None },
            ],
            ..Default::default()
          },
          Operation {
            operation_identifier: OperationIdentifier { index: 0, network_index: None },
            ..Default::default()
          }
        ],
        ..Default::default()
      },
      5 => OperationGroup {
        operations: vec![
          Operation {
            operation_identifier: OperationIdentifier { index: 5, network_index: None },
            ..Default::default()
          }
        ],
        ..Default::default()
      },
    );

    let sorted_groups = sort_operation_groups(6, m);
    assert_eq!(
        vec![
            OperationGroup {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None
                        },
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None
                        },
                        related_operations: vec![OperationIdentifier {
                            index: 0,
                            network_index: None
                        },],
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 3,
                            network_index: None
                        },
                        related_operations: vec![OperationIdentifier {
                            index: 1,
                            network_index: None
                        },],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            OperationGroup {
                operations: vec![Operation {
                    operation_identifier: OperationIdentifier {
                        index: 2,
                        network_index: None
                    },
                    ..Default::default()
                },],
                ..Default::default()
            },
            OperationGroup {
                operations: vec![Operation {
                    operation_identifier: OperationIdentifier {
                        index: 4,
                        network_index: None
                    },
                    ..Default::default()
                },],
                ..Default::default()
            },
            OperationGroup {
                operations: vec![Operation {
                    operation_identifier: OperationIdentifier {
                        index: 5,
                        network_index: None
                    },
                    ..Default::default()
                },],
                ..Default::default()
            }
        ],
        sorted_groups
    );
}

