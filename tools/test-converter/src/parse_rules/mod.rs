use std::{io::Read, path::PathBuf};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::errors::{Result, RulesFileError};

// lazy_static! {
//     pub(crate) static ref TYPE_PARSE_RULES: IndexMap<&'static str, &'static
// str> = indexmap!(       "[](.+){" => "Vec<($1)>",
//       "[]*(.+){" => "Vec<($1)>",
//       "*(.+)" => "Option<<($1)",
//       "{}" => "Default::default()",
//       "&(.+)" => "Option<$1>",
//     );
//     pub(crate) static ref VALUE_PARSE_RULES: IndexMap<&'static str, &'static
// str> = indexmap!(       "[](.+){" => "vec![",
//       "[]*(.+){" => "vec![",
//       "{}" => "Default::default()",
//       "&(.+)" => "",
//       "nil" => "None",
//     );
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct TestStructCriteria {
    #[serde(rename = "type")]
    pub type_: String,
    pub from: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestStructPayloadField {
    #[serde(rename = "type")]
    pub type_: String,
    pub from: String,
    #[serde(flatten)]
    pub sub_fields: IndexMap<String, Self>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TestStructPayload {
    Dynamic {
        struct_name: String,
        #[serde(flatten)]
        fields: IndexMap<String, TestStructPayloadField>,
    },
    Single {
        struct_name: String,
        value: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TypeOverrideTypes {
    Replace,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestStruct {
    pub test_fn_name: String,
    pub struct_name: String,
    pub struct_method: String,
    pub closure: String,
    pub criteria: TestStructCriteria,
    pub payload: TestStructPayload,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypeOverride {
    rule_type: TypeOverrideTypes,
    #[serde(flatten)]
    rules: IndexMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RulesFile {
    pub test_struct: TestStruct,
    pub types: IndexMap<String, TypeOverride>,
}

impl RulesFile {
    fn file(file: PathBuf, write: bool, create: bool) -> Result<std::fs::File> {
        RulesFileError::could_not_open_config_file(
            std::fs::OpenOptions::new()
                .read(true)
                .write(write)
                .create(create)
                .open(&file),
            file.to_str().unwrap_or("invalid path"),
        )
    }

    pub(crate) fn from_toml_file(path: PathBuf) -> Result<Self> {
        let mut file = Self::file(path, false, false)?;
        let mut toml = String::new();
        RulesFileError::failed_to_read_rules(file.read_to_string(&mut toml))?;
        let conf: Self = toml::from_str(&toml).expect("Failed to deserialize toml config");
        Ok(conf)
    }
}
