use mentat_storage::Transaction;
use mentat_types::{
    AccountIdentifier, Amount, Coin, Currency, Metadata, NetworkIdentifier, PublicKey,
};
use serde_json::Value;

use super::errors::WorkerResult;

use crate::tmp::KeyPair;

#[cfg(not(test))]
/// Helper is used by the worker to process Jobs.
pub trait Helper {
    /// called to persist a [`AccountIdentifier`] + [`KeyPair`].
    fn store_key(
        &mut self,
        _: &impl Transaction,
        _: &AccountIdentifier,
        _: Option<KeyPair>,
    ) -> WorkerResult<()>;

    /// returns a slice of all known [`AccountIdentifier`].
    fn all_accounts(&self, _: &impl Transaction) -> WorkerResult<&[AccountIdentifier]>;

    /// a slice of all [`AccountIdentifier`] currently sending or receiving funds.
    fn locked_accounts(&self, _: &impl Transaction) -> WorkerResult<&[AccountIdentifier]>;

    /// returns the balance for a provided address and currency.
    fn balance(
        &self,
        _: &impl Transaction,
        _: &AccountIdentifier,
        _: &Currency,
    ) -> WorkerResult<&Amount>;

    /// returns all Coin owned by an address.
    fn coins(
        &self,
        _: &impl Transaction,
        _: &AccountIdentifier,
        _: &Currency,
    ) -> WorkerResult<&[Coin]>;

    /// returns a new [`AccountIdentifier`] for a provided [`PublicKey`].
    fn derive(
        &self,
        _: &NetworkIdentifier,
        _: &PublicKey,
        _: Metadata,
    ) -> WorkerResult<(Option<AccountIdentifier>, Metadata)>;

    /// transactionally persists a key and value.
    fn set_blob(&self, db_tx: &impl Transaction, key: Value, value: Value) -> WorkerResult<()>;

    /// transactionally retrieves a key and value.
    fn get_blob(&self, db_tx: &impl Transaction, key: &Value) -> WorkerResult<Option<Value>>;
}

#[cfg(test)]
/// Helper is used by the worker to process Jobs.
pub trait Helper {
    /// called to persist a [`AccountIdentifier`] + [`KeyPair`].
    fn store_key<T: 'static + Transaction>(
        &mut self,
        _: T,
        _: &AccountIdentifier,
        _: Option<KeyPair>,
    ) -> WorkerResult<()>;

    /// returns a slice of all known [`AccountIdentifier`].
    fn all_accounts<T: 'static + Transaction>(
        &self,
        _: T,
    ) -> WorkerResult<&'static [AccountIdentifier]>;

    /// a slice of all [`AccountIdentifier`] currently sending or receiving funds.
    fn locked_accounts<T: 'static + Transaction>(
        &self,
        _: T,
    ) -> WorkerResult<&'static [AccountIdentifier]>;

    /// returns the balance for a provided address and currency.
    fn balance<T: 'static + Transaction>(
        &self,
        _: T,
        _: &AccountIdentifier,
        _: &Currency,
    ) -> WorkerResult<&'static Amount>;

    /// returns all Coin owned by an address.
    fn coins<T: 'static + Transaction>(
        &self,
        _: T,
        _: &AccountIdentifier,
        _: &Currency,
    ) -> WorkerResult<&'static [Coin]>;

    /// returns a new [`AccountIdentifier`] for a provided [`PublicKey`].
    fn derive(
        &self,
        _: &NetworkIdentifier,
        _: &PublicKey,
        _: Metadata,
    ) -> WorkerResult<(Option<AccountIdentifier>, Metadata)>;

    /// transactionally persists a key and value.
    fn set_blob<T: 'static + Transaction>(
        &self,
        db_tx: T,
        key: Value,
        value: Value,
    ) -> WorkerResult<()>;

    /// transactionally retrieves a key and value.
    fn get_blob<T: 'static + Transaction>(
        &self,
        db_tx: T,
        key: &Value,
    ) -> WorkerResult<Option<Value>>;
}
