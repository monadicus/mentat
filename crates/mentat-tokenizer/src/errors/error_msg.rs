use colored::Colorize;

#[derive(Debug, Default)]
pub struct ErrorMsg(String);

impl From<&'static str> for ErrorMsg {
    fn from(s: &'static str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for ErrorMsg {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl std::fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", "Error".underline().red(), self.0.red())
    }
}
