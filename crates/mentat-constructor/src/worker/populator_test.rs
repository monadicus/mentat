use mentat_test_utils::TestCase;

use super::{errors::WorkerError, populator::populate_input};

#[test]
fn test_populate_input() {
    let tests = vec![
        TestCase {
            name: "no variables",
            payload: ("", r#"{"foo": "bar"}"#),
            criteria: Ok(serde_json::from_str(r#"{"foo": "bar"}"#).unwrap()),
        },
        TestCase {
            name: "single variable (string)",
            payload: (r#"{"network": "test"}"#, r#"{"foo": {{network}}}"#),
            criteria: Ok(serde_json::from_str(r#"{"foo": "test"}"#).unwrap()),
        },
        TestCase {
            name: "single variable (object)",
            payload: (
                r#"{
                    "network": {"network":"Testnet3", "blockchain":"Bitcoin"}
                }"#,
                r#"{"foo": {{network}}}"#,
            ),
            criteria: Ok(serde_json::from_str(r#"{"foo": {"network":"Testnet3", "blockchain":"Bitcoin"}}"#).unwrap()),
        },
        TestCase {
            name: "single variable used twice",
            payload: (
                r#"{
                    "network": {"network":"Testnet3", "blockchain":"Bitcoin"}
                }"#,
                r#"{"foo": {{network}}, "foo2": {{network}}}"#,
            ),
            criteria: Ok(serde_json::from_str(r#"{"foo": {"network":"Testnet3", "blockchain":"Bitcoin"}, "foo2": {"network":"Testnet3", "blockchain":"Bitcoin"}}"#).unwrap()),
        },
        TestCase {
            name: "multiple variables",
            payload: (
                r#"{
                    "network": {"network":"Testnet3", "blockchain":"Bitcoin"},
                    "key": {"public_key":{"curve_type": "secp256k1", "hex_bytes": "03a6946b55ee2da05d57049a31df1bfd97ff2e5810057f4fb63e505622cdafd513"}}}"#,
                r#"{"foo": {{network}}, "bar": {{key.public_key}}}"#
            ),
            criteria: Ok(serde_json::from_str(r#"{"foo": {"network":"Testnet3", "blockchain":"Bitcoin"}, "bar": {"curve_type": "secp256k1", "hex_bytes": "03a6946b55ee2da05d57049a31df1bfd97ff2e5810057f4fb63e505622cdafd513"}}"#).unwrap())
        },
        TestCase {
            name: "single variable (doesn't exist)",
            payload: (
                "",
                r#"{"foo": {{network}}}"#
            ),
            criteria: Err("network is not present in state".into()),
        },
        TestCase {
            name: "single variable path doesn't exist",
            payload: (
                r#"{"network": {"network":"Testnet3", "blockchain":"Bitcoin"}}"#,
                r#"{"foo": {{network.test}}}"#
            ),
            criteria: Err("network.test is not present in state".into()),
        },
        TestCase {
            name: "invalid json result",
            payload: (
                r#"{"network": {"network":"Testnet3", "blockchain":"Bitcoin"}}"#,
                r#"{{"#
            ),
            criteria: Err(WorkerError::InvalidJSON),
        },
    ];

    TestCase::run_ok_match_err_contains(tests, |(state, input)| {
        populate_input(&serde_json::from_str(state).unwrap_or_default(), input)
    });
}
