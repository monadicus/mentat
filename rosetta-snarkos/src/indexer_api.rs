use mentat::{
    api::{CallerIndexerApi, IndexerApi},
    axum::async_trait,
};

#[derive(Clone, Default)]
pub struct SnarkosIndexerApi;

#[async_trait]
impl CallerIndexerApi for SnarkosIndexerApi {}

#[async_trait]
impl IndexerApi for SnarkosIndexerApi {}
