use mentat::{
    api::{CallerConstructionApi, ConstructionApi},
    async_trait,
};

#[derive(Default)]
pub struct SnarkosConstructionApi;

#[async_trait]
impl CallerConstructionApi for SnarkosConstructionApi {}

impl ConstructionApi for SnarkosConstructionApi {}
