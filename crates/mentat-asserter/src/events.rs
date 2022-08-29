//! Validates that event data is correct.

use super::*;

/// [`BlockEvent`] ensures a *types.BlockEvent
/// is valid.
pub fn block_event(event: Option<&UncheckedBlockEvent>) -> AssertResult<()> {
    // TODO coinbase never checks if event nil
    let event = event.unwrap();

    if event.sequence < 0 {
        Err(EventError::SequenceInvalid)?;
    }

    block_identifier(event.block_identifier.as_ref())?;

    if !event.type_.valid() {
        Err(EventError::BlockEventTypeInvalid)?
    } else {
        Ok(())
    }
}

/// events_blocks_response ensures a [`EventsBlocksResponse`]
/// is valid.
pub fn events_blocks_response(
    response: Option<&UncheckedEventsBlocksResponse>,
) -> AssertResult<()> {
    // TODO: coinbase never checks for nil
    let response = response.unwrap();

    if response.max_sequence < 0 {
        Err(EventError::MaxSequenceInvalid)?;
    }
    let mut seq = -1;
    for (i, event) in response.events.iter().enumerate() {
        block_event(event.as_ref())?;
        let event = event.as_ref().unwrap();

        if seq == -1 {
            seq = event.sequence
        }
        if event.sequence != seq + (i as isize) {
            Err(EventError::SequenceOutOfOrder)?;
        }
    }
    Ok(())
}
