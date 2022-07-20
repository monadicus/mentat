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
        // TODO make max_sequence a i64
        // "invalid max" => EventsBlocksResponseTest {
        //   resp: EventsBlocksResponse {
        //     max_sequence: -1,
        //     events: Vec::new(),
        //   },
        //   err: Some(EventError::MaxSequenceInvalid.into())
        // },
        AsserterTest {
            name: "valid event",
            payload: EventsBlocksResponse {
                max_sequence: 100,
                events: vec![
                    BlockEvent {
                        sequence: 0,
                        block_identifier: BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        },
                        type_: BlockEventType::BlockAdded,
                    },
                    BlockEvent {
                        sequence: 1,
                        block_identifier: BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        },
                        type_: BlockEventType::BlockRemoved,
                    },
                ],
            },
            err: None,
        },
        AsserterTest {
            name: "invalid identifier",
            payload: EventsBlocksResponse {
                max_sequence: 100,
                events: vec![
                    BlockEvent {
                        sequence: 0,
                        block_identifier: BlockIdentifier {
                            index: 0,
                            hash: String::new(),
                        },
                        type_: BlockEventType::BlockAdded,
                    },
                    BlockEvent {
                        sequence: 1,
                        block_identifier: BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        },
                        type_: BlockEventType::BlockRemoved,
                    },
                ],
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        // TODO allow arbitrary block event types
        // "invalid event type" => EventsBlocksResponseTest {
        //   resp: EventsBlocksResponse {
        //     max_sequence: 100,
        //     events: vec![
        //       BlockEvent {
        //         sequence: 0,
        //         block_identifier: BlockIdentifier { index: 0, hash: 0.to_string() },
        //         type_: "revert",
        //       },
        //       BlockEvent {
        //         sequence: 1,
        //         block_identifier: BlockIdentifier { index: 0, hash: 0.to_string() },
        //         type_: BlockEventType::BlockRemoved,
        //       },
        //     ],
        //   },
        //   err: Some(EventError::BlockEventTypeInvalid.into())
        // },
        AsserterTest {
            name: "gap events",
            payload: EventsBlocksResponse {
                max_sequence: 100,
                events: vec![
                    BlockEvent {
                        sequence: 0,
                        block_identifier: BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        },
                        type_: BlockEventType::BlockAdded,
                    },
                    BlockEvent {
                        sequence: 2,
                        block_identifier: BlockIdentifier {
                            index: 0,
                            hash: 0.to_string(),
                        },
                        type_: BlockEventType::BlockRemoved,
                    },
                ],
            },
            err: Some(EventError::SequenceOutOfOrder.into()),
        },
        // TODO allow i64 sequence
        // "gap events" => EventsBlocksResponseTest {
        //   resp: EventsBlocksResponse {
        //     max_sequence: 100,
        //     events: vec![
        //       BlockEvent {
        //         sequence: -1,
        //         block_identifier: BlockIdentifier { index: 0, hash: 0.to_string() },
        //         type_: BlockEventType::BlockAdded,
        //       },
        //       BlockEvent {
        //         sequence: 0,
        //         block_identifier: BlockIdentifier { index: 0, hash: 0.to_string() },
        //         type_: BlockEventType::BlockRemoved,
        //       },
        //     ],
        //   },
        //   err: Some(EventError::SequenceInvalid.into())
        // },
    ];

    AsserterTest::non_asserter_tests(&tests, events_blocks_response);
}
