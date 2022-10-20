//! TODO: this is not complete!
// TODO remove this lint once function return types are fully filled out
#![allow(clippy::all)]
use std::{
    env::temp_dir,
    fmt::Debug,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    time::Duration,
};

use mentat_types::{
    hash, AccountIdentifier, Amount, BlockIdentifier, BlockResponse, Coin, Currency, Metadata,
    NetworkIdentifier, NetworkListResponse, NetworkStatusResponse, PartialBlockIdentifier,
    Sortable,
};
use num_bigint_dig::{BigInt, RandBigInt};
use rand::{thread_rng, Rng};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::rust_utils::Context;

/// specifies that the user can
/// read and write the file.
pub const DEFAULT_FILE_PERMISSIONS: usize = 0o600;

/// specifies anyone can do anything
/// to the file.
pub const ALL_FILE_PERMISSIONS: usize = 0o777;

pub const BASE_10: usize = 10;

pub const BIG_BLOAT_PRECISION: usize = 512;

/// the number
/// of nanoseconds in a millisecond.
pub const NANOSECONDS_IN_MILLISECOND: usize = 1000000;

/// the number
/// of milliseconds in a second.
pub const MILLISECOND_IN_SECOND: usize = 1000;

/// the number 100.
pub const ONE_HUNDRED: usize = 100;

/// the minimum blocks per second
/// to consider when estimating time to tip if the provided
/// estimate is 0.
pub const MIN_BLOCKS_PER_SECOND: f64 = 0.0001;

/// the maximum number of entries
/// in one transaction object. This is used for bootstrap
/// balances process to avoid TxnTooBig error when memory_limit_disabled=false
/// as well as reduce the running time.
pub const MAX_ENTRY_SIZE_PER_TXN: usize = 600;

/// a BigInt of value 100.
pub fn one_hundred_int() -> BigInt {
    BigInt::from(100)
}

/// a BigInt of value 0.
pub fn zero_int() -> BigInt {
    BigInt::default()
}

#[derive(Debug, Error)]
pub enum UtilsError {
    /// returned when the network
    /// you are attempting to connect to is not supported.
    #[error("network not supported")]
    NetworkNotSupported,
    #[error("{0}: unable to create data and network directory")]
    CreatePathError(io::Error),
    #[error("{0}: unable to write to file path {1}")]
    WriteError(io::Error, PathBuf),
    #[error("{0}: unable to load file {1}")]
    ReadError(io::Error, PathBuf),
    #[error("{0}: unable to deserialize")]
    DeserializeError(serde_json::Error),
    #[error("{0}: cannot populate path")]
    CannotPopulatePath(Box<UtilsError>),
}

/// creates a directory in
/// /tmp for usage within testing.
pub fn create_temp_dir() -> PathBuf {
    let storage_dir = temp_dir();
    println!(
        "\x1b[36mUsing temporary directory {}\x1b[0m",
        storage_dir.display()
    );
    storage_dir
}

// deletes a directory at
// a provided path for usage within testing.
pub fn remove_temp_dir(path: &Path) {
    println!(
        "\x1b[33Removing temporary directory {}\x1b[0m",
        path.display()
    );
    remove_dir_all(path).unwrap();
}

/// creates directories along
/// a path if they do not exist.
pub fn ensure_path_exists(path: &Path) -> Result<(), UtilsError> {
    create_dir_all(path).map_err(UtilsError::CreatePathError)
}

///  returns a boolean indicating if two
/// types are equal.
pub fn equal<A, B>(a: Option<&A>, b: Option<&B>) -> bool
where
    A: Sortable + Debug + Serialize,
    B: Sortable + Debug + Serialize,
{
    hash(a) == hash(b)
}

/// attempts to serialize the provided object
/// into a file at filePath.
pub fn serialize_and_write(path: &Path, object: &impl Serialize) -> Result<(), UtilsError> {
    let v = serde_json::to_string_pretty(object).unwrap();
    fs::write(path, v).map_err(|e| UtilsError::WriteError(e, path.to_path_buf()))
}

/// reads the file at the provided path
/// and attempts to deserialize it into output.
pub fn load_and_parse<T: DeserializeOwned>(path: &Path) -> Result<T, UtilsError> {
    let b = fs::read_to_string(path).map_err(|e| UtilsError::ReadError(e, path.to_path_buf()))?;
    serde_json::from_str(&b).map_err(UtilsError::DeserializeError)
}

