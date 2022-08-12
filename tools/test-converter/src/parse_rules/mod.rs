use indexmap::{indexmap, IndexMap};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use toml::Value;

lazy_static! {
    pub(crate) static ref TYPE_PARSE_RULES: IndexMap<&'static str, &'static str> = indexmap!(
      "[](.+){" => "Vec<($1)>",
      "[]*(.+){" => "Vec<($1)>",
      "*(.+)" => "Option<<($1)",
      "{}" => "Default::default()",
      "&(.+)" => "Option<$1>",
    );
    pub(crate) static ref VALUE_PARSE_RULES: IndexMap<&'static str, &'static str> = indexmap!(
      "[](.+){" => "vec![",
      "[]*(.+){" => "vec![",
      "{}" => "Default::default()",
      "&(.+)" => "",
    );

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum Payload {
    Dynamic {
        name: String,
        values: IndexMap<String, String>,
    },
    Single {
        name: String,
        value: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CustomParseRules {
    struct_name: String,
    payload_struct_name: Option<String>,
    criteria_struct_name: Option<String>,
    rules: IndexMap<String, String>,
}


impl CustomParseRules {
    fn convert(self) {
      
    }
}

// to
// TestCase {
//   name: "simple transfer (with extra op)",
//   payload: MatchOperationsTest {
//       operations: vec![
//           Some(Operation {
//               account: Some(AccountIdentifier {
//                   address: "addr2",
//                   ..Default::default()
//               }),
//               amount: Some(Amount {
//                   value: "100",
//                   ..Default::default()
//               }),
//               ..Default::default()
//           }),
//           // extra op ignored
//           Some(Operation::default()),
//           Some(Operation {
//               account: Some(AccountIdentifier {
//                   address: "addr1",
//                   ..Default::default()
//               }),
//               amount: Some(Amount {
//                   value: "-100",
//                   ..Default::default()
//               }),
//               ..Default::default()
//           }),
//       ],
//       descriptions: Descriptions {
//           opposite_amounts: vec![vec![0, 1]],
//           operation_descriptions: vec![
//               Some(OperationDescription {
//                   account: Some(AccountDescription {
//                       exists: true,
//                       ..Default::default()
//                   }),
//                   amount: Some(AmountDescription {
//                       exists: true,
//                       sign: AmountSign::NEGATIVE,
//                       ..Default::default()
//                   }),
//                   ..Default::default()
//               }),
//               Some(OperationDescription {
//                   account: Some(AccountDescription {
//                       exists: true,
//                       ..Default::default()
//                   }),
//                   amount: Some(AmountDescription {
//                       exists: true,
//                       sign: AmountSign::POSITIVE,
//                       ..Default::default()
//                   }),
//                   ..Default::default()
//               }),
//           ],
//           ..Default::default()
//       },
//   },
//   criteria: Some(vec![
//       Some(Match {
//           operations: vec![Some(Operation {
//               account: Some(AccountIdentifier {
//                   address: "addr1",
//                   ..Default::default()
//               }),
//               amount: Some(Amount {
//                   value: "-100",
//                   ..Default::default()
//               }),
//               ..Default::default()
//           })],
//           amounts: vec![Some(BigInt::from(-100))],
//       }),
//       Some(Match {
//           operations: vec![Some(Operation {
//               account: Some(AccountIdentifier {
//                   address: "addr2",
//                   ..Default::default()
//               }),
//               amount: Some(Amount {
//                   value: "100",
//                   ..Default::default()
//               }),
//               ..Default::default()
//           })],
//           amounts: vec![Some(BigInt::from(100))],
//       }),
//   ]),
// }
