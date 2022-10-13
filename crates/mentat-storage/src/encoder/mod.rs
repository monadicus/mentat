mod buffer_pool;
pub use buffer_pool::*;
#[cfg(test)]
mod encoder_test;

use std::{
    io::{Read, Write},
    path::PathBuf,
};

use indexmap::IndexMap;
use mentat_types::{AccountCoin, AccountCurrency};
use serde_json::Value;

use crate::errors::StorageResult;

const JSON_TAG: &str = "json";

/// Used to initialize a dictionary compression.
/// All dictionary_paths are loaded from disk at initialization.
pub struct CompressorEntry {
    namespace: String,
    dictionary_path: PathBuf,
}

/// Encoder handles the encoding/decoding of structs and the
/// compression/decompression of data using zstd. Optionally,
/// the caller can provide a map of dicts on initialization that
/// can be used by zstd. You can read more about these "dicts" here:
/// https://github.com/facebook/zstd#the-case-for-small-data-compression.
///
/// NOTE: If you change these dicts, you will not be able
/// to decode previously encoded data. For many users, providing
/// no dicts is sufficient!
pub struct Encoder {
    compression_dicts: IndexMap<String, Value>,
    pool: Option<BufferPool>,
    compress: bool,
}

impl Encoder {
    const UNICODE_RECORD_SEPARATOR: char = '\u{001E}';

    /// Returns a new Encoder. The dicts
    /// provided should contain k:v of namespace:zstd dict.
    pub fn new(
        entries: &[CompressorEntry],
        pool: Option<BufferPool>,
        compress: bool,
    ) -> StorageResult<Option<Self>> {
        todo!()
    }

    /// Attempts to compress the object and will use a dict if
    /// one exists for the namespace.
    pub fn encode<T>(&self, namespace: &str, object: T) -> StorageResult<Vec<u8>> {
        todo!()
    }

    /// Only compresses an input, leaving encoding to the caller.
    /// This is particularly useful for training a compressor.
    pub fn encode_raw(&self, namespace: &str, input: Vec<u8>) -> StorageResult<Vec<u8>> {
        todo!()
    }

    /// Only decompresses an input, leaving decoding to the caller.
    /// This is particularly useful for training a compressor.
    pub fn decode_raw(&self, namespace: &str, input: Vec<u8>) -> StorageResult<Vec<u8>> {
        todo!()
    }

    fn encode_inner(input: Vec<u8>, zstd_dict: Vec<u8>) -> StorageResult<Vec<u8>> {
        todo!()
    }

    fn decode_inner(b: Vec<u8>, zstd_dict: Vec<u8>) -> StorageResult<Vec<u8>> {
        todo!()
    }

    // TODO ??
    /// CopyStruct performs a deep copy of an entire struct
    /// using its JSON representation.
    pub fn copy_struct<T>(input: &T, output: &mut T) -> StorageResult<()> {
        todo!()
    }

    fn encode_and_write<T>(&self, output: (), object: T) -> StorageResult<()> {
        todo!()
    }

    fn decode_map<T>(&self, input: Vec<u8>) -> StorageResult<IndexMap<String, T>> {
        todo!()
    }

    // Used to encode an AccountCoin using the scheme (on the happy path):
    // accountAddress|coinIdentifier|amountValue|amountCurrencySymbol|
    // amountCurrencyDecimals
    //
    // And the following scheme on the unhappy path:
    // accountAddress|coinIdentifier|amountValue|amountCurrencySymbol|
    // amountCurrencyDecimals|accountMetadata|subAccountAddress|
    // subAccountMetadata|amountMetadata|currencyMetadata
    //
    // In both cases, the | character is represented by the unicodeRecordSeparator rune.
    pub fn encode_account_coin(&self, account_coin: &AccountCoin) -> StorageResult<Vec<u8>> {
        todo!()
    }

    /// Decodes an AccountCoin and optionally
    /// reclaims the memory associated with the input.
    pub fn decode_account_coin(
        &self,
        b: Vec<u8>,
        account_coin: &AccountCoin,
        reclaim_input: bool,
    ) -> StorageResult<()> {
        todo!()
    }

    /// Used to encode an AccountCurrency using the scheme (on the happy path):
    /// accountAddress|currencySymbol|currencyDecimals
    ///
    /// And the following scheme on the unhappy path:
    /// accountAddress|currencySymbol|currencyDecimals|accountMetadata|
    /// subAccountAddress|subAccountMetadata|currencyMetadata
    ///
    /// In both cases, the | character is represented by the unicodeRecordSeparator rune.
    pub fn encode_account_currency(
        &self,
        account_currency: &AccountCurrency,
    ) -> StorageResult<Vec<u8>> {
        todo!()
    }

    /// Decodes an AccountCurrency and optionally
    /// reclaims the memory associated with the input.
    pub fn decode_account_currency(
        &self,
        b: Vec<u8>,
        account_currency: &AccountCurrency,
        reclaim_input: bool,
    ) -> StorageResult<()> {
        todo!()
    }
}

// TODO uses msgpack.Encoder
fn get_encoder(w: impl Write) -> ! {
    todo!()
}

// TODO uses msgpack.Decoder
fn get_decoder(r: impl Read) -> ! {
    todo!()
}
