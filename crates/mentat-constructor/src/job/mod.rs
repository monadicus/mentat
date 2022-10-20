mod errors;
pub use errors::*;
mod types;
pub use types::*;

use mentat_types::{Amount, Metadata, Operation, Transaction};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::helpers::get_json;

impl Job {
    /// creates a new Job.
    pub fn new(workflow: Workflow) -> Self {
        Self {
            identifier: Default::default(),
            state: Value::Object(Default::default()),
            index: Default::default(),
            status: Status::Ready,
            workflow: workflow.name,
            scenarios: workflow.scenarios,
        }
    }

    /// returns a Broadcast for a given job or nil if none is required.
    pub fn create_broadcast(&mut self) -> JobResult<Option<Broadcast>> {
        // ProcessNextScenario will increment the index, so we need to subtract
        // by 1 when attempting to create a broadcast payload.
        let broadcast_index = self
            .index
            .checked_sub(1)
            .ok_or(JobError::UnableToCreateBroadcast)?;

        let scenario_name = &self.scenarios[broadcast_index].name;

        let operations = match self
            .deserialize_struct::<Vec<Operation>>(scenario_name, ReservedVariable::Operations)
        {
            Ok(v) => v,
            Err(JobError::VariableNotFound) => {
                // If <scenario.Name>.operations are not provided, no broadcast
                // is required.
                if self.check_complete() {
                    self.status = Status::Completed
                }

                return Ok(None);
            }
            Err(e) => {
                return Err(format!(
                    "failed to deserialize operations of scenario {scenario_name}: {e}",
                )
                .into());
            }
        };

        let confirmation_depth =
            self.deserialize_number(scenario_name, ReservedVariable::ConfirmationDepth)
                .map_err(|e| {
                    format!(
                        "failed to deserialize confirmation depth of scenario {scenario_name}: {e}",
                    )
                })?;

        let network = self
            .deserialize_struct(scenario_name, ReservedVariable::Network)
            .map_err(|e| {
                format!("failed to deserialize network of scenario {scenario_name}: {e}",)
            })?;

        let metadata = match self
            .deserialize_struct::<Metadata>(scenario_name, ReservedVariable::PreprocessMetadata)
        {
            Ok(v) => v,
            Err(JobError::VariableNotFound) => Default::default(),
            Err(e) => {
                return Err(format!(
                    "failed to deserialize preprocess metadata of scenario {scenario_name}: {e}",
                )
                .into())
            }
        };

        let dry_run = match self.deserialize_boolean(scenario_name, ReservedVariable::DryRun) {
            Ok(v) => v,
            Err(JobError::VariableNotFound) => Default::default(),
            Err(e) => {
                return Err(format!(
                    "failed to deserialize dry run of scenario {scenario_name}: {e}",
                )
                .into())
            }
        };

        self.status = Status::Broadcasting;

        Ok(Some(Broadcast {
            network,
            intent: operations,
            metadata,
            confirmation_depth,
            dry_run,
        }))
    }

    /// attempts to strictly deserialize some input into output.
    pub fn deserialize_value<T: DeserializeOwned>(input: Value) -> JobResult<T> {
        serde_json::from_value(input).map_err(|e| format!("unable to decode: {e}").into())
    }

    fn deserialize_number(
        &self,
        scenario_name: &str,
        reserved_variable: ReservedVariable,
    ) -> JobResult<usize> {
        let v = get_json(&self.state, scenario_name)
            .and_then(|v| get_json(v, &reserved_variable.to_string()))
            .ok_or(JobError::VariableNotFound)?;
        v.as_str()
            .ok_or(JobError::VariableIncorrectFormat)?
            .parse()
            .map_err(|_| JobError::VariableIncorrectFormat)
    }

    fn deserialize_boolean(
        &self,
        scenario_name: &str,
        reserved_variable: ReservedVariable,
    ) -> JobResult<bool> {
        let v = get_json(&self.state, scenario_name)
            .and_then(|v| get_json(v, &reserved_variable.to_string()))
            .ok_or(JobError::VariableNotFound)?;
        // TODO they never return an error here if its not a bool?
        v.as_bool().ok_or(JobError::VariableIncorrectFormat)
    }

    fn deserialize_struct<T: DeserializeOwned>(
        &self,
        scenario_name: &str,
        reserved_variable: ReservedVariable,
    ) -> JobResult<T> {
        let v = get_json(&self.state, scenario_name)
            .and_then(|v| get_json(v, &reserved_variable.to_string()))
            .ok_or(JobError::VariableNotFound)?
            .clone();
        Self::deserialize_value(v)
    }

    /// returns a boolean indicating if a job is complete.
    pub fn check_complete(&self) -> bool {
        self.index > self.scenarios.len() - 1
    }

    fn get_broadcast_scenario(&self) -> JobResult<&Scenario> {
        if self.status == Status::Broadcasting {
            let broadcast_index = self
                .index
                .checked_sub(1)
                .ok_or(JobError::NoBroadcastToConfirm)?;
            Ok(&self.scenarios[broadcast_index])
        } else {
            Err(format!(
                "job is in {} state instead of {}: {}",
                self.state,
                Status::Broadcasting,
                JobError::JobInWrongState
            )
            .into())
        }
    }

    fn inject_key_and_mark_ready(
        &mut self,
        scenario_name: &str,
        key: ReservedVariable,
        obj: Value,
    ) -> JobResult<()> {
        let o = self.state.as_object_mut().unwrap();
        if !o.contains_key(scenario_name) {
            o.insert(scenario_name.into(), Value::Object(Default::default()));
        }

        o.get_mut(scenario_name)
            .unwrap()
            .as_object_mut()
            .ok_or_else(|| "failed to set a raw json value".to_string())?
            .insert(key.to_string(), obj);

        self.status = if self.check_complete() {
            Status::Completed
        } else {
            Status::Ready
        };

        Ok(())
    }

    /// called either after a broadcast has been confirmed at the provided confirmation depth or if it has failed for some reason.
    pub fn broadcast_complete(&mut self, transaction: Option<Transaction>) -> JobResult<()> {
        let scenario_name = self
            .get_broadcast_scenario()
            .map_err(|e| format!("failed to get broadcast scenario: %{e}"))?
            .name
            .clone();

        let transaction = if let Some(t) = transaction {
            t
        } else {
            self.status = Status::Failed;
            return Ok(());
        };

        self.inject_key_and_mark_ready(
            &scenario_name,
            ReservedVariable::Transaction,
            serde_json::to_value(transaction).unwrap(),
        )
        .map_err(|e| format!("unable to store transaction result: {e}").into())
    }

    /// invoked after a transaction dry run has been performed.
    pub fn dry_run_complete(&mut self, suggested_fee: &[Option<Amount>]) -> JobResult<()> {
        let scenario_name = self
            .get_broadcast_scenario()
            .map_err(|e| format!("unable to get broadcast scenario: {e}"))?
            .name
            .clone();

        self.inject_key_and_mark_ready(
            &scenario_name,
            ReservedVariable::SuggestedFee,
            serde_json::to_value(suggested_fee).unwrap(),
        )
        .map_err(|e| format!("unable to store suggested fee result: {e}").into())
    }
}
