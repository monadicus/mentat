use mentat::api::{CallerIndexerApi, IndexerApi};

#[derive(Default)]
pub struct SnarkosIndexerApi;

#[rocket::async_trait]
impl CallerIndexerApi for SnarkosIndexerApi {}

#[rocket::async_trait]
impl IndexerApi for SnarkosIndexerApi {}
