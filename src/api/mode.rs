use std::env;

use super::Response;

pub enum Mode {
    Online,
    Offline,
    Neither,
}

impl Mode {
    pub fn handle<R>(
        &self,
        on: impl Fn() -> Response<R>,
        off: impl Fn() -> Response<R>,
        neither: impl Fn() -> Response<R>,
    ) -> Response<R> {
        match self {
            Mode::Online => on(),
            Mode::Offline => off(),
            Mode::Neither => neither(),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        match env::var("MODE").as_deref() {
            Ok("ONLINE") => Mode::Online,
            Ok("OFFLINE") => Mode::Offline,
            _ => Mode::Neither,
        }
    }
}

pub type ModeState = rocket::State<Mode>;
