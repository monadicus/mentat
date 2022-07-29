//! Defines an overridable trait so that
//! users can implement how caching is handled for their server.

use std::{
    sync::{Arc, Weak},
    time::Instant,
};

use axum::Json;
use tokio::sync::broadcast;

use crate::{api::MentatResponse, types::MentatError};

/// A cache `Entry`, stores when it was last accessed, and the `Json` of the
/// data it has.
pub type Entry<T> = (Instant, Json<T>);
/// A type shorthand to represent a possible fetched `Entry`.
pub type Fetched<T> = Option<Entry<T>>;
/// A type shorthand to represent a possible broadcast that this response is
/// already in progress. It is wrapped in a weak reference to avoid race
/// conditions.
pub type WeakInflight<T> = Option<Weak<broadcast::Sender<MentatResponse<T>>>>;
/// A type shorthand to represent a possible broadcast that this response is
/// already in progress.
pub type Inflight<T> = Option<Arc<broadcast::Sender<Result<Json<T>, MentatError>>>>;
/// The `CacheInner` trait is the inner portion of a cache object.
/// It handles getting and setting entries and inflights.
pub trait CacheInner: Clone + Send + Sync + 'static {
    /// The associated data type that is being stored in the cache entry.
    type Data;

    /// Fetches the last possible `Entry` in the cache.
    fn last_fetched(&self) -> Option<&Entry<Self::Data>>;

    /// Sets the latest cache `Entry`.
    fn set_last_fetched(&mut self, fetched: Entry<Self::Data>);

    /// Grab the possible inflight to see if the same request is in progress.
    fn inflight(&self) -> Inflight<Self::Data>;

    /// Set the possible inflight if the same request is in progress.
    fn set_inflight(&mut self, inflight: WeakInflight<Self::Data>);
}
