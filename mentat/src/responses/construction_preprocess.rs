use indexmap::IndexMap;

use super::*;

/// ConstructionPreprocessResponse contains options that will be sent unmodified
/// to /construction/metadata. If it is not necessary to make a request to
/// /construction/metadata, options should be omitted. Some blockchains require
/// the PublicKey of particular AccountIdentifiers to construct a valid
/// transaction. To fetch these PublicKeys, populate required_public_keys with
/// the AccountIdentifiers associated with the desired PublicKeys. If it is not
/// necessary to retrieve any PublicKeys for construction, required_public_keys
/// should be omitted.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ConstructionPreprocessResponse {
    /// The options that will be sent directly to /construction/metadata by the
    /// caller.
    #[serde(default)]
    pub options: IndexMap<String, Value>,
    pub required_public_keys: Option<Vec<AccountIdentifier>>,
}
