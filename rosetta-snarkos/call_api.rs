use mentat::api::{CallApi, CallerCallApi};

#[derive(Default)]
pub struct SnarkosCallApi;

#[async_trait::async_trait]
impl CallerCallApi for SnarkosCallApi {}

impl CallApi for SnarkosCallApi {}
