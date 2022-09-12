use mentat_types::{
    AccountIdentifier, Amount, Coin, Currency, Metadata, NetworkIdentifier, PublicKey,
};
use serde_json::Value;

use super::errors::WorkerResult;

// TODO need database and keypair types
/// Helper is used by the worker to process Jobs.
pub trait Helper {
    /// called to persist a [`AccountIdentifier`] + [`KeyPair`].
    fn store_key(&mut self, _: (), _: &AccountIdentifier, _: ()) -> WorkerResult<()>;

    /// returns a slice of all known [`AccountIdentifier`].
    fn all_accounts(&self, _: ()) -> WorkerResult<&[AccountIdentifier]>;

    /// a slice of all [`AccountIdentifier`] currently sending or receiving funds.
    fn locked_accounts(&self, _: ()) -> WorkerResult<&[AccountIdentifier]>;

    /// returns the balance for a provided address and currency.
    fn balance(
        &self,
        _: (),
        _: &AccountIdentifier,
        _: &Currency,
    ) -> WorkerResult<Option<Amount>>;

    /// returns all Coin owned by an address.
    fn coins(
        &self,
        _: (),
        _: &AccountIdentifier,
        _: &Currency,
    ) -> WorkerResult<&[Coin]>;

    /// returns a new [`AccountIdentifier`] for a provided [`PublicKey`].
    fn derive(
        _: &NetworkIdentifier,
        _: &PublicKey,
        _: Metadata,
    ) -> WorkerResult<(Option<AccountIdentifier>, Metadata)>;

    /// transactionally persists a key and value.
    fn set_blob(db_tx: (), key: &str, value: Value) -> WorkerResult<()>;

    /// transactionally retrieves a key and value.
    fn get_blob(db_tx: (), key: &str) -> WorkerResult<(bool, Value)>;
}

/// Worker processes jobs.
pub struct Worker<T: Helper>(T);
