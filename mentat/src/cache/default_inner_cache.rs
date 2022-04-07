//! Defines a default `CacheInner` trait implementation.

use std::sync::Weak;

use super::{CacheInner, Entry, Fetched, Inflight, WeakInflight};

/// This struct stores the latest Fetched entry and [`Inflight`]
/// if they exist in memory.
#[derive(Clone)]
pub struct DefaultCacheInner<T> {
    /// Stores the possible entire cache entry in memory.
    last_fetched: Fetched<T>,
    /// Stores the possible request inflight tracker.
    inflight: WeakInflight<T>,
}

impl<T> CacheInner for DefaultCacheInner<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Data = T;

    fn last_fetched(&self) -> Option<&Entry<Self::Data>> {
        self.last_fetched.as_ref()
    }

    fn set_last_fetched(&mut self, fetched: Entry<Self::Data>) {
        self.last_fetched.replace(fetched);
    }

    fn inflight(&self) -> Inflight<Self::Data> {
        self.inflight.as_ref().and_then(Weak::upgrade)
    }

    fn set_inflight(&mut self, inflight: WeakInflight<Self::Data>) {
        self.inflight = inflight;
    }
}

impl<T> Default for DefaultCacheInner<T> {
    fn default() -> Self {
        Self {
            last_fetched: None,
            inflight: None,
        }
    }
}
