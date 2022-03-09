use mentat::api::IndexerApi;

#[derive(Default)]
pub struct SnarkosIndexerApi;

#[async_trait::async_trait]
impl IndexerApi for SnarkosIndexerApi {}
