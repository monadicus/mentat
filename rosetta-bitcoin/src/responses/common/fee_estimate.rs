use mentat::serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct FeeEstimate {
    pub feerate: f64,
    pub errors: Vec<String>,
    pub blocks: usize,
}
