//! Defines the cache struct which contains
//! a `CacheInner` to handle the fetching and inflight producing.

use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::{broadcast, Mutex};

use super::CacheInner;
use crate::{api::MentatResponse, types::MapErrMentat};

/// A type to represent a async closure with some output.
pub type BoxFut<'a, O> = Pin<Box<dyn Future<Output = O> + Send + 'a>>;

/// The `Cache` struct which contains information on the cache.
#[derive(Clone)]
pub struct Cache<C> {
    /// For holding a [`CacheInner`] generic.
    inner: Arc<Mutex<C>>,
    /// Optional how long should we trust that cache.
    refresh_interval: Option<Duration>,
}

impl<C> Cache<C>
where
    C: CacheInner,
    C::Data: Clone + Send + Sync + 'static,
{
    /// Create a `Cache` struct with an optional refresh interval.
    pub fn new(cache: C, refresh_interval: Option<Duration>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(cache)),
            refresh_interval,
        }
    }

    /// Attempts to fetch the latest cache object if it exists.
    /// Also handles multiple of the same request at the same time
    /// telling one request to finish and send its data to the rest.
    pub async fn get_cached<F>(&self, f: F) -> MentatResponse<C::Data>
    where
        F: FnOnce() -> BoxFut<'static, MentatResponse<C::Data>> + Send + 'static,
    {
        let mut rx = {
            let mut inner = self.inner.lock().await;

            // Check if request exists
            if let Some((fetched_at, value)) = inner.last_fetched() {
                match self.refresh_interval {
                    None => return Ok(value.clone()),
                    Some(refresh_interval) if fetched_at.elapsed() < refresh_interval => {
                        return Ok(value.clone());
                    }
                    _ => {}
                }
            }

            // Check if similar request already in progress
            if let Some(inflight) = inner.inflight() {
                inflight.subscribe()
            } else {
                // Request is not already happening lets do the request.
                let (tx, rx) = broadcast::channel::<MentatResponse<C::Data>>(1);
                // refrence-count a sender
                let tx = Arc::new(tx);
                // store weak refrence in state
                // this prevents all requests getting stuck if there be a panic
                inner.set_inflight(Some(Arc::downgrade(&tx)));
                let inner = self.inner.clone();

                // call the closure first, so we don't send _it_ across threads,
                // just the Future it returns
                let fut = f();

                tokio::spawn(async move {
                    let res = fut.await;

                    {
                        let mut inner = inner.lock().await;
                        inner.set_inflight(None);

                        match res {
                            Ok(value) => {
                                inner.set_last_fetched((Instant::now(), value.clone()));
                                let _ = tx.send(Ok(value));
                            }
                            Err(e) => {
                                let _ = tx.send(Err(e));
                            }
                        }
                    }
                });
                rx
            }
        };

        // waiting for in progress request
        rx.recv().await.merr(|e| e)?
    }
}
