use mentat::api::{CallerDataApi, DataApi};

#[derive(Default)]
pub struct SnarkosDataApi;

#[async_trait::async_trait]
impl CallerDataApi for SnarkosDataApi {}

#[async_trait::async_trait]
impl DataApi for SnarkosDataApi {}
