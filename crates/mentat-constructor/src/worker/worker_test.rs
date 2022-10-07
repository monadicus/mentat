use httptest::{
    bytes::Bytes,
    http::Request,
    matchers::{ExecutionContext, Matcher},
    responders::status_code,
    Expectation, Server,
};
use indexmap::{indexmap, IndexMap};
use mentat_asserter::{AccountBalanceError, BlockError, ConstructionError};
use mentat_test_utils::TestCase;
use mentat_types::{
    AccountIdentifier, Amount, CoinIdentifier, Currency, SubAccountIdentifier, UncheckedAmount,
    UncheckedCurrency,
};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::{
    fmt::{self, Debug},
    thread::sleep,
    time::Duration,
};

use crate::{
    helpers::get_json,
    job::Scenario,
    job::{Action, ActionType, FindBalanceInput, FindBalanceOutput, HttpMethod, HttpRequestInput},
};

use super::{
    errors::{VerboseWorkerError, WorkerError},
    http_request_worker, Worker,
};

fn unchecked_currency() -> Option<UncheckedCurrency> {
    Some(UncheckedCurrency {
        symbol: "BTC".into(),
        decimals: 8,
        ..Default::default()
    })
}

fn currency() -> Currency {
    Currency {
        symbol: "BTC".into(),
        decimals: 8,
        ..Default::default()
    }
}

fn unchecked_amount_100() -> Option<UncheckedAmount> {
    Some(UncheckedAmount {
        value: "100".into(),
        currency: unchecked_currency(),
        ..Default::default()
    })
}

fn amount_100() -> Option<Amount> {
    Some(Amount {
        value: "100".into(),
        currency: currency(),
        ..Default::default()
    })
}

fn addr4() -> Option<AccountIdentifier> {
    Some(AccountIdentifier {
        address: "addr4".into(),
        ..Default::default()
    })
}

#[test]
fn test_balance_message() {
    let tests = vec![
        TestCase {
            name: "simple message",
            payload: FindBalanceInput {
                minimum_balance: unchecked_amount_100(),
                ..Default::default()
            },
            criteria: r#"looking for balance {"value":"100","currency":{"symbol":"BTC","decimals":8}}"#
                .to_string(),
        },
        TestCase {
            name: "message with account",
            payload: FindBalanceInput {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                minimum_balance: unchecked_amount_100(),
                ..Default::default()
            },
            criteria: r#"looking for balance {"value":"100","currency":{"symbol":"BTC","decimals":8}} on account {"address":"hello"}"#.into(),
        },
        TestCase {
            name: "message with only subaccount",
            payload: FindBalanceInput {
                sub_account_identifier: Some(SubAccountIdentifier {
                    address: "sub hello".into(),
                    ..Default::default()
                }),
                minimum_balance: unchecked_amount_100(),
                ..Default::default()
            },
            criteria: r#"looking for balance {"value":"100","currency":{"symbol":"BTC","decimals":8}} with sub_account {"address":"sub hello"}"#.into(),
        },
        TestCase {
            name: "message with not address",
            payload: FindBalanceInput {
                not_address: vec!["good".into(), "bye".into()],
                minimum_balance: unchecked_amount_100(),
                ..Default::default()
            },
            criteria: r#"looking for balance {"value":"100","currency":{"symbol":"BTC","decimals":8}} != to addresses ["good","bye"]"#.into(),
        },
        TestCase {
            name: "message with not account",
            payload: FindBalanceInput {
                not_account_identifier: vec![
                    Some(AccountIdentifier {
                        address: "good".into(),
                        ..Default::default()
                    }),
                    Some(AccountIdentifier {
                        address: "bye".into(),
                        ..Default::default()
                    }),
                ],
                minimum_balance: unchecked_amount_100(),

                ..Default::default()
            },
            criteria: r#"looking for balance {"value":"100","currency":{"symbol":"BTC","decimals":8}} != to accounts [{"address":"good"},{"address":"bye"}]"#.into(),
        },
        TestCase {
            name: "message with account and not coins",
            payload: FindBalanceInput {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                minimum_balance: unchecked_amount_100(),
                not_coins: vec![CoinIdentifier {
                    identifier: "coin1".into(),
                }],
                ..Default::default()
            },
            criteria: r#"looking for balance {"value":"100","currency":{"symbol":"BTC","decimals":8}} on account {"address":"hello"} != to coins [{"identifier":"coin1"}]"#.into(),
        },
    ];

    TestCase::run_output_match(tests, |test| test.to_string());
}

