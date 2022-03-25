use std::{
    sync::{Arc, Weak},
    time::Instant,
};

use axum::Json;
use tokio::sync::broadcast;

use crate::{api::MentatResponse, errors::MentatError};

pub type Fetched<T> = Option<(Instant, Json<T>)>;
pub type Inflight<T> = Option<Weak<broadcast::Sender<MentatResponse<T>>>>;
pub type WeakInflight<T> =
    Option<Arc<tokio::sync::broadcast::Sender<Result<Json<T>, MentatError>>>>;

pub trait CacheInner<T>: Clone + Send + Sync + 'static {
    fn last_fetched(&self) -> Option<&(Instant, Json<T>)>;

    fn set_last_fetched(&mut self, fetched: (Instant, Json<T>));

    fn inflight(&self) -> WeakInflight<T>;

    fn set_inflight(&mut self, inflight: Inflight<T>);
}

#[derive(Clone)]
pub struct DefaultCacheInner<T>
where
    T: Clone + Send + Sync + 'static,
{
    last_fetched: Fetched<T>,
    inflight: Inflight<T>,
}

impl<T> CacheInner<T> for DefaultCacheInner<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn last_fetched(&self) -> Option<&(Instant, Json<T>)> {
        self.last_fetched.as_ref()
    }

    fn set_last_fetched(&mut self, fetched: (Instant, Json<T>)) {
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
