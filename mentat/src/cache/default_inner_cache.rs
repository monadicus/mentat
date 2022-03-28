use std::sync::Weak;

use super::{CacheInner, Entry, Fetched, Inflight, WeakInflight};

#[derive(Clone)]
pub struct DefaultCacheInner<T>
where
    T: Clone + Send + Sync + 'static,
{
    last_fetched: Fetched<T>,
    inflight: Inflight<T>,
}

impl<T> CacheInner for DefaultCacheInner<T>
where
    T: Clone + Send + Sync + 'static,
{
    type T = T;

    fn last_fetched(&self) -> Option<&Entry<T>> {
        self.last_fetched.as_ref()
    }

    fn set_last_fetched(&mut self, fetched: Entry<T>) {
        self.last_fetched.replace(fetched);
    }

    fn inflight(&self) -> WeakInflight<T> {
        self.inflight.as_ref().and_then(Weak::upgrade)
    }

    fn set_inflight(&mut self, inflight: Inflight<T>) {
        self.inflight = inflight;
    }
}

impl<T> Default for DefaultCacheInner<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            last_fetched: None,
            inflight: None,
        }
    }
}
