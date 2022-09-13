mod errors;
mod populator;
#[cfg(test)]
mod populator_test;
mod types;

use mentat_types::AccountIdentifier;
use serde_json::Value;
use types::Helper;

use crate::{
    job::{Action, ActionType, Broadcast, FindBalanceInput, Job},
    tmp::Transaction,
};

use self::{
    errors::{VerboseWorkerError, VerboseWorkerResult, WorkerResult},
    populator::{populate_input, set_json},
};

/// Worker processes jobs.
pub struct Worker<T: Helper>(T);

impl<T: Helper> Worker<T> {
    /// returns a new Worker.
    pub fn new(helper: T) -> Self {
        Self(helper)
    }

    fn serialize_string(value: &str) -> String {
        format!("\"{value}\"")
    }

    fn invoke_worker(
        &self,
        db_tx: &impl Transaction,
        action: ActionType,
        input: &Value,
    ) -> WorkerResult<Option<Value>> {
        match action {
            ActionType::SetVariable => Ok(Some(input.clone())),
            ActionType::GenerateKey => Self::generate_key_worker(input).map(Some),
            ActionType::Derive => self.derive_worker(input).map(Some),
            ActionType::SaveAccount => self.save_account_worker(db_tx, input).map(|_| None),
            ActionType::PrintMessage => {
                Self::print_message_worker(input);
                Ok(None)
            }
            ActionType::RandomString => Self::random_string_worker(input).map(Some),
            ActionType::Math => Self::math_worker(input).map(Some),
            ActionType::FindBalance => self.find_balance_worker(db_tx, input).map(Some),
            ActionType::RandomNumber => Self::random_number_worker(input).map(Some),
            ActionType::Assert => Self::assert_worker(input).map(|_| None),
            ActionType::FindCurrencyAmount => Self::find_currency_amount_worker(input).map(Some),
            ActionType::LoadEnv => Self::load_env_worker(input).map(Some),
            ActionType::HttpRequest => Self::http_request_worker(input).map(Some),
            ActionType::SetBlob => self.set_blob_worker(db_tx, input).map(|_| None),
            ActionType::GetBlob => self.get_blob_worker(db_tx, input).map(Some),
        }
    }

    fn actions(
        &self,
        db_tx: &impl Transaction,
        mut state: Value,
        actions: &[Action],
    ) -> VerboseWorkerResult<Value> {
        for (i, action) in actions.iter().enumerate() {
            let processed_input =
                populate_input(&state, &action.input).map_err(|e| VerboseWorkerError {
                    action_index: i,
                    action: Some(action.clone()),
                    state: Some(state.clone()),
                    err: format!("unable to populate variables: {e}").into(),
                    ..Default::default()
                })?;

            let output = self
                .invoke_worker(db_tx, action.type_, &processed_input)
                .map_err(|e| VerboseWorkerError {
                    action_index: i,
                    action: Some(action.clone()),
                    processed_input: Some(processed_input.clone()),
                    state: Some(state.clone()),
                    err: format!("unable ot process action: {e}").into(),
                    ..Default::default()
                })?;

            let output = if let Some(o) = output { o } else { continue };

            // Update state at the specified output path if there is an output.
            let old_state = state.clone();
            set_json(&mut state, &action.output_path, output.clone()).map_err(|e| {
                VerboseWorkerError {
                    action_index: i,
                    action: Some(action.clone()),
                    processed_input: Some(processed_input),
                    output: Some(output),
                    state: Some(old_state),
                    err: format!("unable to update state: {e}").into(),
                    ..Default::default()
                }
            })?;
        }

        Ok(state)
    }

    /// ProcessNextScenario performs the actions in the next available
    /// scenario.
    pub fn process_next_scenario(
        &self,
        db_tx: &impl Transaction,
        j: &mut Job,
    ) -> VerboseWorkerResult<()> {
        let scenario = &j.scenarios[j.index];
        let new_state = self
            .actions(db_tx, j.state.clone(), &scenario.actions)
            .map_err(|mut e| {
                // Set additional context not available within actions.
                e.workflow = j.workflow.clone();
                e.job = Some(j.identifier.clone());
                e.scenario = scenario.name.clone();
                e.scenario_index = j.index;
                e
            })?;

        j.state = new_state;
        j.index += 1;
        Ok(())
    }

    /// Process is called on a Job to execute
    /// the next available scenario. If no scenarios
    /// are remaining, this will return an error.
    pub fn process(
        &self,
        db_tx: &impl Transaction,
        j: Option<Job>,
    ) -> VerboseWorkerResult<Option<Broadcast>> {
        todo!()
    }