/// CreateCommandPath creates a unique path for a command and network within a
/// data directory. This is used to avoid collision when using multiple commands
/// on multiple networks when the same storage resources are used. If the
/// derived path does not exist, we run os.MkdirAll on the path.
pub fn create_command_path(
    data_dir: &Path,
    cmd: String,
    network: Option<&NetworkIdentifier>,
) -> Result<PathBuf, UtilsError> {
    let data_path = data_dir.join(cmd).join(hash(network));
    ensure_path_exists(&data_path).map_err(|e| UtilsError::CannotPopulatePath(Box::new(e)))?;
    Ok(data_path)
}

// TODO these functions are missing error types
/// FetcherHelper is used by util functions to mock Fetcher
pub trait FetcherHelper<T: Clone> {
    fn network_list(
        &self,
        ctx: &Context<()>,
        metadata: &Metadata,
    ) -> Result<NetworkListResponse, ()>;

    fn network_status_retry(
        &self,
        ctx: &Context<()>,
        network: Option<&NetworkIdentifier>,
        metadata: &Metadata,
    ) -> Result<NetworkStatusResponse, ()>;

    fn account_balance_retry(
        &self,
        ctx: &Context<()>,
        network: Option<&NetworkIdentifier>,
        account: Option<&AccountIdentifier>,
        block: Option<&PartialBlockIdentifier>,
        currencies: &[Option<&Currency>],
    ) -> Result<(BlockIdentifier, Amount, Metadata), ()>;

    fn account_coins_retry(
        &self,
        ctx: &Context<()>,
        network: Option<&NetworkIdentifier>,
        acct: Option<&AccountIdentifier>,
        include_mempool: bool,
        currencies: &[Option<&Currency>],
    ) -> Result<(BlockIdentifier, Coin, Metadata), ()>;

    /// CheckNetworkTip returns a boolean indicating if the block returned by
    /// network/status is at tip. It also returns the block identifier
    /// returned by network/status.
    /// Note that the tipDelay param takes tip delay in seconds.
    /// Block returned by network/status is considered to be at tip if one of
    /// the following two conditions is met:
    /// (1) the block was produced within tipDelay of current time
    /// (i.e. block timestamp >= current time - tipDelay)
    /// (2) the network/status endpoint returns a SyncStatus with Synced = true.
    fn check_network_tip(
        &self,
        ctx: &Context<()>,
        network: Option<&NetworkIdentifier>,
        tip_delay: usize,
    ) -> Result<(bool, BlockIdentifier), ()> {
        todo!()
    }

    /// CheckStorageTip returns a boolean indicating if the current
    /// block returned by block storage helper is at tip. It also
    /// returns the block identifier of the current storage block.
    /// Note that the tipDelay param takes tip delay in seconds.
    /// A block in storage is considered to be at tip if one of the
    /// following two conditions is met:
    /// (1) the block was produced within tipDelay of current time
    /// (i.e. block timestamp >= current time - tipDelay)
    /// (2) CheckNetworkTip returns true and the block it returns
    /// is same as the current block in storage
    fn check_storage_tip(
        &self,
        ctx: &Context<()>,
        network: Option<&NetworkIdentifier>,
        tip_delay: usize,
        b: impl BlockStorageHelper,
    ) -> Result<(bool, BlockIdentifier), ()> {
        todo!()
    }

    /// CheckNetworkSupported checks if a Rosetta implementation supports a
    /// given *types.NetworkIdentifier. If it does, the current network
    /// status is returned.
    fn check_network_supported(
        &self,
        ctx: &Context<()>,
        network_identifier: Option<&NetworkIdentifier>,
    ) -> Result<NetworkStatusResponse, ()> {
        todo!()
    }

    /// CurrencyBalance returns the balance of an account
    /// for a particular currency at a particular height.
    /// It is up to the caller to determine if the retrieved
    /// block has the expected hash for the requested index.
    fn currency_balance(
        &self,
        ctx: &Context<()>,
        network: Option<&NetworkIdentifier>,
        account: Option<&AccountIdentifier>,
        currency: Option<&Currency>,
        index: usize,
    ) -> Result<(Amount, BlockIdentifier), ()> {
        todo!()
    }

    /// AllCurrencyBalance returns the balance batch of an account
    /// for all currencies at a particular height.
    /// It is up to the caller to determine if the retrieved
    /// block has the expected hash for the requested index.
    fn all_currency_balance(
        ctx: &Context<()>,
        network: Option<&NetworkIdentifier>,
        account: Option<&AccountIdentifier>,
        index: usize,
    ) -> Result<(Vec<Option<Amount>>, BlockIdentifier), ()> {
        todo!()
    }

