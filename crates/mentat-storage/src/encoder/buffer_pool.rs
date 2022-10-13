// TODO 0rphon: i may already have some code from a personal project that does basically the same thing as this
/// BufferPool contains a sync.Pool
/// of *bytes.Buffer.
pub struct BufferPool {
    // TODO sync.Pool
    pool: (),
}

impl BufferPool {
    /// Returns a new *BufferPool.
    pub fn new() -> Self {
        todo!()
    }

    // TODO bytes.Buffer
    /// Resets the provided *bytes.Buffer and stores
    /// it in the pool for reuse.
    pub fn put(&self, bytes: ()) {
        todo!()
    }

    /// Creates a *bytes.Buffer from the provided
    /// []byte and stores it in the pool for reuse.
    pub fn put_byte_slice(&self, buffer: Vec<u8>) {
        todo!()
    }

    // TODO bytes.Buffer
    /// Returns a new or reused *bytes.Buffer.
    pub fn get(&self) -> () {
        todo!()
    }
}
