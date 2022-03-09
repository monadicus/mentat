use mentat::api::{CallIndexerApi, IndexerApi};

#[derive(Default)]
pub struct SnarkosIndexerApi;

#[async_trait::async_trait]
impl CallIndexerApi for SnarkosIndexerApi {}

#[async_trait::async_trait]
impl IndexerApi for SnarkosIndexerApi {}
