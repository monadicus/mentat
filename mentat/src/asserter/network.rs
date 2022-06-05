use super::util::{hash, string_array};
use crate::identifiers::{NetworkIdentifier, SubNetworkIdentifier};

/// SubNetworkIdentifier asserts a types.SubNetworkIdentifer is valid (if not
/// nil).
pub(crate) fn sub_network_identifier(
    sub_network_identifier: Option<&SubNetworkIdentifier>,
) -> Result<(), String> {
    let sub_network_identifier = match sub_network_identifier {
		None => return Ok(()),
		Some(sni) => sni,
	};

    if sub_network_identifier.network.is_empty() {
        return todo!();
    }

    Ok(())
}

/// NetworkIdentifier ensures a types.NetworkIdentifier has
/// a valid blockchain and network.
pub(crate) fn network_identifier(network: &NetworkIdentifier) -> Result<(), String> {
    // TODO if nil
    if network.blockchain.is_empty() {
        return todo!();
    }

    if network.network.is_empty() {
        return todo!();
    }

    sub_network_identifier(network.sub_network_identifier.as_ref())
}

/// OperationTypes ensures all items in Options.Allow.OperationStatuses
/// are valid and that there are no repeats.
pub(crate) fn operation_types(types: &[String]) -> Result<(), String> {
    string_array("Allow.OperationTypes", types)
}

/// containsNetworkIdentifier returns a boolean indicating if a
/// *types.NetworkIdentifier is contained within a slice of
/// *types.NetworkIdentifier. The check for equality takes
/// into account everything within the types.NetworkIdentifier
/// struct (including currency.Metadata).
pub(crate) fn contains_network_identifier(
    networks: &[NetworkIdentifier],
    network: &NetworkIdentifier,
) -> bool {
    networks.iter().any(|other| hash(other) == hash(network))
}