    /// GetAccountBalances returns an array of AccountBalances
    /// for an array of AccountBalanceRequests
    fn get_account_balances(
        &self,
        ctx: &Context<()>,
        balance_requests: &[AccountBalanceRequest],
    ) -> Result<AccountBalance, ()> {
        todo!()
    }

    /// GetAccountCoins calls /account/coins endpoint and returns an array of
    /// coins at tip.
    fn get_account_coins(
        &self,
        ctx: Context<()>,
        acct_coins_reqs: &[AccountCoinsRequest],
    ) -> Result<AccountCoinsResponse, ()> {
        todo!()
    }
}

// TODO these functions are missing error types
pub trait BlockStorageHelper {
    fn get_block_lazy(
        &self,
        ctx: Context<()>,
        block_identifier: Option<&PartialBlockIdentifier>,
    ) -> Result<BlockResponse, ()>;
    // TODO NOTE: this todo is from the original go code not our code
    // todo add all relevant BlockStorage functions
    // to this interface.
}

// TODO suppose to use BigFloat but the only crate for that in rust only has
// 3000 downloads
/// BigPow10 computes the value of 10^e.
/// Inspired by:
/// https://steemit.com/tutorial/@gopher23/power-and-root-functions-using-big-float-in-golang
pub fn big_pow_10(e: usize) -> f64 {
    todo!()
}

// TODO suppose to use BigFloat but the only crate for that in rust only has
// 3000 downloads
/// Zero returns a float with 256 bit precision.
pub fn zero() -> f64 {
    todo!()
}

// RandomNumber returns some number in the range [minimum, maximum).
pub fn random_number(min: &BigInt, max: &BigInt) -> Result<BigInt, String> {
    if min <= max {
        Ok(thread_rng().gen_bigint_range(min, max))
    } else {
        Err(format!("maximum value {max} < minimum value {min}"))
    }
}

/// PrettyAmount returns a currency amount in native format with
/// its symbol.
pub fn pretty_amount(amount: &BigInt, currency: &Currency) -> String {
    todo!()
}

/// Milliseconds gets the current time in milliseconds.
pub fn milliseconds() -> usize {
    todo!()
}

/// AccountBalanceRequest defines the required information
/// to get an account's balance.
pub struct AccountBalanceRequest {
    account: Option<AccountIdentifier>,
    network: Option<NetworkIdentifier>,
    currency: Option<Currency>,
}

/// AccountBalance defines an account's balance,
/// including either balance or coins, as well as
/// the block which this balance was fetched at.
pub struct AccountBalance {
    account: Option<AccountIdentifier>,
    amount: Option<Amount>,
    coins: Vec<Option<Coin>>,
    block: Option<BlockIdentifier>,
}

/// AccountCoinsRequest defines the required information to get an account's
/// coins.
pub struct AccountCoinsRequest {
    account: Option<AccountIdentifier>,
    network: Option<NetworkIdentifier>,
    currencies: Vec<Option<Currency>>,
    include_mempool: bool,
}

/// AccountCoins defines an account's coins info at tip.
pub struct AccountCoinsResponse {
    coins: Vec<Option<Coin>>,
}

/// AtTip returns a boolean indicating if a block timestamp
/// is within tipDelay from the current time.
pub fn at_tip(tip_delay: usize, block_timestamp: usize) -> bool {
    todo!()
}

/// ContextSleep sleeps for the provided duration and returns
/// an error if context is canceled.
pub fn context_sleep(ctx: &Context<()>, duration: Duration) -> Result<(), ()> {
    todo!()
}

// TODO: probably not possible/applicable for rust
/// MemoryUsage contains memory usage stats converted
/// to MBs.
pub struct MemoryUsage {
    heap: f64,
    stack: f64,
    other_system: f64,
    system: f64,
    garbage_collections: f64,
}

impl MemoryUsage {
    /// MonitorMemoryUsage returns a collection of memory usage
    /// stats in MB. It will also run garbage collection if the heap
    /// is greater than maxHeapUsage in MB.
    pub fn monitor(ctx: &Context<()>, max_heap_usage: usize) -> Self {
        todo!()
    }
}

/// TimeToTip returns the estimate time to tip given
/// the current sync speed.
pub fn time_to_tip(blocks_per_second: f64, last_synced_index: usize, tip_index: usize) -> Duration {
    todo!()
}
