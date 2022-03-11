use super::*;

pub struct DummyIndexerApi;

#[rocket::async_trait]
impl CallerIndexerApi for DummyIndexerApi {}

#[rocket::async_trait]
impl IndexerApi for DummyIndexerApi {}
