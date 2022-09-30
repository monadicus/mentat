use std::any::TypeId;

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
    let tests = vec![
        TestCase {
            name: "simple transfer (with extra op)",
            payload: MatchOperationsTest {
                operations: vec![
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![Some(BigInt::from(-100))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE_OR_ZERO,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE_OR_ZERO,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![Some(BigInt::from(-100))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "0".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "0".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_or_zero_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::ANY,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::ANY,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "0".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![Some(BigInt::from(0))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "0".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1, 2]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    equal_addresses: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: false,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        type_: "output".into(),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        type_: "input".into(),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            type_: "input".into(),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            type_: "output".into(),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        type_: "input".into(),
                        ..<_>::default()
                    })],
                    amounts: vec![Some(BigInt::from(-100))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        type_: "output".into(),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    err_unmatched: true,
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    equal_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    equal_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![Some(BigInt::from(100))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        coin_change: Some(CoinChange {
                            coin_action: CoinAction::CoinSpent,
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            currency: Currency {
                                symbol: "ETH".into(),
                                decimals: 18,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                currency: Some(Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..<_>::default()
                                }),
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                currency: Some(Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..<_>::default()
                                }),
                            }),
                            coin_action: CoinAction::CoinSpent,
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            currency: Currency {
                                symbol: "ETH".into(),
                                decimals: 18,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![Some(BigInt::from(-100))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        coin_change: Some(CoinChange {
                            coin_action: CoinAction::CoinSpent,
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            currency: Currency {
                                symbol: "ETH".into(),
                                decimals: 18,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                currency: Some(Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..<_>::default()
                                }),
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                currency: Some(Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..<_>::default()
                                }),
                            }),
                            coin_action: CoinAction::CoinSpent,
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        coin_change: Some(CoinChange {
                            coin_action: CoinAction::CoinCreated,
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            currency: Currency {
                                symbol: "ETH".into(),
                                decimals: 18,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                currency: Some(Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..<_>::default()
                                }),
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                currency: Some(Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..<_>::default()
                                }),
                            }),
                            coin_action: CoinAction::CoinSpent,
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            currency: Currency {
                                symbol: "ETH".into(),
                                decimals: 18,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                currency: Some(Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..<_>::default()
                                }),
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                currency: Some(Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..<_>::default()
                                }),
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            currency: Currency {
                                symbol: "ETH".into(),
                                decimals: 18,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![Some(BigInt::from(-100))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            currency: Currency {
                                symbol: "BTC".into(),
                                decimals: 8,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            currency: Currency {
                                symbol: "ETH".into(),
                                decimals: 18,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            currency: Currency {
                                symbol: "ETH".into(),
                                decimals: 18,
                                ..<_>::default()
                            },
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                currency: Some(Currency {
                                    symbol: "ETH".into(),
                                    decimals: 18,
                                    ..<_>::default()
                                }),
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                currency: Some(Currency {
                                    symbol: "BTC".into(),
                                    decimals: 8,
                                    ..<_>::default()
                                }),
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                                sub_account_metadata_keys: vec![Some(MetadataDescription {
                                    key: "sub".into(),
                                    value_kind: TypeId::of::<String>(),
                                })],
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                                sub_account_metadata_keys: vec![Some(MetadataDescription {
                                    key: "sub".into(),
                                    value_kind: TypeId::of::<String>(),
                                })],
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![Some(BigInt::from(-100))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                                sub_account_metadata_keys: vec![Some(MetadataDescription {
                                    key: "sub".into(),
                                    value_kind: TypeId::of::<String>(),
                                })],
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        // allowed because no amount requirement provided
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_exists: true,
                                sub_account_address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_exists: true,
                                sub_account_address: "sub 1".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![Some(BigInt::from(100))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 1".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        amount: Some(<_>::default()),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_exists: true,
                                sub_account_address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: false,
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_exists: true,
                                sub_account_address: "sub 1".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_exists: true,
                                sub_account_address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_exists: true,
                                sub_account_metadata_keys: vec![Some(MetadataDescription {
                                    key: "validator".into(),
                                    value_kind: TypeId::of::<i32>(),
                                })],
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_exists: true,
                                sub_account_address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_exists: true,
                                sub_account_address: "sub 1".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_optional: true,
                                sub_account_exists: true,
                                sub_account_address: "sub 1".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![None],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                sub_account_optional: true,
                                sub_account_exists: true,
                                sub_account_address: "sub 1".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    operation_descriptions: vec![Some(<_>::default()), Some(<_>::default())],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 3".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    })],
                    amounts: vec![None],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            sub_account: Some(SubAccountIdentifier {
                                address: "sub 2".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    operation_descriptions: vec![Some(OperationDescription {
                        account: Some(AccountDescription {
                            exists: true,
                            ..<_>::default()
                        }),
                        amount: Some(AmountDescription {
                            exists: true,
                            sign: AmountSign::POSITIVE,
                            ..<_>::default()
                        }),
                        allow_repeats: true,
                        ..<_>::default()
                    })],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![Some(Match {
                operations: vec![
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr2".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    operation_descriptions: vec![Some(OperationDescription {
                        account: Some(AccountDescription {
                            exists: true,
                            ..<_>::default()
                        }),
                        amount: Some(AmountDescription {
                            exists: true,
                            sign: AmountSign::POSITIVE,
                            ..<_>::default()
                        }),
                        allow_repeats: true,
                        ..<_>::default()
                    })],
                    err_unmatched: true,
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![Some(OperationDescription {
                        account: Some(AccountDescription {
                            exists: true,
                            ..<_>::default()
                        }),
                        amount: Some(AmountDescription {
                            exists: true,
                            sign: AmountSign::POSITIVE,
                            ..<_>::default()
                        }),
                        allow_repeats: true,
                        ..<_>::default()
                    })],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            allow_repeats: true,
                            ..<_>::default()
                        }),
                        // will never be possible to meet this description
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            allow_repeats: true,
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        type_: "output".into(),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr3".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        type_: "output".into(),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-200".into(),
                            ..<_>::default()
                        }),
                        type_: "input".into(),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr4".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-200".into(),
                            ..<_>::default()
                        }),
                        type_: "input".into(),
                        ..<_>::default()
                    }),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr5".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-1000".into(),
                            ..<_>::default()
                        }),
                        type_: "runoff".into(),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            allow_repeats: true,
                            type_: "output".into(),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            allow_repeats: true,
                            type_: "input".into(),
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            allow_repeats: true,
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..<_>::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..<_>::default()
                            }),
                            type_: "output".into(),
                            ..<_>::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr3".into(),
                                ..<_>::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..<_>::default()
                            }),
                            type_: "output".into(),
                            ..<_>::default()
                        }),
                    ],
                    amounts: vec![Some(BigInt::from(200)), Some(BigInt::from(200))],
                }),
                Some(Match {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..<_>::default()
                            }),
                            amount: Some(Amount {
                                value: "-200".into(),
                                ..<_>::default()
                            }),
                            type_: "input".into(),
                            ..<_>::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr4".into(),
                                ..<_>::default()
                            }),
                            amount: Some(Amount {
                                value: "-200".into(),
                                ..<_>::default()
                            }),
                            type_: "input".into(),
                            ..<_>::default()
                        }),
                    ],
                    amounts: vec![Some(BigInt::from(-200)), Some(BigInt::from(-200))],
                }),
                Some(Match {
                    operations: vec![Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr5".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "-1000".into(),
                            ..<_>::default()
                        }),
                        type_: "runoff".into(),
                        ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            allow_repeats: true,
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            optional: true,
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
                },
            },
            criteria: Some(vec![
                Some(Match {
                    operations: vec![
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr2".into(),
                                ..<_>::default()
                            }),
                            amount: Some(Amount {
                                value: "200".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
                        }),
                        Some(Operation {
                            account: Some(AccountIdentifier {
                                address: "addr1".into(),
                                ..<_>::default()
                            }),
                            amount: Some(Amount {
                                value: "100".into(),
                                ..<_>::default()
                            }),
                            ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    equal_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            allow_repeats: true,
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            optional: true,
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "200".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                    // extra op ignored
                    Some(<_>::default()),
                    Some(Operation {
                        account: Some(AccountIdentifier {
                            address: "addr1".into(),
                            ..<_>::default()
                        }),
                        amount: Some(Amount {
                            value: "100".into(),
                            ..<_>::default()
                        }),
                        ..<_>::default()
                    }),
                ],
                descriptions: Descriptions {
                    opposite_amounts: vec![vec![0, 1]],
                    operation_descriptions: vec![
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::POSITIVE,
                                ..<_>::default()
                            }),
                            allow_repeats: true,
                            ..<_>::default()
                        }),
                        Some(OperationDescription {
                            account: Some(AccountDescription {
                                exists: true,
                                ..<_>::default()
                            }),
                            amount: Some(AmountDescription {
                                exists: true,
                                sign: AmountSign::NEGATIVE,
                                ..<_>::default()
                            }),
                            optional: true,
                            ..<_>::default()
                        }),
                    ],
                    ..<_>::default()
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
                    ..<_>::default()
                })],
                amounts: vec![Some(BigInt::from(100))],
            }),
            criteria: MatchTestResult {
                op: Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..<_>::default()
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
                        ..<_>::default()
                    }),
                    Some(Operation {
                        operation_identifier: OperationIdentifier {
                            index: 2,
                            network_index: None,
                        },
                        ..<_>::default()
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
                    ..<_>::default()
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
                    ..<_>::default()
                })],
                amounts: vec![None],
            }),
            criteria: MatchTestResult {
                op: Some(Operation {
                    operation_identifier: OperationIdentifier {
                        index: 1,
                        network_index: None,
                    },
                    ..<_>::default()
                }),
                amt: None,
            },
        },
    ];

    TestCase::run_output_match(tests, |test| Match::first(test.as_ref()).into());
}
