//! utils needed to implement goroutine functionality in rust

use crossbeam::channel::{bounded, unbounded, Receiver, Sender};

use crate::errors::SyncerResult;

/// registers a closure to be run on drop
struct Defer<F: FnMut()> {
    /// the closure to be run
    c: F,
}
impl<F: FnMut()> Drop for Defer<F> {
    fn drop(&mut self) {
        (self.c)();
    }
}

/// registers a closure to be run on function return
macro_rules! defer {
    ($e:expr) => {
        let _defer = Defer {
            c: || -> () {
                $e;
            },
        };
    };
}

/// bi-directional channel
#[derive(Clone)]
pub struct Chan<T> {
    /// sender
    pub sender: Sender<T>,
    /// receiver
    pub receiver: Receiver<T>,
}

impl<T> Chan<T> {
    /// creates a new bidirectional channel
    pub fn make() -> Self {
        unbounded().into()
    }
}

impl<T> From<(Sender<T>, Receiver<T>)> for Chan<T> {
    fn from((sender, receiver): (Sender<T>, Receiver<T>)) -> Self {
        Self { sender, receiver }
    }
}

// // TODO: replace with functional
// /// a temporary NONsFUNCTIONING version of Context so that i can uncomment code using it
// pub struct TmpCtx;

// impl TmpCtx {
//     /// DUMMY
//     pub fn done(&self) -> bool {
//         unimplemented!("this is a dummy struct with no functionality")
//     }

//     /// DUMMY
//     pub fn err(&self) -> SyncerResult<()> {
//         unimplemented!("this is a dummy struct with no functionality")
//     }
// }
