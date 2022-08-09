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

#[test]
fn test_group_operations() {
    let tests = vec![
        TestCase {
            name: "no ops",
            payload: Transaction::default(),
            criteria: Vec::new(),
        },
        TestCase {
            name: "unrelated ops",
            payload: Transaction {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "op 0".into(),
                        amount: Some(Amount {
                            currency: Currency {
                                symbol: "BTC".into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "op 1".into(),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        type_: "op 2".into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            criteria: vec![
                OperationGroup {
                    type_: "op 0".into(),
                    operations: vec![Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "op 0".into(),
                        amount: Some(Amount {
                            currency: Currency {
                                symbol: "BTC".into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    currencies: vec![Currency {
                        symbol: "BTC".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                OperationGroup {
                    type_: "op 1".into(),
                    nul_amount_present: true,
                    operations: vec![Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "op 1".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                OperationGroup {
                    type_: "op 2".into(),
                    nul_amount_present: true,
                    operations: vec![Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        type_: "op 2".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
        },
        TestCase {
            name: "related ops",
            payload: Transaction {
                operations: vec![
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        type_: "type 0".into(),
                        amount: Some(Amount {
                            currency: Currency {
                                symbol: "BTC".into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "type 1".into(),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        type_: "type 2".into(),
                        amount: Some(Amount {
                            currency: Currency {
                                symbol: "BTC".into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 3,
                            network_index: None,
                        },
                        related_operations: vec![OperationIdentifier {
                            index: 2,
                            network_index: None,
                        }],
                        type_: "type 2".into(),
                        amount: Some(Amount {
                            currency: Currency {
                                symbol: "ETH".into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 4,
                            network_index: None,
                        },
                        related_operations: vec![OperationIdentifier {
                            index: 2,
                            network_index: None,
                        }],
                        type_: "type 4".into(),
                        ..Default::default()
                    },
                    Operation {
                        operation_identifier: OperationIdentifier {
                            index: 5,
                            network_index: None,
                        },
                        related_operations: vec![OperationIdentifier {
                            index: 0,
                            network_index: None,
                        }],
                        type_: "type 0".into(),
                        amount: Some(Amount {
                            currency: Currency {
                                symbol: "BTC".into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            criteria: vec![
                OperationGroup {
                    type_: "type 0".into(),
                    operations: vec![
                        Operation {
                            operation_identifier: OperationIdentifier {
                                index: 0,
                                network_index: None,
                            },
                            type_: "type 0".into(),
                            amount: Some(Amount {
                                currency: Currency {
                                    symbol: "BTC".into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        Operation {
                            operation_identifier: OperationIdentifier {
                                index: 5,
                                network_index: None,
                            },
                            related_operations: vec![OperationIdentifier {
                                index: 0,
                                network_index: None,
                            }],
                            type_: "type 0".into(),
                            amount: Some(Amount {
                                currency: Currency {
                                    symbol: "BTC".into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                    ],
                    currencies: vec![Currency {
                        symbol: "BTC".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                OperationGroup {
                    type_: "type 1".into(),
                    nul_amount_present: true,
                    operations: vec![Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        type_: "type 1".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                OperationGroup {
                    type_: "".into(),
                    nul_amount_present: true,
                    currencies: vec![
                        Currency {
                            symbol: "BTC".into(),
                            ..Default::default()
                        },
                        Currency {
                            symbol: "ETH".into(),
                            ..Default::default()
                        },
                    ],
                    operations: vec![
                        Operation {
                            operation_identifier: OperationIdentifier {
                                index: 2,
                                network_index: None,
                            },
                            type_: "type 2".into(),
                            amount: Some(Amount {
                                currency: Currency {
                                    symbol: "BTC".into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        Operation {
                            operation_identifier: OperationIdentifier {
                                index: 3,
                                network_index: None,
                            },
                            related_operations: vec![OperationIdentifier {
                                index: 2,
                                network_index: None,
                            }],
                            type_: "type 2".into(),
                            amount: Some(Amount {
                                currency: Currency {
                                    symbol: "ETH".into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        Operation {
                            operation_identifier: OperationIdentifier {
                                index: 4,
                                network_index: None,
                            },
                            related_operations: vec![OperationIdentifier {
                                index: 2,
                                network_index: None,
                            }],
                            type_: "type 4".into(),
                            ..Default::default()
                        },
                    ],
                },
            ],
        },
    ];

    TestCase::run_output_match(tests, |t| group_operations(&t));
}