#[derive(Debug, Clone)]
struct TestFindBalanceWorker {
    input: FindBalanceInput,
    // TODO
    helper: (),
}

#[test]
fn test_find_balance_worker() {
    let tests = vec![
        TestCase {
            name: "simple find balance (satisfiable)",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_account_identifier: vec![addr4()],
                    minimum_balance: unchecked_amount_100(),
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Ok(FindBalanceOutput {
                account_identifier: addr4(),
                balance: amount_100(),
                coin: None,
            }),
        },
        TestCase {
            name: "simple find balance (random create)",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_account_identifier: vec![addr4()],
                    minimum_balance: Some(UncheckedAmount {
                        value: "0".into(),
                        currency: unchecked_currency(),
                        ..Default::default()
                    }),
                    create_limit: 100,
                    create_probability: 100,
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Err(WorkerError::CreateAccount),
        },
        TestCase {
            name: "simple find balance (can't random create)",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_account_identifier: vec![addr4()],
                    minimum_balance: unchecked_amount_100(),
                    create_limit: 100,
                    create_probability: 100,
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Err(WorkerError::Unsatisfiable),
        },
        TestCase {
            name: "simple find balance (no create and unsatisfiable)",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_account_identifier: vec![addr4()],
                    minimum_balance: unchecked_amount_100(),
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Err(WorkerError::Unsatisfiable),
        },
        TestCase {
            name: "simple find balance and create",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_account_identifier: vec![addr4()],
                    minimum_balance: Some(UncheckedAmount {
                        value: "0".into(),
                        currency: unchecked_currency(),
                        ..Default::default()
                    }),
                    create_limit: 100,
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Err(WorkerError::CreateAccount),
        },
        TestCase {
            name: "simple find balance with subaccount",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    sub_account_identifier: Some(SubAccountIdentifier {
                        address: "sub1".into(),
                        ..Default::default()
                    }),
                    not_address: vec!["addr4".into()],
                    minimum_balance: unchecked_amount_100(),
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Ok(FindBalanceOutput {
                account_identifier: Some(AccountIdentifier {
                    address: "addr1".into(),
                    sub_account: Some(SubAccountIdentifier {
                        address: "sub1".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                balance: amount_100(),
                coin: None,
            }),
        },
        TestCase {
            name: "simple find coin",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_address: vec!["addr4".into()],
                    minimum_balance: unchecked_amount_100(),
                    require_coin: true,
                    not_coins: vec![CoinIdentifier {
                        identifier: "coin1".into(),
                    }],
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Ok(FindBalanceOutput {
                account_identifier: Some(AccountIdentifier {
                    address: "addr1".into(),
                    ..Default::default()
                }),
                balance: amount_100(),
                coin: Some(CoinIdentifier {
                    identifier: "coin2".into(),
                }),
            }),
        },
        TestCase {
            name: "could not find coin (no create)",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_address: vec!["addr4".into()],
                    minimum_balance: unchecked_amount_100(),
                    require_coin: true,
                    not_coins: vec![CoinIdentifier {
                        identifier: "coin1".into(),
                    }],
                    create_limit: -1,
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Err(WorkerError::Unsatisfiable),
        },
        TestCase {
            name: "could not find coin (unsatisfiable)",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_address: vec!["addr4".into()],
                    minimum_balance: unchecked_amount_100(),
                    require_coin: true,
                    not_coins: vec![CoinIdentifier {
                        identifier: "coin1".into(),
                    }],
                    create_limit: 10,
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Err(WorkerError::Unsatisfiable),
        },
        TestCase {
            name: "could not find coin (too many accounts)",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_address: vec!["addr4".into()],
                    minimum_balance: unchecked_amount_100(),
                    require_coin: true,
                    not_coins: vec![CoinIdentifier {
                        identifier: "coin1".into(),
                    }],
                    create_limit: 2,
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Err(WorkerError::Unsatisfiable),
        },
        TestCase {
            name: "invalid amount",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_address: vec!["addr4".into()],
                    minimum_balance: Some(UncheckedAmount {
                        value: String::new(),
                        currency: unchecked_currency(),
                        ..Default::default()
                    }),
                    require_coin: true,
                    not_coins: vec![CoinIdentifier {
                        identifier: "coin1".into(),
                    }],
                    create_limit: 2,
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Err(BlockError::AmountValueMissing.into()),
        },
        TestCase {
            name: "invalid currency",
            payload: TestFindBalanceWorker {
                input: FindBalanceInput {
                    not_address: vec!["addr4".into()],
                    minimum_balance: Some(UncheckedAmount {
                        value: "100".into(),
                        currency: Some(UncheckedCurrency {
                            symbol: String::new(),
                            decimals: 8,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    require_coin: true,
                    not_coins: vec![CoinIdentifier {
                        identifier: "coin1".into(),
                    }],
                    create_limit: 2,
                    ..Default::default()
                },
                helper: (),
            },
            criteria: Err(BlockError::AmountCurrencySymbolEmpty.into()),
        },
    ];
    todo!()
}

fn assert_variable_equality<T: DeserializeOwned + PartialEq + Debug>(
    state: &Value,
    variable: &str,
    expected: &T,
) {
    let value = get_json(state, variable).unwrap().clone();
    let item = serde_json::from_value::<T>(value).unwrap();
    assert_eq!(&item, expected);
}

#[test]
fn test_job_complicated_transfer() {
    let s = Scenario {
        name: "create_address".into(),
        actions: vec![
            Action {
                input: r#"{"network":"Testnet3", "blockchain":"Bitcoin"}"#.into(),
                type_: ActionType::SetVariable,
                output_path: Some("network".into()),
            },
            Action {
                input: r#"{"curve_type": "secp256k1"}"#.into(),
                type_: ActionType::GenerateKey,
                output_path: Some("key".into()),
            },
            Action {
                input: r#"{"network_identifier": {{network}}, "public_key": {{key.public_key}}}"#.into(),
                type_: ActionType::Derive,
                output_path: Some("account".into()),
            },
            Action {
                input: r#"{"account_identifier": {{account.account_identifier}}, "keypair": {{key.public_key}}}"#.into(),
                type_: ActionType::SaveAccount,
                output_path: None,
            },
            Action {
                input: r#"{{account.account_identifier}}"#.into(),
                type_: ActionType::PrintMessage,
                output_path: None,
            },
        ],
    };

    let s2 = Scenario {
        name: "create_send".into(),
        actions: vec![
            Action {
                input: r#"{"symbol":"BTC","decimals":8}"#.into(),
                type_: ActionType::SetVariable,
                output_path: Some("default_curr".into()),
            },
            Action {
                input: r#"{"regex": "[a-z]+", "limit":10}"#.into(),
                type_: ActionType::RandomString,
                output_path: Some("random_address".into()),
            },
            Action {
                input: r#"[{"operation_identifier":{"index":0},"type":"","account":{{account.account_identifier}},"amount":{"value":"-90","currency":{{default_curr}}}},{"operation_identifier":{"index":1},"type":"","account":{"address":{{random_address}}},"amount":{"value":"100","currency":{{default_curr}}}}]`, // noli"#.into(),
                type_: ActionType::SetVariable,
                output_path: Some("create_send.operations".into()),
            },
            Action {
                input: r#"{{network}}"#.into(),
                type_: ActionType::SetVariable,
                output_path: Some("create_send.network".into()),
            },
            Action {
                input: r#"{"minimum":"10", "maximum":"100"}"#.into(),
                type_: ActionType::RandomNumber,
                output_path: Some("rand_number".into()),
            },
            Action {
                input: r#"{{rand_number}}"#.into(),
                type_: ActionType::Assert,
                output_path: None,
            },
            Action {
                input: r#"{"symbol":"ETH","decimals":18}"#.into(),
                type_: ActionType::SetVariable,
                output_path: Some("eth_curr".into()),
            },
            Action {
                input: r#"[{"value":"100", "currency":{{default_curr}}},{"value":"200", "currency":{{eth_curr}}}]"#.into(),
                type_: ActionType::SetVariable,
                output_path: Some("mock_suggested_fee_resp".into()),
            },
            Action {
                input: r#"{"currency":{{eth_curr}}, "amounts":{{mock_suggested_fee_resp}}}"#.into(),
                type_: ActionType::FindCurrencyAmount,
                output_path: Some("eth_amount".into()),
            },
            Action {
                input: r#"{"operation":"subtraction", "left_value":{{eth_amount.value}}, "right_value":"200"}"#.into(),
                type_: ActionType::Math,
                output_path: Some("eth_check".into()),
            },
            Action {
                input: r#"{{eth_check}}"#.into(),
                type_: ActionType::Assert,
                output_path: None,
            },
            Action {
                input: r#"{"random_number": {{rand_number}}}"#.into(),
                type_: ActionType::PrintMessage,
                output_path: None,
            },
            Action {
                input: r#""valA""#.into(),
                type_: ActionType::LoadEnv,
                output_path: Some("valA".into()),
            },
            Action {
                input: r#""16""#.into(),
                type_: ActionType::SetVariable,
                output_path: Some("valB".into()),
            },
            Action {
                input: r#"{"operation":"addition", "left_value":{{valA}}, "right_value":{{valB}}}"#.into(),
                type_: ActionType::Math,
                output_path: Some("create_send.confirmation_depth".into()),
            },
            // Attempt to overwrite confirmation depth
            Action {
                input: r#"{"operation":"subtraction", "left_value":"100", "right_value":{{create_send.confirmation_depth}}}"#.into(),
                type_: ActionType::Math,
                output_path: Some("create_send.confirmation_depth".into()),
            },
            // Test multiplication / division
            Action {
                input: r#"{"operation":"multiplication", "left_value":"2", "right_value":{{create_send.confirmation_depth}}}"#.into(),
                type_: ActionType::Math,
                output_path: Some("create_send.confirmation_depth".into()),
            },
            Action {
                input: r#"{"operation":"division", "left_value":"296", "right_value":{{create_send.confirmation_depth}}}"#.into(),
                type_: ActionType::Math,
                output_path: Some("create_send.confirmation_depth".into()),
            },
        ],
    };

    todo!()
}

#[derive(Debug, Clone)]
struct TestJobFailures {
    scenario: Scenario,
    new_index: usize,
    complete: bool,
}

#[test]
fn test_job_failures() {
    let tests = vec![
        TestCase {
            name: "invalid action",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"{"network":"Testnet3", "blockchain":"Bitcoin"}"#.into(),
                        // TODO should contain text "stuff"
                        type_: ActionType::Unknown,
                        output_path: Some("network".into()),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: r#"{"network":"Testnet3", "blockchain":"Bitcoin"}"#.into(),
                    // TODO should contain text "stuff"
                    type_: ActionType::Unknown,
                    output_path: Some("network".into()),
                }),
                processed_input: Some(json!({"network":"Testnet3", "blockchain":"Bitcoin"})),
                err: WorkerError::InvalidActionType,
                ..Default::default()
            },
        },
        TestCase {
            name: "assertion invalid input",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: "\"hello\"".into(),
                        type_: ActionType::Assert,
                        output_path: Default::default(),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: "\"hello\"".into(),
                    type_: ActionType::Assert,
                    output_path: Default::default(),
                }),
                processed_input: Some(json!("hello")),
                err: WorkerError::String("hello is not an integer".into()),
                ..Default::default()
            },
        },
        TestCase {
            name: "failed assertion",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: "\"-1\"".into(),
                        type_: ActionType::Assert,
                        output_path: Default::default(),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: "\"-1\"".into(),
                    type_: ActionType::Assert,
                    output_path: Default::default(),
                }),
                processed_input: Some(json!(-1)),
                err: WorkerError::ActionFailed,
                ..Default::default()
            },
        },
        TestCase {
            name: "invalid currency",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"{"currency":{"decimals":8}}"#.into(),
                        type_: ActionType::FindCurrencyAmount,
                        output_path: Default::default(),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: r#"{"currency":{"decimals":8}}"#.into(),
                    type_: ActionType::FindCurrencyAmount,
                    output_path: Default::default(),
                }),
                processed_input: Some(json!({"currency":{"decimals":8}})),
                err: BlockError::AmountCurrencySymbolEmpty.into(),
                ..Default::default()
            },
        },
        TestCase {
            name: "repeat currency",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"{"currency":{"symbol":"BTC", "decimals":8},"amounts":[{"value":"100","currency":{"symbol":"BTC", "decimals":8}},{"value":"100","currency":{"symbol":"BTC", "decimals":8}}]}"#.into(),
                        type_: ActionType::FindCurrencyAmount,
                        output_path: Default::default(),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: r#"{"currency":{"symbol":"BTC", "decimals":8},"amounts":[{"value":"100","currency":{"symbol":"BTC", "decimals":8}},{"value":"100","currency":{"symbol":"BTC", "decimals":8}}]}"#.into(),
                    type_: ActionType::FindCurrencyAmount,
                    output_path: Default::default(),
                }),
                processed_input: Some(json!({"currency":{"symbol":"BTC", "decimals":8},"amounts":[{"value":"100","currency":{"symbol":"BTC", "decimals":8}},{"value":"100","currency":{"symbol":"BTC", "decimals":8}}]})),
                err: AccountBalanceError::CurrencyUsedMultipleTimes.into(),
                ..Default::default()
            },
        },
        TestCase {
            name: "can't find currency",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec!
                    [Action {
                        input: "\"1\"".into(),
                        type_: ActionType::Assert,
                        output_path: Default::default(),
                    },
                    Action {
                        input: "\"BTC\"".into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("symbol".into()),
                    },
                    Action {
                        input: r#"{"currency":{"symbol":{{symbol}}, "decimals":8}}"#.into(),
                        type_: ActionType::FindCurrencyAmount,
                        output_path: Default::default(),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action_index: 2,
                action: Some(Action {
                    input: r#"{"currency":{"symbol":{{symbol}}, "decimals":8}}"#.into(),
                    type_: ActionType::FindCurrencyAmount,
                    output_path: Default::default(),
                }),
                processed_input: Some(json!({"currency":{"symbol":"BTC", "decimals":8}})),
                state: Some(json!({"symbol":"BTC"})),
                err: WorkerError::ActionFailed,
                ..Default::default()
            },
        },
        TestCase {
            name: "invalid json",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#""network":"Testnet3", "blockchain":"Bitcoin"}"#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("network".into()) }] },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: r#""network":"Testnet3", "blockchain":"Bitcoin"}"#.into(),
                    type_: ActionType::SetVariable,
                    output_path: Some("network".into()),
                }),
                err: WorkerError::InvalidJSON,
                ..Default::default()
            },
        },
        TestCase {
            name: "missing variable",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"{"network":{{var}}, "blockchain":"Bitcoin"}"#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("network".into())
                    }]
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: r#"{"network":{{var}}, "blockchain":"Bitcoin"}"#.into(),
                    type_: ActionType::SetVariable,
                    output_path: Some("network".into()),
                }),
                err: WorkerError::VariableNotFound,
                ..Default::default()
            },
        },
        TestCase {
            name: "invalid input: negative difference in random amount",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "random_number".into(),
                    actions: vec![Action {
                        input: r#"{"minimum":"-100", "maximum":"-200"}"#.into(),
                        type_: ActionType::RandomNumber,
                        output_path: Default::default()
                    }] },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "random_number".into(),
                action: Some(Action {
                    input: r#"{"minimum":"-100", "maximum":"-200"}"#.into(),
                    type_: ActionType::RandomNumber,
                    output_path: Default::default(),
                }),
                processed_input: Some(json!({"minimum":"-100", "maximum":"-200"})),
                err: WorkerError::String("maximum value -200 < minimum value -100".into()),
                ..Default::default()
            }
        },
        TestCase {
            name: "invalid input: generate key",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"{"curve_typ": "secp256k1"}}"#.into(),
                        type_: ActionType::GenerateKey,
                        output_path: Some("key".into()),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: r#"{"curve_typ": "secp256k1"}"#.into(),
                    type_: ActionType::GenerateKey,
                    output_path: Some("key".into()),
                }),
                processed_input: Some(json!({"curve_typ": "secp256k1"})),
                err: WorkerError::String("unknown field \"curve_typ\"".into()),
                ..Default::default()
            }
        },
        TestCase {
            name: "invalid input: derive",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"{"public_key": {}}"#.into(),
                        type_: ActionType::Derive,
                        output_path: Some("address".into()),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: r#"{"public_key": {}}"#.into(),
                    type_: ActionType::Derive,
                    output_path: Some("address".into()),
                }),
                processed_input: Some(json!({"public_key": {}})),
                err: ConstructionError::PublicKeyBytesEmpty.into(),
                ..Default::default()
            }
        },
        TestCase {
            name: "invalid input: save address input",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: "{}".into(),
                        type_: ActionType::SaveAccount,
                        output_path: Default::default(),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: "{}".into(),
                    type_: ActionType::SaveAccount,
                    output_path: Default::default(),
                }),
                processed_input: Some(json!({})),
                err: BlockError::AccountIsNil.into(),
                ..Default::default()
            }
        },
        TestCase {
            name: "invalid action: job.Math",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"{"operation":"addition", "left_value":"1", "right_value":"B"}"#.into(),
                        type_: ActionType::Math,
                        output_path: Default::default(),
                    }],
                },
                new_index: Default::default(),
                complete: Default::default(),
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: r#"{"operation":"addition", "left_value":"1", "right_value":"B"}"#.into(),
                    type_: ActionType::Math,
                    output_path: Default::default(),
                }),
                processed_input: Some(json!({"operation":"addition", "left_value":"1", "right_value":"B"})),
                err: WorkerError::String("B is not an integer".into()),
                ..Default::default()
            }
        },
        TestCase {
            name: "invalid broadcast: invalid operations",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"[{"operation_identifier":{"index":0},"type":"","statsbf":""}]"#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("create_send.operations".into()),
                    }],
                },
                new_index: 1,
                complete: true,
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_send".into(),
                state: Some(json!({"create_send":{"operations":[{"operation_identifier":{"index":0},"type":"","statsbf":""}]}})),
                err: WorkerError::String("failed to unmarshal operations of scenario create_send".into()),
                ..Default::default()
            }
        },
        TestCase {
            name: "invalid broadcast: missing confirmation depth",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_send".into(),
                    actions: vec![Action {
                        input: r#"[{"operation_identifier":{"index":0},"type":"","status":""}]"#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Default::default(),
                    }],
                },
                new_index: 1,
                complete: true,
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_send".into(),
                state: Some(json!({"create_send":{"operations":[{"operation_identifier":{"index":0},"type":"","status":""}]}})),
                processed_input: Some(json!({"currency":{"decimals":8}})),
                err: WorkerError::String("failed to unmarshal confirmation depth of scenario create_send".into()),
                ..Default::default()
            }
        },
        TestCase {
            name: "invalid broadcast: missing network identifier",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_send".into(),
                    actions: vec![
                        Action {
                        input: r#"`[{"operation_identifier":{"index":0},"type":"","status":""}]"#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("create_send.operations".into()),
                    },
                    Action {
                        input: r#""10""#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("create_send.confirmation_depth".into()),
                    }],
                },
                new_index: 1,
                complete: true,
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_send".into(),
                state: Some(json!({"create_send":{"operations":[{"operation_identifier":{"index":0},"type":"","status":""}],"confirmation_depth":"10"}})),
                err: WorkerError::String("failed to unmarshal network of scenario create_send".into()),
                ..Default::default()
            }
        },
        TestCase {
            name: "invalid broadcast: metadata incorrect",
            payload: TestJobFailures {
                scenario: Scenario {
                    name: "create_send".into(),
                    actions: vec![
                        Action {
                        input: r#"[{"operation_identifier":{"index":0},"type":"","status":""}]"#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("create_send.operations".into()),
                    },
                    Action {
                        input: r#""10""#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("create_send.confirmation_depth".into()),
                    },
                    Action {
                        input: r#"{"network":"Testnet3", "blockchain":"Bitcoin"}"#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("create_send.network".into()),
                    },
                    Action {
                        input: r#""hello""#.into(),
                        type_: ActionType::SetVariable,
                        output_path: Some("create_send.preprocess_metadata".into()),
                    },
                    ],
                },
                new_index: 1,
                complete: true,
            },
            criteria: VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_send".into(),
                state: Some(json!({"create_send":{"operations":[{"operation_identifier":{"index":0},"type":"","status":""}],"confirmation_depth":"10","network":{"network":"Testnet3", "blockchain":"Bitcoin"},"preprocess_metadata":"hello"}})),
                err: WorkerError::String("failed to unmarshal preprocess metadata of scenario create_send".into()),
                ..Default::default()
            }
        },
    ];
    todo!()
}

