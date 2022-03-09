use mentat::api::{CallDataApi, DataApi};

#[derive(Default)]
pub struct SnarkosDataApi;

#[async_trait::async_trait]
impl CallDataApi for SnarkosDataApi {}

#[async_trait::async_trait]
impl DataApi for SnarkosDataApi {}
