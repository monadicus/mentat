use std::env;

pub enum Mode {
    Online,
    Offline,
}

impl Mode {
    pub fn is_offline(&self) -> bool {
        matches!(&self, Mode::Offline)
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

pub type ModeState = rocket::State<Mode>;
