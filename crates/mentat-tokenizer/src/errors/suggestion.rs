use colored::Colorize;

#[derive(Debug, Default)]
pub struct Suggestion(String);
impl From<&'static str> for Suggestion {
    fn from(s: &'static str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Suggestion {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl std::fmt::Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            "Suggestion".underline().purple(),
            self.0.purple()
        )
    }
}
