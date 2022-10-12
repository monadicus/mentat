mod badger_database;
mod badger_database_configuration;
#[cfg(test)]
mod badger_database_test;

use crate::errors::StorageResult;
use serde_json::Value;

/// Database is an interface that provides transactional
/// access to a KV store.
pub trait Database {
    type Tx: Transaction;

    /// Transaction acquires an exclusive write lock on the database.
    /// This ensures all other calls to Transaction and WriteTransaction
    /// will block until the returned DatabaseTransaction is committed or
    /// discarded. This is useful for making changes across
    /// multiple prefixes but incurs a large performance overhead.
    fn transaction() -> Self::Tx;

    /// ReadTransaction allows for consistent, read-only access
    /// to the database. This does not acquire any lock
    /// on the database.
    fn read_transaction() -> Self::Tx;

    /// WriteTransaction acquires a granular write lock for a particular
    /// identifier. All subsequent calls to WriteTransaction with the same
    /// identifier will block until the DatabaseTransaction returned is either
    /// committed or discarded.
    fn write_transaction(identifier: String, priority: bool) -> Self::Tx;

    /// Close shuts down the database.
    fn close() -> StorageResult<()>;

    /// Encoder returns the *Encoder used to store/read data
    /// in the database. This *Encoder often performs some
    /// form of compression on data.
    fn encoder() -> StorageResult<()>;
}

/// Transaction is an interface that provides
/// access to a KV store within some transaction
/// context provided by a Database.
///
/// When a Transaction is committed or discarded,
/// all memory utilized is reclaimed. If you want to persist
/// any data retrieved, make sure to make a copy!
pub trait Transaction {
    fn set(&mut self, _: Value, _: Value, _: bool) -> StorageResult<()>;
    fn get(&self, _: &Value) -> StorageResult<(Value, bool)>;
    fn delete(&mut self, _: &Value) -> StorageResult<()>;

    fn scan(
        &self,
        // prefix restriction
        _: &Value,
        // seek start
        _: &Value,
        _: fn(&Value, &Value) -> StorageResult<()>,
        // log entries
        _: bool,
        // reverse == true means greatest to least
        _: bool,
    ) -> StorageResult<usize>;

    fn commit(&mut self) -> StorageResult<()>;
    fn discard(&mut self);
}

/// CommitWorker is returned by a module to be called after
/// changes have been committed. It is common to put logging activities
/// in here (that shouldn't be printed until the block is committed).
pub type CommitWorker = fn() -> ();
