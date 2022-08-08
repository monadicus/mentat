use super::*;

#[test]
fn test_events_block_response() {
    let tests = vec![
        TestCase {
            name: "no events",
            payload: Default::default(),
            result: None,
        },
        TestCase {
            name: "invalid max",
            payload: NullableEventsBlocksResponse {
                max_sequence: -1,
                events: Vec::new(),
            },
            result: Some(EventError::MaxSequenceInvalid.into()),
        },
        TestCase {
            name: "valid event",
            payload: NullableEventsBlocksResponse {
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
            },
            result: None,
        },
        TestCase {
            name: "invalid identifier",
            payload: NullableEventsBlocksResponse {
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
            },
            result: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
        TestCase {
            name: "invalid event type",
            payload: NullableEventsBlocksResponse {
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
            },
            result: Some(EventError::BlockEventTypeInvalid.into()),
        },
        TestCase {
            name: "gap events",
            payload: NullableEventsBlocksResponse {
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
            },
            result: Some(EventError::SequenceOutOfOrder.into()),
        },
        TestCase {
            name: "gap events",
            payload: NullableEventsBlocksResponse {
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
            },
            result: Some(EventError::SequenceInvalid.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| events_blocks_response(Some(&t)));
}
