use indexmap::{IndexMap, IndexSet};

use super::network::{contains_network_identifier, network_identifier};
use crate::identifiers::NetworkIdentifier;

/// SupportedNetworks returns an error if there is an invalid
/// types.NetworkIdentifier or there is a duplicate.
pub(crate) fn supported_networks(networks: &[NetworkIdentifier]) -> Result<(), String> {
    if networks.is_empty() {
        return todo!();
    }

    let mut parsed = Vec::new();
    for network in networks {
        network_identifier(network)?;

        if contains_network_identifier(networks, network) {
            return Err(format!("{}: {network:?}", todo!()));
        }
        parsed.push(network);
    }

    Ok(())
}
