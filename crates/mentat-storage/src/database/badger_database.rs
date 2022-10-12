use std::time::Duration;

/// DefaultBlockCacheSize is 0 MB.
const DefaultBlockCacheSize: usize = 0;

/// DefaultIndexCacheSize is 2 GB.
const DefaultIndexCacheSize: usize = 2000 << 20;

/// TinyIndexCacheSize is 10 MB.
const TinyIndexCacheSize: usize = 10 << 20;

/// DefaultMaxTableSize is 256 MB. The larger
/// this value is, the larger database transactions
/// storage can handle (~15% of the max table size
/// == max commit size).
const DefaultMaxTableSize: usize = 256 << 20;

/// DefaultLogValueSize is 64 MB.
const DefaultLogValueSize: usize = 64 << 20;

/// PerformanceMaxTableSize is 3072 MB. The larger
/// this value is, the larger database transactions
/// storage can handle (~15% of the max table size
/// == max commit size).
const PerformanceMaxTableSize: usize = 3072 << 20;

/// PerformanceLogValueSize is 256 MB.
const PerformanceLogValueSize: usize = 256 << 20;

/// AllInMemoryTableSize is 2048 MB.
const AllInMemoryTableSize: usize = 2048 << 20;

/// PerformanceLogValueSize is 512 MB.
const AllInMemoryLogValueSize: usize = 512 << 20;

// TODO
/// DefaultCompressionMode is the default block
/// compression setting.
const DefaultCompressionMode: Option<()> = None;

/// logModulo determines how often we should print
/// logs while scanning data.
const logModulo: usize = 5000;

/// Default GC settings for reclaiming
/// space in value logs.
const defaultGCInterval: Duration = Duration::from_secs(60);
const defaultGCDiscardRatio: f64 = 0.1;
const defaultGCSleep: Duration = Duration::from_secs(10);

pub fn todo() {
    todo!("this wont work! need to use sled instead of badger")
}