#[derive(Debug, Clone, Default)]
struct TestHttpRequestWorkerPayload {
    input: HttpRequestInput,
    dont_prepend_url: bool,
    expected_path: &'static str,
    expected_latency: Duration,
    expected_method: Method,
    expected_body: Value,
    response: Value,
    content_type: &'static str,
    status_code: StatusCode,
}

impl Matcher<Request<Bytes>> for TestHttpRequestWorkerPayload {
    fn matches(&mut self, input: &Request<Bytes>, _: &mut ExecutionContext) -> bool {
        sleep(self.expected_latency);
        input.method() == self.expected_method
            && input.uri() == self.expected_path
            && matches!(serde_json::to_value(input.body().as_ref()), Ok(b) if b == self.expected_body)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[test]
fn test_http_request_worker() {
    let tests = vec![
        TestCase {
            name: "simple get",
            payload: TestHttpRequestWorkerPayload {
                input: HttpRequestInput {
                    method: HttpMethod::Get,
                    url: "/faucet?test=123".into(),
                    timeout: 100,
                    body: Default::default(),
                },
                dont_prepend_url: false,
                expected_path: "/faucet?test=123",
                expected_latency: Duration::from_millis(1),
                expected_method: HttpMethod::Get.into(),
                expected_body: json!(""),
                response: json!({"money":100}),
                content_type: "application/json; charset=UTF-8",
                status_code: StatusCode::OK,
            },
            criteria: Ok(json!({"money":100})),
        },
        TestCase {
            name: "simple post",
            payload: TestHttpRequestWorkerPayload {
                input: HttpRequestInput {
                    method: HttpMethod::Post,
                    url: "/faucet".into(),
                    timeout: 100,
                    body: json!({"address":"123"}),
                },
                dont_prepend_url: false,
                expected_path: "/faucet",
                expected_latency: Duration::from_millis(1),
                expected_method: HttpMethod::Post.into(),
                expected_body: json!({"address":"123"}),
                response: json!({"money":100}),
                content_type: "application/json; charset=UTF-8",
                status_code: StatusCode::OK,
            },
            criteria: Ok(json!({"money":100})),
        },
        TestCase {
            name: "invalid method",
            payload: TestHttpRequestWorkerPayload {
                input: HttpRequestInput {
                    //todo
                    method: HttpMethod::Unknown,
                    url: "/faucet".into(),
                    timeout: 100,
                    body: json!({"address":"123"}),
                },
                dont_prepend_url: false,
                ..Default::default()
            },
            criteria: Err(WorkerError::InvalidInput),
        },
        TestCase {
            name: "invalid timeout",
            payload: TestHttpRequestWorkerPayload {
                input: HttpRequestInput {
                    method: HttpMethod::Post,
                    url: "/faucet".into(),
                    timeout: -1,
                    body: json!({"address":"123"}),
                },
                dont_prepend_url: false,
                ..Default::default()
            },
            criteria: Err(WorkerError::InvalidInput),
        },
        TestCase {
            name: "no url",
            payload: TestHttpRequestWorkerPayload {
                input: HttpRequestInput {
                    method: HttpMethod::Post,
                    url: "".into(),
                    timeout: 100,
                    body: json!({"address":"123"}),
                },
                dont_prepend_url: true,
                ..Default::default()
            },
            criteria: Err(WorkerError::String("empty url".into())),
        },
        TestCase {
            name: "invalid url",
            payload: TestHttpRequestWorkerPayload {
                input: HttpRequestInput {
                    method: HttpMethod::Post,
                    url: "blah".into(),
                    timeout: 100,
                    body: json!({"address":"123"}),
                },
                dont_prepend_url: true,
                ..Default::default()
            },
            criteria: Err(WorkerError::String("invalid URI for request".into())),
        },
        TestCase {
            name: "timeout",
            payload: TestHttpRequestWorkerPayload {
                input: HttpRequestInput {
                    method: HttpMethod::Get,
                    url: "/faucet?test=123".into(),
                    timeout: 1,
                    body: Default::default(),
                },
                dont_prepend_url: false,
                expected_path: "/faucet?test=123",
                expected_latency: Duration::from_millis(1200),
                expected_method: HttpMethod::Get.into(),
                expected_body: json!(""),
                response: json!({"money":100}),
                content_type: "application/json; charset=UTF-8",
                status_code: StatusCode::OK,
            },
            criteria: Err(WorkerError::String(
                "context deadline exceeded (Client.Timeout exceeded while awaiting headers)".into(),
            )),
        },
        TestCase {
            name: "error",
            payload: TestHttpRequestWorkerPayload {
                input: HttpRequestInput {
                    method: HttpMethod::Get,
                    url: "/faucet?test=123".into(),
                    timeout: 10,
                    body: Default::default(),
                },
                dont_prepend_url: false,
                expected_path: "/faucet?test=123",
                expected_latency: Duration::from_millis(1),
                expected_method: HttpMethod::Get.into(),
                expected_body: json!(""),
                response: json!({"money":100}),
                content_type: "application/json; charset=UTF-8",
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            criteria: Err(WorkerError::ActionFailed),
        },
        // we don't throw an error
        TestCase {
            name: "invalid content type",
            payload: TestHttpRequestWorkerPayload {
                input: HttpRequestInput {
                    method: HttpMethod::Get,
                    url: "/faucet?test=123".into(),
                    timeout: 10,
                    body: Default::default(),
                },
                dont_prepend_url: false,
                expected_path: "/faucet?test=123",
                expected_latency: Duration::from_millis(1),
                expected_method: HttpMethod::Get.into(),
                expected_body: json!(""),
                response: json!({"money":100}),
                content_type: "text/plain",
                status_code: StatusCode::OK,
            },
            criteria: Ok(json!({"money":100})),
        },
    ];

    TestCase::run_async_result_match(tests, |mut test| async move {
        let ts = Server::run();
        ts.expect(
            Expectation::matching(test.clone()).respond_with(
                status_code(test.status_code.into())
                    .append_header("Content-Type", test.content_type)
                    .body(test.response.to_string()),
            ),
        );

        if !test.dont_prepend_url {
            test.input.url = ts.url_str(&test.input.url)
        }

        let value = serde_json::to_value(test.input).unwrap();
        http_request_worker(value).await
    });
}

#[derive(Debug, Clone)]
struct TestBlobWorkers {
    scenario: Scenario,
    // TODO
    helper: (),
    asserter_state: IndexMap<String, String>,
}

#[test]
fn test_blob_workers() {
    let tests = vec![
        TestCase {
            name: "simple save and get",
            payload: TestBlobWorkers {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![
                        Action {
                            input: r#"{"key":"Testnet3", "value":"Bitcoin"}"#.into(),
                            type_: ActionType::SetBlob,
                            output_path: Default::default(),
                        },
                        Action {
                            input: r#"{"key":"Testnet3"}"#.into(),
                            type_: ActionType::GetBlob,
                            output_path: Some("k".into()),
                        },
                    ],
                },
                helper: (),
                asserter_state: indexmap! {"k".into() => "Bitcoin".into()},
            },
            criteria: None,
        },
        TestCase {
            name: "get missing",
            payload: TestBlobWorkers {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"{"key":"Testnet3"}"#.into(),
                        type_: ActionType::GetBlob,
                        output_path: Some("k".into()),
                    }],
                },
                helper: (),
                asserter_state: Default::default(),
            },
            criteria: Some(VerboseWorkerError {
                workflow: "random".into(),
                scenario: "create_address".into(),
                action: Some(Action {
                    input: r#"{"key":"Testnet3"}"#.into(),
                    type_: ActionType::GetBlob,
                    output_path: Some("k".into()),
                }),
                processed_input: Some(json!({ "key": "Testnet3" })),
                err: WorkerError::ActionFailed,
                ..Default::default()
            }),
        },
        TestCase {
            name: "complex save and get",
            payload: TestBlobWorkers {
                scenario: Scenario {
                    name: "create_address".into(),
                    actions: vec![Action {
                        input: r#"{"key":{"address":"hello", "sub_account":{"address":"neat"}}, "value":{"stuff":"neat"}}"#.into(),
                        type_: ActionType::SetBlob,
                        output_path: None,
                    },
                    Action {
                        input: r#"{"key":{"address":"hello", "sub_account":{"address":"neat2"}}, "value":"addr2"}"#.into(),
                        type_: ActionType::SetBlob,
                        output_path: None,
                    },
                    // switch order
                    Action {
                        input: r#"{"key":{"sub_account":{"address":"neat"}, "address":"hello"}}"#.into(),
                        type_: ActionType::GetBlob,
                        output_path: Some("k".into()),
                    }],
                },
                helper: (),
                asserter_state: indexmap!{
                    "k".into() => json!({
                        "stuff": "neat"
                    }).to_string()
                },
            },
            criteria: None,
        },
    ];
    todo!()
}
