use mentat::api::{CallApi, CallerCallApi};

#[derive(Default)]
pub struct SnarkosCallApi;

#[rocket::async_trait]
impl CallerCallApi for SnarkosCallApi {}

impl CallApi for SnarkosCallApi {}
