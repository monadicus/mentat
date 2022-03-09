use std::env;

pub enum Mode {
    Online,
    Offline,
    Neither,
}

impl Mode {
    pub fn is_offline(&self) -> bool {
        matches!(&self, Mode::Online)
    }

    pub fn is_online(&self) -> bool {
        matches!(&self, Mode::Online)
    }
}

impl Default for Mode {
    fn default() -> Self {
        match env::var("MODE").as_deref() {
            Ok("NEITHER") => Mode::Neither,
            Ok("OFFLINE") => Mode::Offline,
            _ => Mode::Online,
        }
    }
}

pub type ModeState = rocket::State<Mode>;
