mod errors;
mod populator;
#[cfg(test)]
mod populator_test;
mod types;
#[cfg(test)]
mod worker_test;

use indexmap::IndexSet;
use mentat_asserter::{
    account_array, account_identifier, amount, assert_unique_amounts, currency, public_key,
};
use mentat_storage::database::Transaction;
use mentat_types::{
    add_values, big_int, divide_values, hash, multiply_values, sub_values, AccountIdentifier,
    ConstructionDeriveResponse, UncheckedConstructionDeriveRequest,
};
use mentat_utils::utils::random_number;
use num_bigint_dig::{BigInt, Sign};
use rand::{thread_rng, Rng};
use rand_regex::Regex;
use reqwest::{header::HeaderValue, Client, Request, StatusCode};
use serde_json::{json, Value};
use types::Helper;
use url::Url;

use crate::{
    helpers::set_json,
    job::{
        Action, ActionType, Broadcast, FindBalanceInput, FindBalanceOutput,
        FindCurrencyAmountInput, GenerateKeyInput, GetBlobInput, HttpMethod, HttpRequestInput, Job,
        MathInput, MathOperation, RandomNumberInput, RandomStringInput, SaveAccountInput,
        SetBlobInput,
    },
    tmp::generate_key_pair,
};

use self::{
    errors::{VerboseWorkerError, VerboseWorkerResult, WorkerError, WorkerResult},
    populator::populate_input,
};

use std::{env, str::FromStr, time::Duration};

/// Worker processes jobs.
pub struct Worker<T: Helper>(pub T);

impl<T: Helper> Worker<T> {
    /// returns a new Worker.
    pub fn new(helper: T) -> Self {
        Self(helper)
    }

    async fn invoke_worker(
        &mut self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
        action: ActionType,
        input: Value,
    ) -> WorkerResult<Option<Value>> {
        match action {
            ActionType::SetVariable => Ok(Some(input)),
            ActionType::GenerateKey => generate_key_worker(input).map(Some),
            ActionType::Derive => self.derive_worker(input).map(Some),
            ActionType::SaveAccount => self.save_account_worker(db_tx, input).map(|_| None),
            ActionType::PrintMessage => {
                print_message_worker(&input);
                Ok(None)
            }
            ActionType::RandomString => random_string_worker(input).map(Some),
            ActionType::Math => math_worker(input).map(Some),
            ActionType::FindBalance => self.find_balance_worker(db_tx, input).map(Some),
            ActionType::RandomNumber => random_number_worker(input).map(Some),
            ActionType::Assert => assert_worker(input).map(|_| None),
            ActionType::FindCurrencyAmount => find_currency_amount_worker(input).map(Some),
            ActionType::LoadEnv => load_env_worker(input),
            ActionType::HttpRequest => http_request_worker(input).await.map(Some),
            ActionType::SetBlob => self.set_blob_worker(db_tx, input).map(|_| None),
            ActionType::GetBlob => self.get_blob_worker(db_tx, input).map(Some),
            ActionType::Unknown => Err(WorkerError::InvalidActionType),
        }
    }

    async fn actions(
        &mut self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
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
                .invoke_worker(db_tx, action.type_, processed_input.clone())
                .await
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
            set_json(
                &mut state,
                action.output_path.as_ref().unwrap(),
                output.clone(),
            )
            .map_err(|e| VerboseWorkerError {
                action_index: i,
                action: Some(action.clone()),
                processed_input: Some(processed_input),
                output: Some(output),
                state: Some(old_state),
                err: format!("unable to update state: {e}").into(),
                ..Default::default()
            })?;
        }

