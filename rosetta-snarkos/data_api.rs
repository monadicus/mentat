use mentat::api::DataApi;

#[derive(Default)]
pub struct SnarkosDataApi;

#[async_trait::async_trait]
impl DataApi for SnarkosDataApi {}
