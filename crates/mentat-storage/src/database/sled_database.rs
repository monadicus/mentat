use std::{
    path::Path,
    sync::{atomic::AtomicBool, Arc, Mutex, RwLockWriteGuard},
    time::Duration,
};

use mentat_utils::{mutex_map::MutexMap, priority_mutex::PriorityMutex, sharded_map::ShardedMap};
pub use sled::Config;
use sled::Db;

use crate::{
    encoder::{Buffer, CompressorEntry, Encoder},
    errors::StorageResult,
};

use super::{Database, SledBuilder, Transaction};

/// A wrapper around Sled DB that implements the Database interface.
pub struct SledDatabase {
    pub(crate) sled_options: Config,
    pub(crate) compressor_entries: Vec<CompressorEntry>,
    pub(crate) pool: Option<Buffer<u8>>,
    pub(crate) db: Db,
    pub(crate) encoder: Encoder,
    pub(crate) compress: bool,
    pub(crate) writer: MutexMap<()>,
    pub(crate) writer_shards: usize,
}

impl SledDatabase {
    /// DefaultBlockCacheSize is 0 MB.
    pub const DEFAULT_BLOCK_CACHE_SIZE: usize = 0;

    /// DefaultIndexCacheSize is 2 GB.
    pub const DEFAULT_INDEX_CACHE_SIZE: u64 = 2000 << 20;

    /// TinyIndexCacheSize is 10 MB.
    pub const TINY_INDEX_CACHE_SIZE: u64 = 10 << 20;

    /// DefaultMaxTableSize is 256 MB. The larger
    /// this value is, the larger database transactions
    /// storage can handle (~15% of the max table size
    /// == max commit size).
    pub const DEFAULT_MAX_TABLE_SIZE: usize = 256 << 20;

    /// DefaultLogValueSize is 64 MB.
    pub const DEFAULT_LOG_VALUE_SIZE: usize = 64 << 20;

    /// PerformanceMaxTableSize is 3072 MB. The larger
    /// this value is, the larger database transactions
    /// storage can handle (~15% of the max table size
    /// == max commit size).
    pub const PERFORMANCE_MAX_TABLE_SIZE: usize = 3072 << 20;

    /// PerformanceLogValueSize is 256 MB.
    pub const PERFORMANCE_LOG_VALUE_SIZE: usize = 256 << 20;

    /// AllInMemoryTableSize is 2048 MB.
    pub const ALL_IN_MEMORY_TABLE_SIZE: usize = 2048 << 20;

    /// PerformanceLogValueSize is 512 MB.
    pub const ALL_IN_MEMORY_LOG_VALUE_SIZE: usize = 512 << 20;

    /// DefaultCompressionMode is the default block
    /// compression setting.
    pub const DEFAULT_COMPRESSION_MODE: Option<i32> = None;

    /// logModulo determines how often we should print
    /// logs while scanning data.
    pub const LOG_MODULO: usize = 5000;

    /// Default GC settings for reclaiming
    /// space in value logs.
    pub const DEFAULT_GCINTERVAL: Duration = Duration::from_secs(60);
    pub const DEFAULT_GCDISCARD_RATIO: f64 = 0.1;
    pub const DEFAULT_GCSLEEP: Duration = Duration::from_secs(10);

    // TODO this doc was for badger. see how much of it applies to sled
    /// The default options used to initialized
    /// a new Sled DB. These settings override many of the default Sled DB
    /// settings to restrict memory usage to ~6 GB. If constraining memory
    /// usage is not desired for your use case, you can provide your own
    /// Sled DB settings with the configuration option WithCustomSettings.
    ///
    /// There are many threads about optimizing memory usage in Sled (which
    /// can grow to many GBs if left untuned). Our own research indicates
    /// that each MB increase in MaxTableSize and/or ValueLogFileSize corresponds
    /// to a 10 MB increase in RAM usage (all other settings equal). Our primary
    /// concern is large database transaction size, so we configure MaxTableSize
    /// to be 4 times the size of ValueLogFileSize (if we skewed any further to
    /// MaxTableSize, we would quickly hit the default open file limit on many OSes).
    pub fn default_sled_config(dir: &Path) -> Config {
        todo!()
    }

    /// Performance geared
    /// Sled DB options that use much more RAM than the
    /// default settings.
    pub fn performance_sled_config(dir: &Path) -> Config {
        todo!()
    }

    /// Performance geared
    /// Sled DB options that use much more RAM than the
    /// default settings and performance settings
    pub fn all_in_memory_sled_config(dir: &Path) -> Config {
        todo!()
    }

    /// Creates a builder for a new Sled Database.
    pub fn builder(sled_options: Config) -> SledBuilder {
        SledBuilder {
            sled_options,
            ..Default::default()
        }
    }
}

