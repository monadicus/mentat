//! some tools to help with multithreading in the syncer. these may be moved to
//! a `utils` crate in the future

use std::{
    sync::Arc,
    thread::JoinHandle,
    time::{Duration, Instant},
};

use parking_lot::Mutex;

use crate::errors::SyncerError;

/// helper enum for Context so that it can throw errors on behalf of the struct
/// being used
#[derive(Clone)]
pub enum ContextResult {
    /// the threads were told to stop early
    Canceled,
    /// the threads took longer than the allowed time
    DeadlineExceeded,
}

impl From<ContextResult> for SyncerError {
    fn from(e: ContextResult) -> Self {
        match e {
            ContextResult::Canceled => SyncerError::Canceled,
            ContextResult::DeadlineExceeded => SyncerError::DeadlineExceeded,
        }
    }
}

/// used to indicate errors and set deadlines across threads
#[derive(Clone)]
pub struct Context<E: Clone> {
    /// a buffer for an error thrown by any thread
    error: Arc<Mutex<Option<E>>>,
    /// an optional limit on how long the threads can stay alive
    deadline: Option<Duration>,
    /// time that the context was initiated
    start: Instant,
}

impl<E: Clone> Context<E> {
    /// creates a new context with optional deadline
    pub fn new(deadline: Option<Duration>) -> Self {
        Self {
            error: Arc::new(Mutex::new(None)),
            deadline,
            start: Instant::now(),
        }
    }

    /// sets the error to be shared by other threads. does nothing if an error
    /// has already been set
    pub fn set_err(&self, e: E) {
        if self.error.lock().is_none() {
            *self.error.lock() = Some(e);
        }
    }

    /// tells threads to exit early
    pub fn cancel(&self)
    where
        E: From<ContextResult>,
    {
        self.set_err(ContextResult::Canceled.into())
    }

    /// checks if the deadline has been reached yet and tells threads
    /// `DeadlineExceeded` if true
    fn check_deadline(&self)
    where
        E: From<ContextResult>,
    {
        if let Some(d) = self.deadline {
            if self.start.elapsed() > d {
                self.set_err(ContextResult::DeadlineExceeded.into())
            }
        }
    }

    /// returns true if the context has told threads to stop executing
    pub fn done(&self) -> bool
    where
        E: From<ContextResult>,
    {
        self.check_deadline();
        self.error.lock().is_some()
    }

    /// returns an error, if any thread has thrown one
    pub fn err(&self) -> Result<(), E>
    where
        E: From<ContextResult>,
    {
        self.check_deadline();
        self.error.lock().clone().map(|e| Err(e)).unwrap_or(Ok(()))
    }
}

/// holds threads and a context for those threads
pub struct ThreadHandler<T, E: Clone> {
    /// a list of all threads currently being tracked
    handles: Vec<JoinHandle<Result<T, E>>>,
    /// a shared state between the threads that tells if they should exit early
    context: Context<E>,
}

impl<T, E: Clone> ThreadHandler<T, E> {
    /// returns a reference to the context used by the threads
    pub fn ctx(&self) -> &Context<E> {
        &self.context
    }

    /// pushes a handle to the list of active threads
    pub fn push(&mut self, thread: JoinHandle<Result<T, E>>) {
        self.handles.push(thread)
    }

    /// blocks until all threads have exited
    pub fn wait(&mut self)
    where
        E: From<String>,
    {
        while !self.handles.is_empty() {
            self.update();
        }
    }

    /// checks the status of all threads and updates the context if an error has
    /// been thrown
    pub fn update(&mut self)
    where
        E: From<String>,
    {
        let (finished, unfinished): (Vec<_>, _) = self
            .handles
            .drain(..)
            .into_iter()
            .partition(|h| h.is_finished());
        self.handles = unfinished;

        if let Some(e) = finished.into_iter().find_map(|h| match h.join() {
            Ok(Ok(_)) => None,
            Ok(Err(e)) => Some(e),
            Err(e) => Some(format!("{e:?}").into()),
        }) {
            self.context.set_err(e);
        }
    }
}

impl<T, E: Clone> From<Context<E>> for ThreadHandler<T, E> {
    fn from(context: Context<E>) -> Self {
        Self {
            handles: Vec::new(),
            context,
        }
    }
}
