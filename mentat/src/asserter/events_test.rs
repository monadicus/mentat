use indexmap::{indexmap, IndexMap};

use crate::{
    asserter::{
        errors::{AsserterError, BlockError, EventError},
        events::events_blocks_response,
    },
    types::{BlockEvent, BlockEventType, BlockIdentifier, EventsBlocksResponse},
};

struct EventsBlocksResponseTest {
    resp: EventsBlocksResponse,
    err: Option<AsserterError>,
}

#[test]
fn test_events_block_response() {
    let tests: IndexMap<&str, EventsBlocksResponseTest> = indexmap!(
      "no events" => EventsBlocksResponseTest {
        resp: Default::default(),
        err: None
      },
      // TODO make max_sequence a i64
      // "invalid max" => EventsBlocksResponseTest {
      //   resp: EventsBlocksResponse {
      //     max_sequence: -1,
      //     events: Vec::new(),
      //   },
      //   err: Some(EventError::MaxSequenceInvalid.into())
      // },
      "valid event" => EventsBlocksResponseTest {
        resp: EventsBlocksResponse {
          max_sequence: 100,
          events: vec![
            BlockEvent {
              sequence: 0,
              block_identifier: BlockIdentifier { index: 0, hash: 0.to_string() },
              type_: BlockEventType::BlockAdded,
            },
            BlockEvent {
              sequence: 1,
              block_identifier: BlockIdentifier { index: 0, hash: 0.to_string() },
              type_: BlockEventType::BlockRemoved,
            },
          ],
        },
        err: None
      },
      "invalid identifier" => EventsBlocksResponseTest {
        resp: EventsBlocksResponse {
          max_sequence: 100,
          events: vec![
            BlockEvent {
              sequence: 0,
              block_identifier: BlockIdentifier { index: 0, hash: String::new() },
              type_: BlockEventType::BlockAdded,
            },
            BlockEvent {
              sequence: 1,
              block_identifier: BlockIdentifier { index: 0, hash: 0.to_string() },
              type_: BlockEventType::BlockRemoved,
            },
          ],
        },
        err: Some(BlockError::BlockIdentifierHashMissing.into())
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
      "gap events" => EventsBlocksResponseTest {
        resp: EventsBlocksResponse {
          max_sequence: 100,
          events: vec![
            BlockEvent {
              sequence: 0,
              block_identifier: BlockIdentifier { index: 0, hash: 0.to_string() },
              type_: BlockEventType::BlockAdded,
            },
            BlockEvent {
              sequence: 2,
              block_identifier: BlockIdentifier { index: 0, hash: 0.to_string() },
              type_: BlockEventType::BlockRemoved,
            },
          ],
        },
        err: Some(EventError::SequenceOutOfOrder.into())
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
    );

    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        let res = events_blocks_response(&test.resp);
        if let Err(err) = res {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}