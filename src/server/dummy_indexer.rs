use super::*;

pub struct DummyIndexerApi;

#[async_trait::async_trait]
impl CallIndexerApi for DummyIndexerApi {}

#[async_trait::async_trait]
impl IndexerApi for DummyIndexerApi {}
