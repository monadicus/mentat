use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
    time::{Duration, Instant},
};

use parking_lot::Mutex;
use tokio::sync::broadcast;

use super::CacheInner;
use crate::{api::MentatResponse, errors::MentatError};

#[derive(Clone)]
pub struct Cache<C, T>
where
    C: CacheInner<T>,
    T: Clone + Send + Sync + 'static,
{
    inner: Arc<Mutex<C>>,
    refresh_interval: Option<Duration>,
    _data: PhantomData<T>,
}

pub type BoxFut<'a, O> = Pin<Box<dyn Future<Output = O> + Send + 'a>>;

impl<C, T> Cache<C, T>
where
    C: CacheInner<T>,
    T: Clone + Send + Sync + 'static,
{
    pub fn new(cache: C, refresh_interval: Option<Duration>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(cache)),
            refresh_interval,
            _data: PhantomData,
        }
    }

    pub async fn get_cached<F>(&self, f: F) -> MentatResponse<T>
    where
        F: FnOnce() -> BoxFut<'static, MentatResponse<T>> + Send + 'static,
    {
        let mut rx = {
            let mut inner = self.inner.lock();

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
                let (tx, rx) = broadcast::channel::<MentatResponse<T>>(1);
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
                        let mut inner = inner.lock();
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
        Ok(rx.recv().await.map_err(MentatError::from)??)
    }
}
