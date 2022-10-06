use indexmap::indexmap;
use num_bigint_dig::BigInt;
use serde_json::json;

use super::*;

#[derive(Default, PartialEq, Eq)]
struct MatchOperationsTest {
    operations: Vec<Option<Operation>>,
    descriptions: Descriptions,
}

#[test]
fn test_match_operations() {
    let tests =
        vec![
            TestCase {
                name: "simple transfer (with extra op)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(-100))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(100))],
                    }),
                ]),
            },
            TestCase {
                name: "simple transfer (with OppositeOrZeroAmounts) and opposite value amounts",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE_OR_ZERO,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE_OR_ZERO,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(-100))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(100))],
                    }),
                ]),
            },
            TestCase {
                name: "simple transfer (with OppositeOrZeroAmounts) and 0-value amounts",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "0".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "0".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_or_zero_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::ANY,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::ANY,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "0".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(0))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "0".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(0))],
                    }),
                ]),
            },
            TestCase {
                name: "simple transfer (with too many opposite amounts)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1, 2]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple transfer (with missing account error)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        equal_addresses: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: false,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple transfer (check type)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            type_: "output".into(),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            type_: "input".into(),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                type_: "input".into(),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                type_: "output".into(),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            type_: "input".into(),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(-100))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            type_: "output".into(),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(100))],
                    }),
                ]),
            },
            TestCase {
                name: "simple transfer (reject extra op)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        err_unmatched: true,
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple transfer (with unequal amounts)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        equal_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple transfer (with equal amounts)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        equal_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(100))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(100))],
                    }),
                ]),
            },
            TestCase {
                name: "simple transfer (with coin action)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                currency: Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            coin_change: Some(CoinChange {
                                coin_action: CoinAction::CoinSpent,
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                currency: Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    currency: Some(Currency {
                                        symbol: "ETH".into(),
                                        decimals: 18,
                                        ..Default::default()
                                    }),
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    currency: Some(Currency {
                                        symbol: "BTC".into(),
                                        decimals: 8,
                                        ..Default::default()
                                    }),
                                }),
                                coin_action: CoinAction::CoinSpent,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                currency: Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(-100))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                currency: Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            coin_change: Some(CoinChange {
                                coin_action: CoinAction::CoinSpent,
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(100))],
                    }),
                ]),
            },
            TestCase {
                name: "simple transfer (missing coin action)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                currency: Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                currency: Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    currency: Some(Currency {
                                        symbol: "ETH".into(),
                                        decimals: 18,
                                        ..Default::default()
                                    }),
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    currency: Some(Currency {
                                        symbol: "BTC".into(),
                                        decimals: 8,
                                        ..Default::default()
                                    }),
                                }),
                                coin_action: CoinAction::CoinSpent,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple transfer (incorrect coin action)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                currency: Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            coin_change: Some(CoinChange {
                                coin_action: CoinAction::CoinCreated,
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                currency: Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    currency: Some(Currency {
                                        symbol: "ETH".into(),
                                        decimals: 18,
                                        ..Default::default()
                                    }),
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    currency: Some(Currency {
                                        symbol: "BTC".into(),
                                        decimals: 8,
                                        ..Default::default()
                                    }),
                                }),
                                coin_action: CoinAction::CoinSpent,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple transfer (with currency)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                currency: Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                currency: Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    currency: Some(Currency {
                                        symbol: "ETH".into(),
                                        decimals: 18,
                                        ..Default::default()
                                    }),
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    currency: Some(Currency {
                                        symbol: "BTC".into(),
                                        decimals: 8,
                                        ..Default::default()
                                    }),
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                currency: Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(-100))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                currency: Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(100))],
                    }),
                ]),
            },
            TestCase {
                name: "simple transfer (with missing currency)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                currency: Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                currency: Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    currency: Some(Currency {
                                        symbol: "ETH".into(),
                                        decimals: 18,
                                        ..Default::default()
                                    }),
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    currency: Some(Currency {
                                        symbol: "BTC".into(),
                                        decimals: 8,
                                        ..Default::default()
                                    }),
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple transfer (with sender metadata) and non-equal addresses",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub".into(),
                                    metadata: indexmap! {
                                        "validator".to_string() => json!("10"),
                                    },
                                }),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        equal_addresses: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub".into(),
                                    sub_account_metadata_keys: vec![Some(
                                        MetadataDescription::new::<String>("validator".into()),
                                    )],
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple transfer (with sender metadata)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub".into(),
                                    metadata: indexmap! {
                                        "validator".to_string() => json!("10"),
                                    },
                                }),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        equal_addresses: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub".into(),
                                    sub_account_metadata_keys: vec![Some(
                                        MetadataDescription::new::<String>("validator".into()),
                                    )],
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub".into(),
                                    metadata: indexmap! {
                                        "validator".to_string() => json!("10"),
                                    },
                                }),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(-100))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(100))],
                    }),
                ]),
            },
            TestCase {
                name: "simple transfer (with missing sender address metadata)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub".into(),
                                    sub_account_metadata_keys: vec![Some(
                                        MetadataDescription::new::<String>("validator".into()),
                                    )],
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "nil amount ops",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 1".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            // allowed because no amount requirement provided
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 1".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(100))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 1".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![None],
                    }),
                ]),
            },
            TestCase {
                name: "nil amount ops (force false amount)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 1".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            amount: Some(<_>::default()),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: false,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 1".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "nil amount ops (only require metadata keys)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 1".into(),
                                    metadata: indexmap! {
                                        "validator".into() => json!(-1000),
                                    },
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_metadata_keys: vec![Some(
                                        MetadataDescription::new::<i32>("validator".into()),
                                    )],
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![None],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 1".into(),
                                    metadata: indexmap! {
                                        "validator".into() => json!(-1000),
                                    },
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![None],
                    }),
                ]),
            },
            TestCase {
                name: "nil amount ops (sub account address mismatch)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 3".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 1".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "sub account optional - when sub-account exist",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_optional: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_optional: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 1".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![None],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![None],
                    }),
                ]),
            },
            TestCase {
                name: "sub account optional - when sub-account address is different",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_optional: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 3".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    sub_account_optional: true,
                                    sub_account_exists: true,
                                    sub_account_address: "sub 1".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "nil descriptions",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 3".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: <_>::default(),
                },
                criteria: None,
            },
            TestCase {
                name: "2 empty descriptions",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 3".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![Some(<_>::default()), Some(<_>::default())],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 3".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![None],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                sub_account: Some(SubAccountIdentifier {
                                    address: "sub 2".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        })],
                        amounts: vec![None],
                    }),
                ]),
            },
            TestCase {
                name: "empty operations",
                payload: MatchOperationsTest {
                    operations: vec![],
                    descriptions: Descriptions {
                        operation_descriptions: vec![Some(<_>::default()), Some(<_>::default())],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple repeated op",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..Default::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..Default::default()
                            }),
                            allow_repeats: true,
                            ..Default::default()
                        })],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![Some(Match {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    amounts: vec![Some(BigInt::from(200)), Some(BigInt::from(100))],
                })]),
            },
            TestCase {
                name: "simple repeated op (no extra ops allowed)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..Default::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..Default::default()
                            }),
                            allow_repeats: true,
                            ..Default::default()
                        })],
                        err_unmatched: true,
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple repeated op (with invalid comparison indexes)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..Default::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..Default::default()
                            }),
                            allow_repeats: true,
                            ..Default::default()
                        })],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "simple repeated op (with overlapping, repeated descriptions)",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                allow_repeats: true,
                                ..Default::default()
                            }),
                            // will never be possible to meet this description
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                allow_repeats: true,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "complex repeated op",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            type_: "output".into(),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr3".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            type_: "output".into(),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-200".into(),
                                ..Default::default()
                            }),
                            type_: "input".into(),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr4".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-200".into(),
                                ..Default::default()
                            }),
                            type_: "input".into(),
                            ..Default::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr5".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-1000".into(),
                                ..Default::default()
                            }),
                            type_: "runoff".into(),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                allow_repeats: true,
                                type_: "output".into(),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                allow_repeats: true,
                                type_: "input".into(),
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                allow_repeats: true,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![
                            Some(Operation {
                                account: Some(AccountIdentifier {
                                    address: "addr2".into(),
                                    ..Default::default()
                                }),
                                amount: Some(Amount {
                                    value: "200".into(),
                                    ..Default::default()
                                }),
                                type_: "output".into(),
                                ..Default::default()
                            }),
                            Some(Operation {
                                account: Some(AccountIdentifier {
                                    address: "addr3".into(),
                                    ..Default::default()
                                }),
                                amount: Some(Amount {
                                    value: "200".into(),
                                    ..Default::default()
                                }),
                                type_: "output".into(),
                                ..Default::default()
                            }),
                        ],
                        amounts: vec![Some(BigInt::from(200)), Some(BigInt::from(200))],
                    }),
                    Some(Match {
                        operations: vec![
                            Some(Operation {
                                account: Some(AccountIdentifier {
                                    address: "addr1".into(),
                                    ..Default::default()
                                }),
                                amount: Some(Amount {
                                    value: "-200".into(),
                                    ..Default::default()
                                }),
                                type_: "input".into(),
                                ..Default::default()
                            }),
                            Some(Operation {
                                account: Some(AccountIdentifier {
                                    address: "addr4".into(),
                                    ..Default::default()
                                }),
                                amount: Some(Amount {
                                    value: "-200".into(),
                                    ..Default::default()
                                }),
                                type_: "input".into(),
                                ..Default::default()
                            }),
                        ],
                        amounts: vec![Some(BigInt::from(-200)), Some(BigInt::from(-200))],
                    }),
                    Some(Match {
                        operations: vec![Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr5".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "-1000".into(),
                                ..Default::default()
                            }),
                            type_: "runoff".into(),
                            ..Default::default()
                        })],
                        amounts: vec![Some(BigInt::from(-1000))],
                    }),
                ]),
            },
            TestCase {
                name: "optional description not met",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                allow_repeats: true,
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                optional: true,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: Some(vec![
                    Some(Match {
                        operations: vec![
                            Some(Operation {
                                account: Some(AccountIdentifier {
                                    address: "addr2".into(),
                                    ..Default::default()
                                }),
                                amount: Some(Amount {
                                    value: "200".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            Some(Operation {
                                account: Some(AccountIdentifier {
                                    address: "addr1".into(),
                                    ..Default::default()
                                }),
                                amount: Some(Amount {
                                    value: "100".into(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                        ],
                        amounts: vec![Some(BigInt::from(200)), Some(BigInt::from(100))],
                    }),
                    None,
                ]),
            },
            TestCase {
                name: "optional description equal amounts not found",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        equal_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                allow_repeats: true,
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                optional: true,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
            TestCase {
                name: "optional description opposite amounts not found",
                payload: MatchOperationsTest {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        // extra op ignored
                        Some(<_>::default()),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..Default::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                    ],
                    descriptions: Descriptions {
                        opposite_amounts: vec![vec![0, 1]],
                        operation_descriptions: vec![
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::POSITIVE,
                                    ..Default::default()
                                }),
                                allow_repeats: true,
                                ..Default::default()
                            }),
                            Some(OperationDescription {
                                account: Some(AccountDescription {
                                    exists: true,
                                    ..Default::default()
                                }),
                                amount: Some(AmountDescription {
                                    exists: true,
                                    sign: AmountSign::NEGATIVE,
                                    ..Default::default()
                                }),
                                optional: true,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
                criteria: None,
            },
        ];

    TestCase::run_ok_match(tests, |payload| {
        match_operations(payload.descriptions, payload.operations)
    });
}

#[derive(Debug, Default, PartialEq, Eq)]
struct MatchTestResult {
    op: Option<Operation>,
    amt: Option<BigInt>,
}

impl From<(Option<&Operation>, Option<&BigInt>)> for MatchTestResult {
    fn from((op, amt): (Option<&Operation>, Option<&BigInt>)) -> Self {
        Self {
            op: op.cloned(),
            amt: amt.cloned(),
        }
    }
}

#[test]
fn test_match() {
    let tests = vec![
        TestCase {
            name: "nil match",
            payload: None,
            criteria: MatchTestResult::default(),
        },
        TestCase {
            name: "empty match",
            payload: Some(Match::default()),
            criteria: MatchTestResult::default(),
        },
        TestCase {
            name: "single op match",
            payload: Some(Match {
                operations: vec![Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                })],
                amounts: vec![Some(BigInt::from(100))],
            }),
            criteria: MatchTestResult {
                op: Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                }),
                amt: Some(BigInt::from(100)),
            },
        },
        TestCase {
            name: "multi-op match",
            payload: Some(Match {
                operations: vec![
                    Some(Operation {
                        operation_identifier: OperationIdentifier {
                            index: 1,
                            network_index: None,
                        },
                        ..Default::default()
                    }),
                    Some(Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        ..Default::default()
                    }),
                ],
                amounts: vec![Some(BigInt::from(100)), Some(BigInt::from(200))],
            }),
            criteria: MatchTestResult {
                op: Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                }),
                amt: Some(BigInt::from(100)),
            },
        },
        TestCase {
            name: "single op match with nil amount",
            payload: Some(Match {
                operations: vec![Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                })],
                amounts: vec![None],
            }),
            criteria: MatchTestResult {
                op: Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..Default::default()
                }),
                amt: None,
            },
        },
    ];

    TestCase::run_output_match(tests, |test| Match::first(test.as_ref()).into());
}
