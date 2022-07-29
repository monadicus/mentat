//! The module defines the `CallRequest` request.

use indexmap::IndexMap;

use super::*;

/// `CallRequest` is the input to the `/call` endpoint.
#[derive(Debug, Default, Deserialize, Serialize, Nullable)]
#[serde(default)]
pub struct NullableCallRequest {
    /// The [`NetworkIdentifier`] specifies which network a particular object is
    /// associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_identifier: Option<NetworkIdentifier>,
    /// Method is some network-specific procedure call. This method could map to
    /// a network-specific RPC endpoint, a method in an SDK generated from a
    /// smart contract, or some hybrid of the two. The implementation must
    /// define all available methods in the [`Allow`] object. However, it is up
    /// to the caller to determine which parameters to provide when invoking
    /// `/call`.
    pub method: String,
    /// Parameters is some network-specific argument for a method. It is up to
    /// the caller to determine which parameters to provide when invoking
    /// `/call`.
    #[serde(default)]
    pub parameters: IndexMap<String, Value>,
}
