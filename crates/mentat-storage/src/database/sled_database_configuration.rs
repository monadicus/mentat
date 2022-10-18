use std::path::{Path, PathBuf};

use crossbeam_channel::unbounded;
use mentat_utils::sharded_map::DEFAULT_SHARDS;
use sled::Config;

use crate::encoder::{BufferPool, CompressorEntry};

use super::SledDatabase;

#[derive(Default)]
pub struct SledBuilder {
    pub sled_options: Config,
    pub compressor_entries: Option<Vec<CompressorEntry>>,
    pub compress: Option<bool>,
    pub writer_shards: Option<usize>,
}

impl SledBuilder {
    pub fn new(sled_options: Config) -> Self {
        Self {
            sled_options,
            ..Default::default()
        }
    }

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

    pub fn build(mut self) -> SledDatabase {
        let b = SledDatabase {
            sled_options: self.sled_options,
            compressor_entries: self.compressor_entries.unwrap_or_default(),
            pool: BufferPool::new(),
            compress: true,
            writer: todo!(),
            writer_shards: self.writer_shards.unwrap_or(DEFAULT_SHARDS),
            closed: unbounded(),
        };
        todo!()
    }
}
