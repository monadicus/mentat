use super::{block::block_identifier, errors::AssertResult};
use crate::{
    asserter::errors::EventError,
    models::{BlockEvent, BlockEventType},
    responses::EventsBlocksResponse,
};

/// [`BlockEvent`] ensures a *types.BlockEvent
/// is valid.
pub(crate) fn block_event(event: &BlockEvent) -> AssertResult<()> {
    if event.sequence < 0 {
        todo!("impossible case");
        return Err(EventError::SequenceInvalid.into());
    }
    block_identifier(&event.block_identifier)?;

    // TODO only two enum types
    // match event.type_ {
    //     BlockEventType::BlockAdded => Ok(()),
    //     BlockEventType::BlockRemoved => Ok(()),
    // }
    Ok(())
}

/// events_blocks_response ensures a [`EventsBlocksResponse`]
/// is valid.
pub(crate) fn events_blocks_response(response: &EventsBlocksResponse) -> AssertResult<()> {
    // TODO impossible case
    if response.max_sequence < 0 {
        todo!("impossible case");
        return Err(EventError::MaxSequenceInvalid.into());
    }
    let mut seq = -1;
    for (i, event) in response.events.iter().enumerate() {
        block_event(event)?;
        if seq == -1 {
            seq = event.sequence as i64
        }
        if event.sequence as i64 != seq + i as i64 {
            return Err(EventError::SequenceOutOfOrder.into());
        }
    }
    Ok(())
}
