use super::*;

pub struct DummyIndexerApi;

#[async_trait::async_trait]
impl CallerIndexerApi for DummyIndexerApi {}

#[async_trait::async_trait]
impl IndexerApi for DummyIndexerApi {}