        Ok(state)
    }

    /// ProcessNextScenario performs the actions in the next available
    /// scenario.
    pub async fn process_next_scenario(
        &mut self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
        j: &mut Job,
    ) -> VerboseWorkerResult<()> {
        let scenario = &j.scenarios[j.index];
        let new_state = self
            .actions(db_tx, j.state.clone(), &scenario.actions)
            .await
            .map_err(|mut e| {
                // Set additional context not available within actions.
                e.workflow = j.workflow;
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
    pub async fn process(
        &mut self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
        j: &mut Job,
    ) -> VerboseWorkerResult<Option<Broadcast>> {
        if j.check_complete() {
            return Err(VerboseWorkerError {
                err: WorkerError::JobComplete,
                ..Default::default()
            });
        }

        self.process_next_scenario(db_tx, j).await?;

        j.create_broadcast().map_err(|e| VerboseWorkerError {
            workflow: j.workflow,
            job: Some(j.identifier.clone()),
            scenario: j.scenarios[j.index - 1].name.clone(),
            scenario_index: j.index - 1,
            err: format!("unable to create broadcast: {e}").into(),
            ..Default::default()
        })
    }

    /// DeriveWorker attempts to derive an account given a
    /// *types.ConstructionDeriveRequest input.
    pub fn derive_worker(&self, raw_input: Value) -> WorkerResult<Value> {
        let input = Job::deserialize_value::<UncheckedConstructionDeriveRequest>(raw_input)
            .map_err(|e| format!("failed to deserialize input: {e}"))?;

        public_key(input.public_key.as_ref())
            .map_err(|e| format!("public key {:?} is invalid: {e}", input.public_key))?;

        let (account_identifier, metadata) = self
            .0
            .derive(
                &input.network_identifier.unwrap(),
                &input.public_key.unwrap().into(),
                input.metadata,
            )
            .map_err(|e| format!("failed to derive account identifier: {e}"))?;

        Ok(json!(&ConstructionDeriveResponse {
            address: None,
            account_identifier,
            metadata,
        }))
    }

    /// SaveAccountWorker saves a *types.AccountIdentifier and associated KeyPair
    /// in KeyStorage.
    pub fn save_account_worker(
        &mut self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
        raw_input: Value,
    ) -> WorkerResult<()> {
        let input = Job::deserialize_value::<SaveAccountInput>(raw_input)
            .map_err(|e| format!("failed to deserialize input {e}"))?;
        account_identifier(input.account_identifier.as_ref()).map_err(|e| {
            format!(
                "account identifier {:?} is invalid: {e}",
                input.account_identifier
            )
        })?;
        self.0
            .store_key(
                #[cfg(not(test))]
                db_tx,
                #[cfg(test)]
                db_tx.clone(),
                &input.account_identifier.unwrap(),
                // TODO no check nil
                input.key_pair.as_ref().unwrap(),
            )
            .map_err(|e| format!("failed to store key: {e}").into())
    }

    fn check_account_coins(
        &self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
        input: &FindBalanceInput,
        account: &AccountIdentifier,
    ) -> WorkerResult<Option<Value>> {
        let minimum_balance = input.minimum_balance.as_ref().unwrap();
        let coins = self
            .0
            .coins(
                #[cfg(not(test))]
                db_tx,
                #[cfg(test)]
                db_tx.clone(),
                account,
                &minimum_balance.currency.clone().unwrap().into(),
            )
            .map_err(|e| {
                format!("failed to return coins of account identifier {account:?} in currency {:?}: {e}", minimum_balance.currency)
            })?;

        for coin in coins {
            if input.not_coins.contains(&coin.coin_identifier) {
                continue;
            }

            // TODO all the logic below this could just be a normal comparison
            let diff = sub_values(&coin.amount.value, &minimum_balance.value).map_err(|e| {
                format!(
                    "failed to subtract values {} - {}: {e}",
                    coin.amount.value, minimum_balance.value
                )
            })?;

            let big_int_dif = BigInt::from_str(&diff)
                .map_err(|e| format!("failed to convert string {diff} to big int: {e}"))?;

            if big_int_dif.sign() == Sign::Minus {
                continue;
            } else {
                return Ok(Some(json!(FindBalanceOutput {
                    account_identifier: Some(account.clone()),
                    balance: Some(coin.amount.clone()),
                    coin: Some(coin.coin_identifier.clone())
                })));
            }
        }

        Ok(None)
    }

    fn check_account_balance(
        &self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
        input: &FindBalanceInput,
        account: &AccountIdentifier,
    ) -> WorkerResult<Option<Value>> {
        let minimum_balance = input.minimum_balance.as_ref().unwrap();
        let amount = self
            .0
            .balance(
                #[cfg(not(test))]
                db_tx,
                #[cfg(test)]
                db_tx.clone(),
                account,
                &minimum_balance.currency.clone().unwrap().into()
            )
            .map_err(|e| {
                format!(
                    "failed to return balance of account identifier {account:?} in currency {:?}: {e}", minimum_balance.currency
                )
            })?;

        // TODO all the logic below this could just be a normal comparison
        // look for amounts > min
        let diff = sub_values(&amount.value, &minimum_balance.value).map_err(|e| {
            format!(
                "failed to subtract values {} - {}: {e}",
                amount.value, minimum_balance.value
            )
        })?;

        let big_int_dif = BigInt::from_str(&diff)
            .map_err(|e| format!("failed to convert string {diff} to big int: {e}"))?;

        if big_int_dif.sign() == Sign::Minus {
            println!("check_account_balance: Account ({}) has balance ({}), less than the minimum balance ({})", account.address, amount.value, minimum_balance.value);
            return Ok(None);
        }

        Ok(Some(json!(FindBalanceOutput {
            account_identifier: Some(account.clone()),
            balance: Some(amount.clone()),
            coin: None
        })))
    }

    fn available_accounts(
        &self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
    ) -> WorkerResult<(&[AccountIdentifier], Vec<&AccountIdentifier>)> {
        let accounts = self
            .0
            .all_accounts(
                #[cfg(not(test))]
                db_tx,
                #[cfg(test)]
                db_tx.clone(),
            )
            .map_err(|e| format!("unable to get all accounts: {e}"))?;

        // If there are no accounts, we should create one.
        if accounts.is_empty() {
            return Err(WorkerError::CreateAccount);
        }

        // We fetch all locked accounts to subtract them from AllAccounts.
        // We consider an account "locked" if it is actively involved in a broadcast.
        let locked_accounts = self
            .0
            .locked_accounts(
                #[cfg(not(test))]
                db_tx,
                #[cfg(test)]
                db_tx.clone(),
            )
            .map_err(|e| format!("unable to get locked accounts: {e}"))?;

        // Convert to a map so can do fast lookups
        let locked_set = IndexSet::<_>::from_iter(locked_accounts.iter().map(|a| hash(Some(a))));

        let unlocked_accounts = accounts
            .iter()
            .filter(|a| !locked_set.contains(&hash(Some(*a))))
            .collect::<Vec<_>>();

        Ok((accounts, unlocked_accounts))
    }

    /// FindBalanceWorker attempts to find an account (and coin) with some minimum
    /// balance in a particular currency.
    pub fn find_balance_worker(
        &self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
        raw_input: Value,
    ) -> WorkerResult<Value> {
        let input = Job::deserialize_value::<FindBalanceInput>(raw_input)
            .map_err(|e| format!("failed to deserialize input {e}"))?;

        // Validate that input is properly formatted
        find_balance_worker_input_validation(&input)
            .map_err(|e| format!("failed to validate the input of find balance worker: {e}"))?;

        println!("{input}");

        let (accounts, available_accounts) = self
            .available_accounts(db_tx)
            .map_err(|e| format!("unable to get available accounts: {e}"))?;

        // Randomly, we choose to generate a new account. If we didn't do this,
        // we would never grow past 2 accounts for mocking transfers.
        if should_create_random_account(&input, accounts.len()) {
            Err(WorkerError::CreateAccount)?;
        }

        let mut unmatched_accounts = Vec::new();
        // Consider each available account as a potential account.
        for account in &available_accounts {
            if skip_account(&input, account) {
                continue;
            }

            let output = if input.require_coin {
                self.check_account_coins(db_tx, &input, account)
            } else {
                self.check_account_balance(db_tx, &input, account)
            }
            .map_err(|e| format!("failed to check account coins or balance: {e}"))?;

            if let Some(o) = output {
                return Ok(o);
            } else {
                // If we did not fund a match, we should continue.
                unmatched_accounts.push(&account.address)
            }
        }

        if !unmatched_accounts.is_empty() {
            println!(
                "{}: account(s) insufficiently funded. Please fund the address {unmatched_accounts:?}",
                unmatched_accounts.len(),
            )
        }

        if input.minimum_balance.as_ref().unwrap().value != "0" {
            // If we can't do anything, we should return with ErrUnsatisfiable.
            Err(WorkerError::Unsatisfiable)
        } else if input.create_limit > 0 && accounts.len() < input.create_limit as usize {
            // If we should create an account and the number of accounts
            // we have is less than the limit, we return ErrCreateAccount.
            Err(WorkerError::CreateAccount)
        } else {
            // If we reach here, it means we shouldn't create another account
            // and should just return unsatisfiable.
            Err(WorkerError::Unsatisfiable)
        }
    }

    /// SetBlobWorker transactionally saves a key and value for use
    /// across workflows.
    pub fn set_blob_worker(
        &self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
        raw_input: Value,
    ) -> WorkerResult<()> {
        let input = Job::deserialize_value::<SetBlobInput>(raw_input)
            .map_err(|e| format!("failed to deserialize input {e}"))?;

        // TODO may be doing this wrong
        // By using Value for key, we can ensure that JSON
        // objects with the same keys but in a different order are
        // treated as equal.
        self.0
            .set_blob(
                #[cfg(not(test))]
                db_tx,
                #[cfg(test)]
                db_tx.clone(),
                input.key,
                input.value,
            )
            .map_err(|e| format!("failed to set blob: {}", e).into())
    }

    /// GetBlobWorker transactionally retrieves a value associated with
    /// a key, if it exists.
    pub fn get_blob_worker(
        &self,
        #[cfg(not(test))] db_tx: &impl Transaction,
        #[cfg(test)] db_tx: &(impl Transaction + Clone + 'static),
        raw_input: Value,
    ) -> WorkerResult<Value> {
        let input = Job::deserialize_value::<GetBlobInput>(raw_input)
            .map_err(|e| format!("failed to deserialize input {e}"))?;

        // TODO may be doing this wrong
        // By using Value for key, we can ensure that JSON
        // objects with the same keys but in a different order are
        // treated as equal.
        self.0
            .get_blob(
                #[cfg(not(test))]
                db_tx,
                #[cfg(test)]
                db_tx.clone(),
                &input.key,
            )
            .map_err(|e| format!("failed to get blob: {e}"))?
            .ok_or_else(|| {
                format!(
                    "key {} does not exist: {}",
                    input.key,
                    WorkerError::ActionFailed
                )
                .into()
            })
    }
}

/// GenerateKeyWorker attempts to generate a key given a
/// *GenerateKeyInput input.
pub fn generate_key_worker(raw_input: Value) -> WorkerResult<Value> {
    let input = Job::deserialize_value::<GenerateKeyInput>(raw_input)
        .map_err(|e| format!("failed to deserialize input: {e}"))?;
    let kp = generate_key_pair(&input.curve_type)
        .map_err(|e| format!("failed to generate key pair: {e}"))?;
    Ok(json!(kp))
}

/// PrintMessageWorker logs some message to stdout.
pub fn print_message_worker(message: &Value) {
    println!("Message: {message}")
}

/// RandomStringWorker generates a string that complies
/// with the provided regex input.
pub fn random_string_worker(raw_input: Value) -> WorkerResult<Value> {
    let input = Job::deserialize_value::<RandomStringInput>(raw_input)
        .map_err(|e| format!("failed to deserialize input {e}"))?;

    let reg = Regex::compile(&input.regex, input.limit)
        .map_err(|e| format!("failed to generate a string with the provided regex input: {e}"))?;
    let output: String = thread_rng().sample(&reg);

    Ok(Value::String(output))
}

/// MathWorker performs some MathOperation on 2 numbers.
pub fn math_worker(raw_input: Value) -> WorkerResult<Value> {
    let input = Job::deserialize_value::<MathInput>(raw_input)
        .map_err(|e| format!("failed to deserialize input: {e}"))?;

    let result = match input.operation {
        MathOperation::Addition => add_values(&input.left_value, &input.right_value),
        MathOperation::Subtraction => sub_values(&input.left_value, &input.right_value),
        MathOperation::Multiplication => multiply_values(&input.left_value, &input.right_value),
        MathOperation::Division => divide_values(&input.left_value, &input.right_value),
        MathOperation::Unknown => {
            // TODO: fallback enum variant doesnt retain the invalid text
            return Err(format!(
                "math operation UNKNOWN is invalid: {}",
                WorkerError::InputOperationIsNotSupported
            )
            .into());
        }
    }
    .map_err(|e| format!("failed to perform math operation: {e}"))?;

    Ok(Value::String(result))
}

/// RandomNumberWorker generates a random number in the range
/// [minimum,maximum).
pub fn random_number_worker(raw_input: Value) -> WorkerResult<Value> {
    let input = Job::deserialize_value::<RandomNumberInput>(raw_input)
        .map_err(|e| format!("failed to deserialize input {e}"))?;

    let min = big_int(&input.minimum)
        .map_err(|e| format!("failed to convert string {} to big int: {e}", input.minimum))?;
    let max = big_int(&input.maximum)
        .map_err(|e| format!("failed to convert string {} to big int: {e}", input.maximum))?;

    let rand_num = random_number(&min, &max)
        .map_err(|e| format!("failed to return random number in [{min}-{max}]: {e}"))?;

    Ok(Value::String(rand_num.to_string()))
}

/// FindCurrencyAmountWorker finds a *types.Amount with a specific
/// *types.Currency in a []*types.Amount.
pub fn find_currency_amount_worker(raw_input: Value) -> WorkerResult<Value> {
    let input = Job::deserialize_value::<FindCurrencyAmountInput>(raw_input)
        .map_err(|e| format!("failed to deserialize input {e}"))?;

    currency(input.currency.as_ref())
        .map_err(|e| format!("currency {:?} is invalid: {e}", input.currency))?;

    assert_unique_amounts(&input.amounts)
        .map_err(|e| format!("amount {:?} is invalid: {e}", input.amounts))?;

    input
        .amounts
        .iter()
        .flatten()
        .find(|amount| amount.currency == input.currency)
        .map(|a| json!(a))
        .ok_or_else(|| {
            format!(
                "unable to find currency {:?}: {}",
                input.currency,
                WorkerError::ActionFailed
            )
            .into()
        })
}

/// LoadEnvWorker loads an environment variable and stores
/// it in state. This is useful for algorithmic fauceting.
pub fn load_env_worker(raw_input: Value) -> WorkerResult<Option<Value>> {
    // todo: is this needed??
    // We deserialize the input here to handle string
    // unwrapping automatically.
    let input = Job::deserialize_value::<String>(raw_input)
        .map_err(|e| format!("failed to deserialize input {e}"))?;

    Ok(env::var(input).ok().map(|v| json!(v)))
}

/// HTTPRequestWorker makes an HTTP request and returns the response to
/// store in a variable. This is useful for algorithmic fauceting.
pub async fn http_request_worker(raw_input: Value) -> WorkerResult<Value> {
    let input = Job::deserialize_value::<HttpRequestInput>(raw_input)
        .map_err(|e| format!("failed to deserialize input {e}"))?;

    let url = Url::parse(&input.url)
        .map_err(|e| format!("failed to parse request URI {}: {e}", input.url))?;

    let client = Client::builder()
        .timeout(Duration::from_secs(input.timeout.try_into().map_err(
            |_| {
                format!(
                    "{} is not a valid timeout: {}",
                    input.timeout,
                    WorkerError::InvalidInput
                )
            },
        )?))
        .build()
        .unwrap();

    let mut request = Request::new(input.method.into(), url);
    request
        .headers_mut()
        .append("Accept", HeaderValue::from_static("application/json"));
    if input.method == HttpMethod::Post {
        *request.body_mut() = Some(input.body.to_string().into());
        request
            .headers_mut()
            .insert("Content-Type", HeaderValue::from_static("application/json"));
    }

    let resp = client
        .execute(request)
        .await
        .map_err(|e| format!("failed to send request: {e}"))?;

    let status = resp.status();
    let body = resp
        .json::<Value>()
        .await
        .map_err(|e| format!("failed to read response: {e}"))?;

    if status != StatusCode::OK {
        Err(format!(
            "status code  {} with body {}: {}",
            status,
            body,
            WorkerError::ActionFailed
        )
        .into())
    } else {
        Ok(body)
    }
}

fn should_create_random_account(input: &FindBalanceInput, account_count: usize) -> bool {
    !(input.minimum_balance.as_ref().unwrap().value != "0"
        || input.create_limit <= 0
        || account_count >= input.create_limit as usize)
        && thread_rng().gen_ratio(input.create_probability, 100)
}

/// findBalanceWorkerInputValidation ensures the input to FindBalanceWorker
/// is valid.
pub fn find_balance_worker_input_validation(input: &FindBalanceInput) -> WorkerResult<()> {
    amount(input.minimum_balance.as_ref()).map_err(|e| {
        format!(
            "minimum balance {:?} is invalid: {e}",
            input.minimum_balance
        )
    })?;

    if let Some(id) = &input.account_identifier {
        account_identifier(Some(id))
            .map_err(|e| format!("account identifier {id:?} is invalid: {e}"))?;
        if input.sub_account_identifier.is_some() {
            Err("cannot populate both account and sub account")?;
        } else if input.not_account_identifier.is_empty() {
            Err("cannot populate both account and not accounts")?;
        } else if input.not_address.is_empty() {
            Err("cannot populate both account and not addresses")?;
        }
    }

    if !input.not_account_identifier.is_empty() {
        account_array("not account identifier", &input.not_account_identifier).map_err(|e| {
            format!(
                "account identifiers of not account identifier {:?} are invalid: {e}",
                input.not_account_identifier
            )
        })?;
    }

    Ok(())
}

fn skip_account(input: &FindBalanceInput, account: &AccountIdentifier) -> bool {
    // TODO: cloning shouldnt be needed here
    // If we require an account and that account
    // is not equal to the account we are considering,
    // we should continue.
    matches!(&input.account_identifier, Some(id) if id != account)
        // If we specify not to use certain addresses and we are considering
        // one of them, we should continue.
        || input.not_address.contains(&account.address)
        // If we specify that we do not use certain accounts
        // and the account we are considering is one of them,
        // we should continue.
        || input.not_account_identifier.contains(&Some(account.clone()))
        // If we require a particular SubAccountIdentifier, we skip
        // if the account we are examining does not have it.
        || matches!(&input.sub_account_identifier, Some(id) if account.sub_account.is_none() || account.sub_account == Some(id.clone()))
}

/// AssertWorker checks if an input is < 0.
pub fn assert_worker(raw_input: Value) -> WorkerResult<()> {
    // todo: is this needed??
    // We deserialize the input here to handle string
    // unwrapping automatically.
    let input = Job::deserialize_value::<String>(raw_input)
        .map_err(|e| format!("failed to deserialize input {e}"))?;

    let val = BigInt::from_str(&input)
        .map_err(|e| format!("failed to convert the string {input} to big int: {e}"))?;

    if val.sign() == Sign::Minus {
        Err(format!("{val} < 0: {}", WorkerError::ActionFailed).into())
    } else {
        Ok(())
    }
}
