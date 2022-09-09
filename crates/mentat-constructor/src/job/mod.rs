mod errors;
pub use errors::*;
mod types;
use mentat_types::{Amount, Transaction};
use mentat_utils::rust_utils::Context;
pub use types::*;

use bincode::deserialize;
use num_bigint_dig::BigInt;
use serde::de::DeserializeOwned;

impl Job {
    /// creates a new Job.
    pub fn new(workflow: Workflow) -> Self {
        Self {
            identifier: Default::default(),
            state: Default::default(),
            index: Default::default(),
            status: Status::Ready,
            workflow: workflow.name,
            scenarios: workflow.scenarios,
        }
    }

    /// returns a Broadcast for a given job or nil if none is required.
    pub fn create_broadcast(&self) -> JobResult<Option<Broadcast>> {
        todo!()
    }

    /// attempts to strictly deserialize some input into output.
    pub fn deserialize_input<T: DeserializeOwned>(input: &[u8]) -> JobResult<T> {
        deserialize(input).map_err(|e| format!("unable to decode: {e}").into())
    }

    fn deserialize_number(
        &self,
        scenario_name: &str,
        reserved_variable: &ReservedVariable,
    ) -> JobResult<BigInt> {
        todo!()
    }

    fn deserialize_boolean(
        &self,
        scenario_name: &str,
        reserved_variable: &ReservedVariable,
    ) -> JobResult<bool> {
        todo!()
    }

    fn deserialize_struct<T: DeserializeOwned>(
        &self,
        scenario_name: &str,
        reserved_variable: &ReservedVariable,
    ) -> JobResult<T> {
        todo!()
    }

    /// returns a boolean indicating if a job is complete.
    pub fn check_complete(&self) -> bool {
        todo!()
    }

    fn get_broadcast_scenario(&self) -> JobResult<Scenario> {
        todo!()
    }

    fn inject_key_and_mark_ready(
        &self,
        scenario_name: &str,
        key: &ReservedVariable,
        obj: &str,
    ) -> JobResult<()> {
        todo!()
    }

    /// called either after a broadcast has been confirmed at the provided confirmation depth or if it has failed for some reason.
    pub fn broadcast_complete(
        &self,
        ctx: &Context<JobError>,
        transaction: Option<Transaction>,
    ) -> JobResult<()> {
        todo!()
    }

    /// invoked after a transaction dry run has been performed.
    pub fn dry_run_complete(
        &self,
        ctx: &Context<JobError>,
        suggested_fee: &[Option<Amount>],
    ) -> JobResult<()> {
        todo!()
    }
}
