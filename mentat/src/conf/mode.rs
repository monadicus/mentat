use std::{env, fmt, str::FromStr};

use super::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Online,
    Offline,
}

pub type ModeState = axum::extract::Extension<Mode>;

impl Mode {
    pub fn is_offline(&self) -> bool {
        matches!(&self, &Mode::Offline)
    }
}

impl Default for Mode {
    fn default() -> Self {
        match env::var("MODE").as_deref() {
            Ok("OFFLINE") => Mode::Offline,
            _ => Mode::Online,
        }
    }
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Mode::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Mode::*;

        match self {
            Offline => write!(f, "OFFLINE"),
            Online => write!(f, "ONLINE"),
        }
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Mode::*;
        let s = s.to_uppercase();

        match s.as_ref() {
            "OFFLINE" => Ok(Offline),
            "ONLINE" => Ok(Online),
            _ => Err(format!("Unexpected mode {s}")),
        }
    }
}

impl Serialize for Mode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
