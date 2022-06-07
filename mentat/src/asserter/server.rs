use indexmap::{IndexMap, IndexSet};

use super::{
    errors::AssertResult,
    network::{contains_network_identifier, network_identifier},
};
use crate::identifiers::NetworkIdentifier;

/// SupportedNetworks returns an error if there is an invalid
/// types.NetworkIdentifier or there is a duplicate.
pub(crate) fn supported_networks(networks: &[NetworkIdentifier]) -> AssertResult<()> {
    if networks.is_empty() {
        return todo!();
    }

    let mut parsed = Vec::new();
    for network in networks {
        network_identifier(network)?;

        if contains_network_identifier(networks, network) {
            Err(format!("{}: {network:?}", todo!()))?;
        }
        parsed.push(network);
    }

    Ok(())
}