impl Database for SledDatabase {
    type Tx = SledTransaction;

    /// Creates a new exclusive write SledTransaction.
    fn transaction(&self) -> Self::Tx {
        // TODO IMPORTANT: RW lock not held! should be stored inside the transaction but cant have a lifetime on the associated type
        let lock = self.writer.global_lock();

        SledTransaction {
            db: (),
            txn: Some(()),
            hold_global: true,
            rw_lock: Default::default(),
            identifier: Default::default(),
        }
    }

    /// Creates a new read SledTransaction.
    fn read_transaction(&self) -> Self::Tx {
        todo!()
    }

    /// Creates a new write SledTransaction
    /// for a particular identifier.
    fn write_transaction(&self, identifier: String, priority: bool) -> Self::Tx {
        todo!()
    }

    // TODO can probably just impl drop on this struct instead?
    /// Close closes the database to prevent corruption.
    /// The caller should defer this in main.
    fn close(self) -> StorageResult<()> {
        todo!()
    }

    /// Returns the Sled Database encoder.
    fn encoder(&self) -> StorageResult<Encoder> {
        todo!()
    }
}

/// A wrapper around a Sled
/// DB transaction that implements the DatabaseTransaction
/// interface.
#[derive(Clone)]
pub struct SledTransaction {
    // TODO should be `&'a Db`
    db: (),
    // TODO figure out how a db transaction should be stored in sled
    txn: Option<()>,
    // TODO figure out what this is locking
    rw_lock: Arc<AtomicBool>,

    hold_global: bool,
    pub identifier: String,
}

impl SledTransaction {
    fn release_locks(&mut self) {
        if self.hold_global {
            self.hold_global = false;
            // TODO IMPORTANT should drop db lock here, but lock isn't stored yet
        }

        if !self.identifier.is_empty() {
            // TODO should unlock tx lock here
            self.identifier.clear();
        }
    }
}

impl Transaction for SledTransaction {
    /// Set changes the value of the key to the value within a transaction.
    fn set(&mut self, _: serde_json::Value, _: serde_json::Value, _: bool) -> StorageResult<()> {
        todo!()
    }

    /// Get accesses the value of the key within a transaction.
    /// It is up to the caller to reclaim any memory returned.
    fn get(&self, _: &serde_json::Value) -> StorageResult<(serde_json::Value, bool)> {
        todo!()
    }

    /// Delete removes the key and its value within the transaction.
    fn delete(&mut self, _: &serde_json::Value) -> StorageResult<()> {
        todo!()
    }

    /// Scan calls a worker for each item in a scan instead
    /// of reading all items into memory.
    fn scan(
        &self,
        // prefix restriction
        _: &serde_json::Value,
        // seek start
        _: &serde_json::Value,
        _: fn(&serde_json::Value, &serde_json::Value) -> StorageResult<()>,
        // log entries
        _: bool,
        // reverse == true means greatest to least
        _: bool,
    ) -> StorageResult<usize> {
        todo!()
    }

    /// Commit attempts to commit and discard the transaction.
    fn commit(self) -> StorageResult<()> {
        todo!()
    }

    /// Discard discards an open transaction. All transactions
    /// must be either discarded or committed.
    fn discard(&mut self) {
        // TODO
        // self.txn.discard();
        self.release_locks();
    }
}

// TODO figure out good way to apply this to all types that impl transaction. macro was clunky, adding `Drop` bounds to trait generates a clippy warning
impl Drop for SledTransaction {
    fn drop(&mut self) {
        self.discard()
    }
}

fn decompress_and_save(
    encoder: Encoder,
    namespace: &str,
    tmp_dir: &Path,
    k: &[u8],
    v: &[u8],
) -> StorageResult<(f64, f64)> {
    todo!()
}

fn decompress_and_encode(
    path: &Path,
    namespace: &str,
    encoder: Encoder,
) -> StorageResult<(f64, f64, f64)> {
    todo!()
}

/// recompress compares the new compressor versus
/// what is already on-chain. It returns the old
/// on-disk size vs the new on-disk size with the new
/// compressor.
pub fn recompress(
    db: impl Database,
    namespace: &str,
    restricted_namespace: &str,
    new_compressor: Encoder,
) -> StorageResult<(f64, f64)> {
    todo!()
}

// TODO figure out what here applies to sled
/// Creates a zstd dictionary for a given SledDatabase DB namespace.
/// Optionally, you can specify the maximum number of entries to load into
/// storage (if -1 is provided, then all possible are loaded).
pub fn sled_train(
    namespace: &str,
    db: &str,
    output: &str,
    max_entries: usize,
    compressor_entries: Vec<CompressorEntry>,
) -> StorageResult<(f64, f64)> {
    todo!()
}
