use mentat::api::ConstructionApi;

#[derive(Default)]
pub struct SnarkosConstructionApi;

#[async_trait::async_trait]
impl ConstructionApi for SnarkosConstructionApi {
    // #[cfg(feature = "rosetta-snarkos-mainnet")]
    // #[cfg(feature = "rosetta-snarkos-testnet")]
}
