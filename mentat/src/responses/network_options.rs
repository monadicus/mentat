use crate::misc::Version;

use super::*;

/// NetworkOptionsResponse contains information about the versioning of the node and the allowed operation statuses, operation types, and errors.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NetworkOptionsResponse {
    /// The Version object is utilized to inform the client of the versions of different components of the Rosetta implementation.
    pub version: Version,
    /// Allow specifies supported Operation status, Operation types, and all possible error statuses. This Allow object is used by clients to validate the correctness of a Rosetta Server implementation. It is expected that these clients will error if they receive some response that contains any of the above information that is not specified here.
    pub allow: Allow,
}