    /// DeriveWorker attempts to derive an account given a
    /// *types.ConstructionDeriveRequest input.
    pub fn derive_worker(&self, raw_input: &Value) -> WorkerResult<Value> {
        todo!()
    }

    /// GenerateKeyWorker attempts to generate a key given a
    /// *GenerateKeyInput input.
    pub fn generate_key_worker(raw_input: &Value) -> WorkerResult<Value> {
        todo!()
    }

    /// SaveAccountWorker saves a *types.AccountIdentifier and associated KeyPair
    /// in KeyStorage.
    pub fn save_account_worker(
        &self,
        db_tx: &impl Transaction,
        raw_input: &Value,
    ) -> WorkerResult<()> {
        todo!()
    }

    /// PrintMessageWorker logs some message to stdout.
    pub fn print_message_worker(message: &Value) {
        todo!()
    }

    /// RandomStringWorker generates a string that complies
    /// with the provided regex input.
    pub fn random_string_worker(raw_input: &Value) -> WorkerResult<Value> {
        todo!()
    }

    /// MathWorker performs some MathOperation on 2 numbers.
    pub fn math_worker(raw_input: &Value) -> WorkerResult<Value> {
        todo!()
    }

    /// RandomNumberWorker generates a random number in the range
    /// [minimum,maximum).
    pub fn random_number_worker(raw_input: &Value) -> WorkerResult<Value> {
        todo!()
    }

    /// balanceMessage prints out a log message while waiting
    /// that reflects the *FindBalanceInput.
    pub fn balance_message(input: Option<&FindBalanceInput>) -> String {
        todo!()
    }

    fn check_account_coins(
        &self,
        db_tx: &impl Transaction,
        input: Option<&FindBalanceInput>,
        account: Option<&AccountIdentifier>,
    ) -> WorkerResult<Value> {
        todo!()
    }

    fn check_account_balance(
        &self,
        db_tx: &impl Transaction,
        input: Option<&FindBalanceInput>,
        account: Option<&AccountIdentifier>,
    ) -> WorkerResult<Value> {
        todo!()
    }

    fn available_accounts(
        &self,
        db_tx: &impl Transaction,
    ) -> WorkerResult<(Vec<AccountIdentifier>, Vec<AccountIdentifier>)> {
        todo!()
    }

    fn should_create_random_account(
        input: Option<&FindBalanceInput>,
        account_count: usize,
    ) -> WorkerResult<bool> {
        todo!()
    }

    /// findBalanceWorkerInputValidation ensures the input to FindBalanceWorker
    /// is valid.
    pub fn find_balance_worker_input_validation(
        input: Option<&FindBalanceInput>,
    ) -> WorkerResult<()> {
        todo!()
    }

    fn skip_account(
        input: FindBalanceInput,
        account: Option<&AccountIdentifier>,
    ) -> WorkerResult<bool> {
        todo!()
    }

    /// FindBalanceWorker attempts to find an account (and coin) with some minimum
    /// balance in a particular currency.
    pub fn find_balance_worker(
        &self,
        db_tx: &impl Transaction,
        raw_input: &Value,
    ) -> WorkerResult<Value> {
        todo!()
    }

    /// AssertWorker checks if an input is < 0.
    pub fn assert_worker(raw_input: &Value) -> WorkerResult<()> {
        todo!()
    }

    /// FindCurrencyAmountWorker finds a *types.Amount with a specific
    /// *types.Currency in a []*types.Amount.
    pub fn find_currency_amount_worker(raw_input: &Value) -> WorkerResult<Value> {
        todo!()
    }

    /// LoadEnvWorker loads an environment variable and stores
    /// it in state. This is useful for algorithmic fauceting.
    pub fn load_env_worker(raw_input: &Value) -> WorkerResult<Value> {
        todo!()
    }

    /// HTTPRequestWorker makes an HTTP request and returns the response to
    /// store in a variable. This is useful for algorithmic fauceting.
    pub fn http_request_worker(raw_input: &Value) -> WorkerResult<Value> {
        todo!()
    }

    /// SetBlobWorker transactionally saves a key and value for use
    /// across workflows.
    pub fn set_blob_worker(&self, db_tx: &impl Transaction, raw_input: &Value) -> WorkerResult<()> {
        todo!()
    }

    /// GetBlobWorker transactionally retrieves a value associated with
    /// a key, if it exists.
    pub fn get_blob_worker(
        &self,
        db_tx: &impl Transaction,
        raw_input: &Value,
    ) -> WorkerResult<Value> {
        todo!()
    }
}
