use mentat::api::{CallerConstructionApi, ConstructionApi};

#[derive(Default)]
pub struct SnarkosConstructionApi;

#[rocket::async_trait]
impl CallerConstructionApi for SnarkosConstructionApi {}

impl ConstructionApi for SnarkosConstructionApi {}
