use super::test_utils::AsserterTest;
use crate::{
    asserter::{
        errors::{BlockError, EventError},
        events::events_blocks_response,
    },
    types::{
        BlockIdentifier, NullableBlockEvent, NullableBlockEventType, NullableEventsBlocksResponse,
    },
};

#[test]
fn test_events_block_response() {
    let tests = [
        AsserterTest {
            name: "no events",
            payload: Some(Default::default()),
            err: None,
        },
        AsserterTest {
            name: "invalid max",
            payload: Some(NullableEventsBlocksResponse {
                max_sequence: -1,
                events: Vec::new(),
            }),
            err: Some(EventError::MaxSequenceInvalid.into()),
        },
        AsserterTest {
            name: "valid event",
            payload: Some(NullableEventsBlocksResponse {
                max_sequence: 100,
                events: vec![
                    Some(NullableBlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: NullableBlockEventType::BLOCK_ADDED.into(),
                    }),
                    Some(NullableBlockEvent {
                        sequence: 1,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: NullableBlockEventType::BLOCK_REMOVED.into(),
                    }),
                ],
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid identifier",
            payload: Some(NullableEventsBlocksResponse {
                max_sequence: 100,
                events: vec![
                    Some(NullableBlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: String::new(),
                        }),
                        type_: NullableBlockEventType::BLOCK_ADDED.into(),
                    }),
                    Some(NullableBlockEvent {
                        sequence: 1,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: NullableBlockEventType::BLOCK_REMOVED.into(),
                    }),
                ],
            }),
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        AsserterTest {
            name: "invalid event type",
            payload: Some(NullableEventsBlocksResponse {
                max_sequence: 100,
                events: vec![
                    Some(NullableBlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: "revert".into(),
                    }),
                    Some(NullableBlockEvent {
                        sequence: 1,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: NullableBlockEventType::BLOCK_REMOVED.into(),
                    }),
                ],
            }),
            err: Some(EventError::BlockEventTypeInvalid.into()),
        },
        AsserterTest {
            name: "gap events",
            payload: Some(NullableEventsBlocksResponse {
                max_sequence: 100,
                events: vec![
                    Some(NullableBlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: NullableBlockEventType::BLOCK_ADDED.into(),
                    }),
                    Some(NullableBlockEvent {
                        sequence: 2,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: NullableBlockEventType::BLOCK_REMOVED.into(),
                    }),
                ],
            }),
            err: Some(EventError::SequenceOutOfOrder.into()),
        },
        AsserterTest {
            name: "gap events",
            payload: Some(NullableEventsBlocksResponse {
                max_sequence: 100,
                events: vec![
                    Some(NullableBlockEvent {
                        sequence: -1,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: NullableBlockEventType::BLOCK_ADDED.into(),
                    }),
                    Some(NullableBlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: NullableBlockEventType::BLOCK_REMOVED.into(),
                    }),
                ],
            }),
            err: Some(EventError::SequenceInvalid.into()),
        },
    ];

    AsserterTest::non_asserter_tests(&tests, events_blocks_response);
}
