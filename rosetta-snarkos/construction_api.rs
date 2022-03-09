use mentat::api::{CallConstructionApi, ConstructionApi};

#[derive(Default)]
pub struct SnarkosConstructionApi;

#[async_trait::async_trait]
impl CallConstructionApi for SnarkosConstructionApi {}

impl ConstructionApi for SnarkosConstructionApi {}
