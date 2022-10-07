use crate::worker::errors::WorkerError;

use super::errors::WorkerResult;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde_json::Value;

/// PopulateInput populates user defined variables in the input
/// with their corresponding values from the execution state.
pub fn populate_input(state: &Value, input: &str) -> WorkerResult<Value> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{\{[^\}]*\}\}").unwrap();
    }

    let mut err = None;
    let input = RE.replace_all(input, |caps: &Captures| {
        // remove special characters
        let cap = caps[0].replacen("{{", "", 1).replacen("}}", "", 1);
        let cap_segments = cap.split('.');

        let mut ret = state;
        for c in cap_segments {
            if let Some(v) = ret.get(&c) {
                ret = v;
            } else {
                err = Some(format!(
                    "{cap} is not present in state: {}",
                    WorkerError::VariableNotFound
                ));
                return "".into();
            }
        }

        ret.to_string()
    });

    if let Some(e) = err {
        Err(format!("{e}: unable to insert variables").into())
    } else if let Ok(v) = serde_json::from_str::<Value>(&input) {
        Ok(v)
    } else {
        tracing::error!("invalid json: {input}");
        Err(WorkerError::InvalidJSON)
    }
}
