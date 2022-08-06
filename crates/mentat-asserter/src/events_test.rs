use super::*;

#[test]
fn test_events_block_response() {
    let tests = vec![
        FnTest {
            name: "no events",
            payload: Default::default(),
            result: None,
        },
        FnTest {
            name: "invalid max",
            payload: NullableEventsBlocksResponse {
                max_sequence: -1,
                events: Vec::new(),
            },
            result: Some(EventError::MaxSequenceInvalid.into()),
        },
        FnTest {
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
        FnTest {
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
        FnTest {
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
        FnTest {
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
        FnTest {
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

    FnTest::run_err_match(tests, |t| events_blocks_response(Some(&t)));
}
