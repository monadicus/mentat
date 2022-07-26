//! Validates that network data is correct.

use indexmap::IndexSet;

use super::{
    block_identifier, currency, hash, string_array, timestamp, AssertResult, AsserterError,
    BlockError, ErrorError, MentatError, NetworkError, NetworkIdentifier, NullableAllow,
    NullableBalanceExemption, NullableNetworkListResponse, NullableNetworkOptionsResponse,
    NullableNetworkStatusResponse, OperationStatus, Peer, SubNetworkIdentifier, SyncStatus,
    Version,
};

/// `sub_network_identifier` asserts a [`SubNetworkIdentifier`] is valid (if not
/// nil).
pub(crate) fn sub_network_identifier(
    sub_network_identifier: Option<&SubNetworkIdentifier>,
) -> AssertResult<()> {
    let sub_network_identifier = match sub_network_identifier {
        None => return Ok(()),
        Some(sni) => sni,
    };

    if sub_network_identifier.network.is_empty() {
        Err(NetworkError::SubNetworkIdentifierInvalid)?;
    }

    Ok(())
}

/// `network_identifier` ensures a [`NetworkIdentifier`] has
/// a valid blockchain and network.
pub(crate) fn network_identifier(network: Option<&NetworkIdentifier>) -> AssertResult<()> {
    let network = network.ok_or(NetworkError::NetworkIdentifierIsNil)?;

    if network.blockchain.is_empty() {
        Err(NetworkError::NetworkIdentifierBlockchainMissing)?;
    }

    if network.network.is_empty() {
        Err(NetworkError::NetworkIdentifierNetworkMissing)?;
    }

    sub_network_identifier(network.sub_network_identifier.as_ref())
}

/// peer ensures a [`Peer`] has a valid peer_id.
pub(crate) fn peer(peer: Option<&Peer>) -> AssertResult<()> {
    let peer = peer.ok_or(NetworkError::PeerIDMissing)?;

    if peer.peer_id.is_empty() {
        Err(NetworkError::PeerIDMissing)?;
    }

    Ok(())
}

/// `version` ensures the [`Version`] of the node is
/// returned.
pub(crate) fn version(version: Option<&Version>) -> AssertResult<()> {
    let version = version.ok_or(NetworkError::VersionIsNil)?;

    if version.node_version.is_empty() {
        Err(NetworkError::VersionNodeVersionMissing)?;
    }

    if version.middleware_version.is_some()
        && version.middleware_version.as_ref().unwrap().is_empty()
    {
        Err(NetworkError::VersionMiddlewareVersionMissing)?;
    }

    Ok(())
}

/// `sync_status` ensures any [`SyncStatus`] is valid.
pub(crate) fn sync_status(status: Option<&SyncStatus>) -> AssertResult<()> {
    let status = match status {
        Some(s) => s,
        None => return Ok(()),
    };

    if status.current_index.is_none() || status.current_index.unwrap() < 0 {
        Err(NetworkError::SyncStatusCurrentIndexNegative)?;
    }

    if status.target_index.is_none() || status.target_index.unwrap() < 0 {
        Err(NetworkError::SyncStatusTargetIndexNegative)?;
    }

    if status.stage.is_none() || status.stage.as_ref().unwrap().is_empty() {
        Err(NetworkError::SyncStatusStageInvalid)?;
    }

    Ok(())
}

/// `network_status_response` ensures any [`NetworkStatusResponse`]
/// is valid.
pub(crate) fn network_status_response(
    resp: Option<&NullableNetworkStatusResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(NetworkError::NetworkStatusResponseIsNil)?;

    block_identifier(resp.current_block_identifier.as_ref())?;
    timestamp(resp.current_block_timestamp as i64)?;
    block_identifier(resp.genesis_block_identifier.as_ref())?;
    resp.peers
        .iter()
        .map(|p| peer(p.as_ref()))
        .collect::<AssertResult<Vec<_>>>()?;
    sync_status(resp.sync_status.as_ref())
}

/// `operation_statuses` ensures all [`OperationStatus`] in
/// [`OperationStatuses`] are valid and that there exists at least 1
/// successful status.
pub(crate) fn operation_statuses(stats: &[Option<OperationStatus>]) -> AssertResult<()> {
    if stats.is_empty() {
        Err(NetworkError::NoAllowedOperationStatuses)?;
    }

    let mut statuses = Vec::new();
    let mut found_success = false;
    for status in stats {
        // TODO coinbase never checks for nil here
        let status = status.as_ref().unwrap();

        if status.status.is_empty() {
            Err(BlockError::OperationStatusMissing)?;
        }

        if status.successful {
            found_success = true;
        }

        statuses.push(status.status.clone());
    }

    if !found_success {
        Err(NetworkError::NoSuccessfulAllowedOperationStatuses)?;
    }

    string_array("Allow.OperationStatuses", &statuses).map_err(AsserterError::from)
}

/// `operation_types` ensures all items in Options.Allow.OperationStatuses
/// are valid and that there are no repeats.
pub(crate) fn operation_types(types: &[String]) -> AssertResult<()> {
    string_array("Allow.OperationTypes", types).map_err(AsserterError::from)
}

