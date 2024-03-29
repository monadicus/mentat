//! The module defines the `NetworkOptionsResponse` response.

use super::*;

/// [`NetworkOptionsResponse`] contains information about the versioning of the
/// node and the allowed operation statuses, operation types, and errors.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Unchecked)]
#[serde(default)]
pub struct UncheckedNetworkOptionsResponse {
    /// The [`Version`] object is utilized to inform the client of the versions
    /// of different components of the Rosetta implementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    /// [`Allow`] specifies [`supported Operation`] status, [`Operation types`],
    /// and all possible error statuses. This Allow object is used by
    /// clients to validate the correctness of a Rosetta Server
    /// implementation. It is expected that these clients will error if they
    /// receive some response that contains any of the above information
    /// that is not specified here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<UncheckedAllow>,
}
