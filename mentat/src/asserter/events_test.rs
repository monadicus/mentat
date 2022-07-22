use super::test_utils::AsserterTest;
use crate::{
    asserter::{
        errors::{BlockError, EventError},
        events::events_blocks_response,
    },
    types::{BlockEvent, BlockEventType, BlockIdentifier, EventsBlocksResponse},
};

#[test]
fn test_events_block_response() {
    let tests = [
        AsserterTest {
            name: "no events",
            payload: Default::default(),
            err: None,
        },
        AsserterTest {
            name: "invalid max",
            payload: EventsBlocksResponse {
                max_sequence: -1,
                events: Some(Vec::new()),
            },
            err: Some(EventError::MaxSequenceInvalid.into()),
        },
        AsserterTest {
            name: "valid event",
            payload: EventsBlocksResponse {
                max_sequence: 100,
                events: Some(vec![
                    Some(BlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: BlockEventType::BLOCK_ADDED.into(),
                    }),
                    Some(BlockEvent {
                        sequence: 1,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: BlockEventType::BLOCK_REMOVED.into(),
                    }),
                ]),
            },
            err: None,
        },
        AsserterTest {
            name: "invalid identifier",
            payload: EventsBlocksResponse {
                max_sequence: 100,
                events: Some(vec![
                    Some(BlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: String::new(),
                        }),
                        type_: BlockEventType::BLOCK_ADDED.into(),
                    }),
                    Some(BlockEvent {
                        sequence: 1,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: BlockEventType::BLOCK_REMOVED.into(),
                    }),
                ]),
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        AsserterTest {
            name: "invalid event type",
            payload: EventsBlocksResponse {
                max_sequence: 100,
                events: Some(vec![
                    Some(BlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: "revert".into(),
                    }),
                    Some(BlockEvent {
                        sequence: 1,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: BlockEventType::BLOCK_REMOVED.into(),
                    }),
                ]),
            },
            err: Some(EventError::BlockEventTypeInvalid.into()),
        },
        AsserterTest {
            name: "gap events",
            payload: EventsBlocksResponse {
                max_sequence: 100,
                events: Some(vec![
                    Some(BlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: BlockEventType::BLOCK_ADDED.into(),
                    }),
                    Some(BlockEvent {
                        sequence: 2,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: BlockEventType::BLOCK_REMOVED.into(),
                    }),
                ]),
            },
            err: Some(EventError::SequenceOutOfOrder.into()),
        },
        AsserterTest {
            name: "gap events",
            payload: EventsBlocksResponse {
                max_sequence: 100,
                events: Some(vec![
                    Some(BlockEvent {
                        sequence: -1,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: BlockEventType::BLOCK_ADDED.into(),
                    }),
                    Some(BlockEvent {
                        sequence: 0,
                        block_identifier: Some(BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        }),
                        type_: BlockEventType::BLOCK_REMOVED.into(),
                    }),
                ]),
            },
            err: Some(EventError::SequenceInvalid.into()),
        },
    ];

    AsserterTest::non_asserter_tests(&tests, events_blocks_response);
}
