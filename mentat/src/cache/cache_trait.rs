use std::{future::Future, pin::Pin, time::Duration};

use axum::async_trait;

use super::CacheInner;
use crate::api::MentatResponse;

pub type BoxFut<'a, O> = Pin<Box<dyn Future<Output = O> + Send + 'a>>;

#[async_trait]
pub trait Cache<C, T>
where
    C: CacheInner<T>,
    T: Clone + Send + Sync + 'static,
{
    fn new(cache: C, refresh_interval: Option<Duration>) -> Self;

    async fn get_cached<F>(&self, f: F) -> MentatResponse<T>
    where
        F: FnOnce() -> BoxFut<'static, MentatResponse<T>> + Send + 'static;
}
