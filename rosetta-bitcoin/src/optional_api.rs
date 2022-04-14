use mentat::{api::OptionalApi, axum::async_trait};
#[derive(Clone, Default)]
pub struct BitcoinOptionalApi;

#[async_trait]
impl OptionalApi for BitcoinOptionalApi {}
