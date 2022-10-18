use std::ops::{Deref, DerefMut};

pub struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(size: usize) -> Self {
        Self(Vec::with_capacity(size))
    }

    pub fn flushed(&mut self) -> &mut Vec<T> {
        self.0.clear();
        &mut self.0
    }

    pub fn swap(&mut self, buf: &mut Vec<T>) {
        std::mem::swap(&mut self.0, buf);
    }
}

impl<T> Default for Buffer<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
