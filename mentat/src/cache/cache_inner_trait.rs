use std::{
    sync::{Arc, Weak},
    time::Instant,
};

use axum::Json;
use tokio::sync::broadcast;

use crate::{api::MentatResponse, errors::MentatError};

pub type Entry<T> = (Instant, Json<T>);
pub type Fetched<T> = Option<Entry<T>>;
pub type Inflight<T> = Option<Weak<broadcast::Sender<MentatResponse<T>>>>;
pub type WeakInflight<T> =
    Option<Arc<tokio::sync::broadcast::Sender<Result<Json<T>, MentatError>>>>;

pub trait CacheInner<T>: Clone + Send + Sync + 'static {
    fn last_fetched(&self) -> Option<&Entry<T>>;

    fn set_last_fetched(&mut self, fetched: Entry<T>);

    fn inflight(&self) -> WeakInflight<T>;

    fn set_inflight(&mut self, inflight: Inflight<T>);
}

