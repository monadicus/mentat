use std::{
    env::temp_dir,
    fmt::Debug,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
};

use mentat_types::{hash, NetworkIdentifier, Sortable};
use num_bigint_dig::{BigInt, Sign};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

/// specifies that the user can
/// read and write the file.
pub const DEFAULT_FILE_PERMISSIONS: usize = 0o600;

/// specifies anyone can do anything
/// to the file.
pub const ALL_FILE_PERMISSIONS: usize = 0o777;

const BASE_10: usize = 10;

const BIG_BLOAT_PRECISION: usize = 512;

/// the number
/// of nanoseconds in a millisecond.
const NANOSECONDS_IN_MILLISECOND: usize = 1000000;

/// the number
/// of milliseconds in a second.
const MILLISECOND_IN_SECOND: usize = 1000;

/// the number 100.
const ONE_HUNDRED: usize = 100;

/// the minimum blocks per second
/// to consider when estimating time to tip if the provided
/// estimate is 0.
const MIN_BLOCKS_PER_SECOND: f64 = 0.0001;

/// the maximum number of entries
/// in one transaction object. This is used for bootstrap
/// balances process to avoid TxnTooBig error when memory_limit_disabled=false
/// as well as reduce the running time.
const MaxEntrySizePerTxn: usize = 600;

/// a BigInt of value 100.
fn one_hundred_int() -> BigInt {
    BigInt::from(100)
}

/// a BigInt of value 0.
fn zero_int() -> BigInt {
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
    #[error("{0}: unable to unmarshal")]
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
/// and attempts to unmarshal it into output.
pub fn load_and_parse<T: DeserializeOwned>(path: &Path) -> Result<T, UtilsError> {
    let b = fs::read_to_string(path).map_err(|e| UtilsError::ReadError(e, path.to_path_buf()))?;
    serde_json::from_str(&b).map_err(UtilsError::DeserializeError)
}

/// CreateCommandPath creates a unique path for a command and network within a data directory. This
/// is used to avoid collision when using multiple commands on multiple networks
/// when the same storage resources are used. If the derived path does not exist,
/// we run os.MkdirAll on the path.
pub fn create_command_path(
    data_dir: &Path,
    cmd: String,
    network: Option<&NetworkIdentifier>,
) -> Result<PathBuf, UtilsError> {
    let data_path = data_dir.join(cmd).join(hash(network));
    ensure_path_exists(&data_path).map_err(|e| UtilsError::CannotPopulatePath(Box::new(e)))?;
    Ok(data_path)
}