/// `error` ensures a [`MentatError`] is valid.
pub(crate) fn error(err: Option<&MentatError>) -> AssertResult<()> {
    let err = err.ok_or(ErrorError::IsNil)?;

    if err.code < 0 {
        Err(ErrorError::CodeIsNeg)?
    } else if err.message.is_empty() {
        Err(ErrorError::MessageMissing)?
    } else if err.description.is_some() && err.description.as_ref().unwrap().is_empty() {
        Err(ErrorError::DescriptionEmpty)?
    } else {
        Ok(())
    }
}

/// `errors` ensures each [`MentatError`] in a slice is valid
/// and that there is no error code collision.
pub(crate) fn errors(errors: &[Option<MentatError>]) -> AssertResult<()> {
    let mut status_codes = IndexSet::new();

    for err in errors {
        error(err.as_ref())?;
        let err = err.as_ref().unwrap();

        if !err.details.is_empty() {
            Err(NetworkError::ErrorDetailsPopulated)?;
        }

        if status_codes.contains(&err.code) {
            Err(NetworkError::ErrorCodeUsedMultipleTimes)?;
        }

        status_codes.insert(err.code);
    }

    Ok(())
}

/// `balance_exemptions` ensures [`BalanceExemption`]] in a slice is valid.
pub(crate) fn balance_exemptions(
    exemptions: &[Option<NullableBalanceExemption>],
) -> AssertResult<()> {
    for (index, exemption) in exemptions.iter().enumerate() {
        let exemption = exemption.as_ref().ok_or(format!(
            "{} (index {})",
            NetworkError::BalanceExemptionIsNil,
            index
        ))?;

        if !exemption.exemption_type.valid() {
            Err(format!(
                "{} (index {}): {}",
                NetworkError::BalanceExemptionTypeInvalid,
                index,
                exemption.exemption_type
            ))?;
        }

        if exemption.currency.is_none() && exemption.sub_account_address.is_none() {
            Err(format!(
                "{} (index {index})",
                NetworkError::BalanceExemptionMissingSubject
            ))?
        }

        if exemption.currency.is_some() {
            currency(exemption.currency.as_ref())
                .map_err(|err| format!("{err} (index {index})"))?;
        }

        if exemption.sub_account_address.is_some()
            && exemption.sub_account_address.as_ref().unwrap().is_empty()
        {
            Err(format!(
                "{} (index {index})",
                NetworkError::BalanceExemptionSubAccountAddressEmpty
            ))?
        }
    }

    Ok(())
}

/// `call_methods` ensures Allow.CallMethods are valid.
pub(crate) fn call_methods(methods: &[String]) -> AssertResult<()> {
    if methods.is_empty() {
        return Ok(());
    }

    string_array("Allow.CallMethods", methods)
}

/// `allow` ensures a [`Allow`] object is valid.
pub(crate) fn allow(allowed: Option<&NullableAllow>) -> AssertResult<()> {
    let allowed = allowed.ok_or(NetworkError::AllowIsNil)?;

    operation_statuses(&allowed.operation_statuses)?;
    operation_types(&allowed.operation_types)?;
    errors(&allowed.errors)?;
    call_methods(&allowed.call_methods)?;
    balance_exemptions(&allowed.balance_exemptions)?;

    if !allowed.balance_exemptions.is_empty() && !allowed.historical_balance_lookup {
        Err(NetworkError::BalanceExemptionNoHistoricalLookup)?;
    }

    if allowed.timestamp_start_index.is_some() && allowed.timestamp_start_index.unwrap() < 0 {
        Err(format!(
            "{}: {}",
            NetworkError::TimestampStartIndexInvalid,
            allowed.timestamp_start_index.unwrap()
        ))?
    }

    Ok(())
}

/// `network_options_response` ensures a [`NetworkOptionsResponse`] object is
/// valid.
pub(crate) fn network_options_response(
    options: Option<&NullableNetworkOptionsResponse>,
) -> AssertResult<()> {
    let options = options.ok_or(NetworkError::NetworkOptionsResponseIsNil)?;
    version(options.version.as_ref())?;
    allow(options.allow.as_ref())
}

/// `contains_network_identifier` returns a boolean indicating if a
/// [`NetworkIdentifier`] is contained within a slice of
/// [`NetworkIdentifier`]. The check for equality takes
/// into account everything within the NetworkIdentifier
/// struct (including currency.Metadata).
pub(crate) fn contains_network_identifier(
    networks: &[NetworkIdentifier],
    network: Option<&NetworkIdentifier>,
) -> bool {
    networks
        .iter()
        .any(|other| hash(Some(other)) == hash(network))
}

/// `network_list_response` ensures a [`NetworkListResponse`] object is valid.
pub(crate) fn network_list_response(
    resp: Option<&NullableNetworkListResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(NetworkError::NetworkListResponseIsNil)?;
    let mut seen = Vec::new();
    for network in &resp.network_identifiers {
        network_identifier(network.as_ref())?;
        if contains_network_identifier(&seen, network.as_ref()) {
            Err(NetworkError::NetworkListResponseNetworksContainsDuplicates)?;
        }
        seen.push(network.clone().unwrap());
    }
    Ok(())
}
