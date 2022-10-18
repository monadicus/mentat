use mentat_utils::{mutex_map::MutexMap, sharded_map::DEFAULT_SHARDS};
use sled::Config;

use crate::{
    encoder::{BufferPool, CompressorEntry, Encoder},
    errors::StorageResult,
};

use super::SledDatabase;

#[derive(Default)]
pub struct SledBuilder {
    pub sled_options: Config,
    pub compressor_entries: Option<Vec<CompressorEntry>>,
    pub compress: Option<bool>,
    pub writer_shards: Option<usize>,
}

impl SledBuilder {
    /// Provides zstd dictionaries
    /// for given namespaces.
    pub fn with_compressor_entries(mut self, entries: Vec<CompressorEntry>) -> Self {
        self.compressor_entries = Some(entries);
        self.compress = Some(true);
        self.sled_options = self.sled_options.use_compression(true);
        self
    }

    /// Disables zstd compression.
    pub fn without_compression(mut self) -> Self {
        self.compress = Some(false);
        self.sled_options = self.sled_options.use_compression(false);
        self
    }

    /// Overrides the default shards used
    /// in the writer [`MutexMap`]. It is recommended
    /// to set this value to your write concurrency to prevent
    /// lock contention.
    pub fn with_writer_shards(mut self, shards: usize) -> Self {
        self.writer_shards = Some(shards);
        self
    }

    // Creates a new SledDatabase from the given builder
    pub fn build(self) -> StorageResult<SledDatabase> {
        let writer_shards = self.writer_shards.unwrap_or(DEFAULT_SHARDS);
        let compressor_entries = self.compressor_entries.unwrap_or_default();
        let compress = self.compress.unwrap_or_default();
        let pool = BufferPool::new();
        Ok(SledDatabase {
            encoder: Encoder::new(&compressor_entries, &pool, compress)
                .map_err(|e| format!("unable to load compressor: {e}"))?,
            db: self
                .sled_options
                .open()
                .map_err(|e| format!("unable to open database: {e}"))?,
            sled_options: self.sled_options,
            compressor_entries,
            pool: Some(pool),
            compress,
            // Initialize utis.MutexMap used to track granular
            // write transactions.
            writer: Some(MutexMap::new(writer_shards)),
            writer_shards,
        })
    }
}
