use mentat::api::{CallerIndexerApi, IndexerApi};

#[derive(Default)]
pub struct SnarkosIndexerApi;

#[async_trait::async_trait]
impl CallerIndexerApi for SnarkosIndexerApi {}

#[async_trait::async_trait]
impl IndexerApi for SnarkosIndexerApi {}
