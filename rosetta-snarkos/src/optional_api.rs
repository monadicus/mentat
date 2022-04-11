use mentat::{api::OptionalApi, axum::async_trait};

#[derive(Clone, Default)]
pub struct SnarkosOptionalApi;

#[async_trait]
impl OptionalApi for SnarkosOptionalApi {}
