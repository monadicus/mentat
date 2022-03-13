use indexmap::IndexMap;

use super::*;

/// CallRequest is the input to the /call endpoint.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CallRequest {
    /// The network_identifier specifies which network a particular object is
    /// associated with.
    pub network_identifier: NetworkIdentifier,
    /// Method is some network-specific procedure call. This method could map to
    /// a network-specific RPC endpoint, a method in an SDK generated from a
    /// smart contract, or some hybrid of the two. The implementation must
    /// define all available methods in the Allow object. However, it is up to
    /// the caller to determine which parameters to provide when invoking /call.
    pub method:             String,
    /// Parameters is some network-specific argument for a method. It is up to
    /// the caller to determine which parameters to provide when invoking /call.
    pub parameters:         IndexMap<String, Value>,
}
