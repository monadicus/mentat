use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Weak},
    time::{Duration, Instant},
};

use axum::Json;
use parking_lot::Mutex;
use tokio::sync::broadcast;

use crate::{api::MentatResponse, errors::MentatError};

pub struct CacheInner<T>
where
    T: Clone + Send + Sync + 'static,
{
    last_fetched: Option<(Instant, Json<T>)>,
    inflight: Option<Weak<broadcast::Sender<MentatResponse<T>>>>,
}

impl<T> Default for CacheInner<T>
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

#[derive(Clone)]
pub struct Cached<T>
where
    T: Clone + Send + Sync + 'static,
{
    inner: Arc<Mutex<CacheInner<T>>>,
    refresh_interval: Option<Duration>,
}

pub type BoxFut<'a, O> = Pin<Box<dyn Future<Output = O> + Send + 'a>>;

impl<T> Cached<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(refresh_interval: Option<Duration>) -> Self {
        Self {
            inner: Default::default(),
            refresh_interval,
        }
    }

    pub async fn get_cached<F>(&self, f: F) -> MentatResponse<T>
    where
        F: FnOnce() -> BoxFut<'static, MentatResponse<T>> + Send + 'static,
    {
        let mut rx = {
            let mut inner = self.inner.lock();

            // Check if request exists
            if let Some((fetched_at, value)) = inner.last_fetched.as_ref() {
                match self.refresh_interval {
                    None => return Ok(value.clone()),
                    Some(refresh_interval) if fetched_at.elapsed() < refresh_interval => {
                        return Ok(value.clone());
                    }
                    _ => {}
                }
            }

            // Check if similar request already in progress
            if let Some(inflight) = inner.inflight.as_ref().and_then(Weak::upgrade) {
                inflight.subscribe()
            } else {
                // Request is not already happening lets do the request.
                let (tx, rx) = broadcast::channel::<MentatResponse<T>>(1);
                // refrence-count a sender
                let tx = Arc::new(tx);
                // store weak refrence in state
                // this prevents all requests getting stuck if there be a panic
                inner.inflight = Some(Arc::downgrade(&tx));
                let inner = self.inner.clone();

                // call the closure first, so we don't send _it_ across threads,
                // just the Future it returns
                let fut = f();

                tokio::spawn(async move {
                    let res = fut.await;

                    {
                        let mut inner = inner.lock();
                        inner.inflight = None;

                        match res {
                            Ok(value) => {
                                inner.last_fetched.replace((Instant::now(), value.clone()));
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
