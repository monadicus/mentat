use indexmap::IndexMap;

use super::{util::{hash, string_array}, block::{block_identifier, timestamp}, errors::{NetworkError, AssertResult, BlockError, AsserterError, ErrorError}};
use crate::{
    identifiers::{NetworkIdentifier, SubNetworkIdentifier},
    misc::{OperationStatus, Peer, SyncStatus, Version}, responses::NetworkStatusResponse, errors::MentatError,
};

/// SubNetworkIdentifier asserts a types.SubNetworkIdentifer is valid (if not
/// nil).
pub(crate) fn sub_network_identifier(
    sub_network_identifier: Option<&SubNetworkIdentifier>,
) -> AssertResult<()> {
    let sub_network_identifier = match sub_network_identifier {
        None => return Ok(()),
        Some(sni) => sni,
    };

    if sub_network_identifier.network.is_empty() {
        return Err(NetworkError::SubNetworkIdentifierInvalid.into());
    }

    Ok(())
}

/// NetworkIdentifier ensures a types.NetworkIdentifier has
/// a valid blockchain and network.
pub(crate) fn network_identifier(network: &NetworkIdentifier) -> AssertResult<()> {
    // TODO if nil
    if network.blockchain.is_empty() {
        return Err(NetworkError::NetworkIdentifierBlockchainMissing.into());
    }

    if network.network.is_empty() {
        return Err(NetworkError::NetworkIdentifierNetworkMissing.into());
    }

    sub_network_identifier(network.sub_network_identifier.as_ref())
}

/// Peer ensures a types.Peer has a valid peer_id.
pub(crate) fn peer(peer: &Peer) -> AssertResult<()> {
	// or if p nil
	if peer.peer_id.is_empty() {
		return Err(NetworkError::PeerIDMissing.into());
	}

	Ok(())
}

/// Version ensures the version of the node is
/// returned.
pub(crate) fn version(version: &Version) -> AssertResult<()> {
	// if version nil
	if version.node_version.is_empty() {
		return Err(NetworkError::VersionNodeVersionMissing.into());
	}

	if version.middleware_version.is_none() || version.middleware_version.as_ref().unwrap().is_empty() {
		return Err(NetworkError::VersionMiddlewareVersionMissing.into());
	}

	Ok(())
}

/// SyncStatus ensures any types.SyncStatus is valid.
pub(crate) fn sync_status(status: Option<&SyncStatus>) -> AssertResult<()> {
	let status = match status {
		Some(s) => s,
		None => return Ok(()),
	};

	if status.current_index.is_none() || status.current_index.unwrap() < 0 {
		return Err(NetworkError::SyncStatusCurrentIndexNegative.into());
	}

	if status.target_index.is_none() || status.target_index.unwrap() < 0 {
		return Err(NetworkError::SyncStatusTargetIndexNegative.into());
	}

	if status.stage.is_none() || status.stage.as_ref().unwrap().is_empty() {
		return Err(NetworkError::SyncStatusStageInvalid.into());
	}

	Ok(())
}

/// NetworkStatusResponse ensures any types.NetworkStatusResponse
/// is valid.
pub(crate) fn network_status_response(resp: &NetworkStatusResponse) -> AssertResult<()> {
	// TODO if resp nil
	block_identifier(&resp.current_block_identifier)?;
	timestamp(resp.current_block_timestamp as i64)?;
	block_identifier(&resp.genesis_block_identifier)?;
	resp.peers.iter().map(peer).collect::<AssertResult<Vec<_>>>()?;
	sync_status(resp.sync_status.as_ref())
}

/// OperationStatuses ensures all items in Options.Allow.OperationStatuses
/// are valid and that there exists at least 1 successful status.
pub(crate) fn operation_statuses(stats: &[OperationStatus]) -> AssertResult<()> {
    if stats.is_empty() {
        return Err(NetworkError::NoAllowedOperationStatuses.into());
    }

    let mut statuses = Vec::new();
    let mut found_success = false;
    for status in stats {
        if status.status.is_empty() {
            return Err(BlockError::OperationStatusMissing.into());
        }

        if status.successful {
            found_success = true;
        }

        statuses.push(status.status.clone());
    }

    if !found_success {
        return Err(NetworkError::NoSuccessfulAllowedOperationStatuses.into());
    }

    string_array("Allow.OperationStatuses", &statuses).map_err(AsserterError::from)
}

/// OperationTypes ensures all items in Options.Allow.OperationStatuses
/// are valid and that there are no repeats.
pub(crate) fn operation_types(types: &[String]) -> AssertResult<()> {
    string_array("Allow.OperationTypes", types).map_err(AsserterError::from)
}

/// Error ensures a types.Error is valid.
pub(crate) fn error(err: MentatError) -> AssertResult<()> {
	// if err nil

	if err.code < 0 {
		return Err(ErrorError::IsNil.into());
	}

	if err.message.is_empty() {
		return Err(ErrorError::MessageMissing.into());
	}

	if err.description.is_none() || err.description.unwrap().is_empty() {
		return Err(ErrorError::DescriptionEmpty.into());
	}

	Ok(())
}

/// Errors ensures each types.Error in a slice is valid
/// and that there is no error code collision.
pub(crate) fn errors(errors: &[MentatError]) -> AssertResult<()> {
	Ok(())
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


