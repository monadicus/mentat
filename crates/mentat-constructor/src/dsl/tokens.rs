use std::fmt;

use mentat_tokenizer::{Lexer, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Eof,
    Split2,
    OpenBracket,
    CloseBracket,
    OpenDoubleBracket,
    CloseDoubleBracket,
    EndScenarioContinue,
    Quote,
    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
    OpenParens,
    CloseParens,
    EndLine,
    FunctionEndLine,
    CommentMarker,
    PathSeparator,
    Identifier(String),
    Comment(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eof => write!(f, "<eof>"),
            Self::Split2 => write!(f, "2"),
            Self::OpenBracket => write!(f, "{{"),
            Self::CloseBracket => write!(f, "}}"),
            Self::OpenDoubleBracket => write!(f, "{{{{"),
            Self::CloseDoubleBracket => write!(f, "}}}}"),
            Self::EndScenarioContinue => write!(f, "}},"),
            Self::Quote => write!(f, "\""),
            Self::Equal => write!(f, "="),
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::OpenParens => write!(f, "("),
            Self::CloseParens => write!(f, ")"),
            Self::EndLine => write!(f, ";"),
            Self::FunctionEndLine => write!(f, ");"),
            Self::CommentMarker => write!(f, "//"),
            Self::PathSeparator => write!(f, "."),
            Self::Identifier(ident) => write!(f, "{ident}"),
            Self::Comment(comment) => write!(f, "// {comment}"),
        }
    }
}

impl TokenKind for Token {
    type IdentifierType = String;

    fn dummy() -> Self {
        Self::Eof
    }

    fn is_eof(&self) -> bool {
        self == &Self::Eof
    }

    fn eof() -> Self {
        Self::Eof
    }

    fn is_identifier(&self) -> Option<Self::IdentifierType> {
        if let Self::Identifier(ident) = self.clone() {
            Some(ident)
        } else {
            None
        }
    }
}

impl Lexer for Token {
    fn tokenize_single(_input: &str) -> mentat_tokenizer::Result<(Self, usize)>
    where
        Self: Sized,
    {
        todo!()
    }
}
