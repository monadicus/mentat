use std::{convert::TryInto, str::FromStr};

use mentat_test_utils::TestCase;
use serde_json::Value;

use super::{errors::WorkerError, populator::populate_input};

#[test]
fn test_populate_input() {
    let tests = vec![
        TestCase {
            name: "no variables",
            payload: ("", "{\"foo\": \"bar\"}"),
            criteria: Ok(Value::from_str("{\"foo\": \"bar\"}").unwrap()),
        },
        TestCase {
            name: "single variable (string)",
            payload: ("{\"network\": \"test\"}", "{\"foo\": {{network}}}"),
            criteria: Ok("{\"foo\": \"test\"}".try_into().unwrap()),
        },
        TestCase {
            name: "single variable (object)",
            payload: (
                "{
                    \"network\": {\"network\":\"Testnet3\", \"blockchain\":\"Bitcoin\"}
                }",
                "{\"foo\": {{network}}}",
            ),
            criteria: Ok("{\"foo\": {\"network\":\"Testnet3\", \"blockchain\":\"Bitcoin\"}}".try_into().unwrap()),
        },
        TestCase {
            name: "single variable used twice",
            payload: (
                "{
                    \"network\": {\"network\":\"Testnet3\", \"blockchain\":\"Bitcoin\"}
                }",
                "{\"foo\": {{network}}, \"foo2\": {{network}}}",
            ),
            criteria: Ok("{\"foo\": {\"network\":\"Testnet3\", \"blockchain\":\"Bitcoin\"}, \"foo2\": {\"network\":\"Testnet3\", \"blockchain\":\"Bitcoin\"}}".try_into().unwrap()),
        },
        TestCase {
            name: "multiple variables",
            payload: (
                "{
                    \"network\": {\"network\":\"Testnet3\", \"blockchain\":\"Bitcoin\"},
                    \"key\": {\"public_key\":{\"curve_type\": \"secp256k1\", \"hex_bytes\": \"03a6946b55ee2da05d57049a31df1bfd97ff2e5810057f4fb63e505622cdafd513\"}}}",
                "{\"foo\": {{network}}, \"bar\": {{key.public_key}}}"
            ),
            criteria: Ok("{\"foo\": {\"network\":\"Testnet3\", \"blockchain\":\"Bitcoin\"}, \"bar\": {\"curve_type\": \"secp256k1\", \"hex_bytes\": \"03a6946b55ee2da05d57049a31df1bfd97ff2e5810057f4fb63e505622cdafd513\"}}".try_into().unwrap())
        },
        TestCase {
            name: "single variable (doesn't exist)",
            payload: (
                "",
                "{\"foo\": {{network}}}"
            ),
            criteria: Err("network is not present in state".into()),
        },
        TestCase {
            name: "single variable path doesn't exist",
            payload: (
                "{\"network\": {\"network\":\"Testnet3\", \"blockchain\":\"Bitcoin\"}}",
                "{\"foo\": {{network.test}}}"
            ),
            criteria: Err("network.test is not present in state".into()),
        },
        TestCase {
            name: "invalid json result",
            payload: (
                "{\"network\": {\"network\":\"Testnet3\", \"blockchain\":\"Bitcoin\"}}",
                "{{"
            ),
            criteria: Err(WorkerError::InvalidJSON),
        },
    ];

    TestCase::run_result_match(tests, |(state, input)| {
        populate_input(&state.try_into().unwrap(), input)
    });
}
