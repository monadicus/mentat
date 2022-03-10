use mentat::api::{CallerConstructionApi, ConstructionApi};

#[derive(Default)]
pub struct SnarkosConstructionApi;

#[async_trait::async_trait]
impl CallerConstructionApi for SnarkosConstructionApi {}

impl ConstructionApi for SnarkosConstructionApi {}
