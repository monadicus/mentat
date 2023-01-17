use core::fmt::Display;

use super::lexer::Lexer;
use crate::errors::Result;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Integer {
//     UInt(usize),
//     Int(isize),
// }

// impl Display for Integer {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Integer::UInt(uint) => write!(f, "{uint}"),
//             Integer::Int(int) => write!(f, "{int}"),
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum TokenKind {
//     Eof,
//     // Complex non ident tokens.
//     Decimal(f64),
//     Integer(Integer),
//     Comment(String),
//     String(String),

//     // Symbol Tokens.
//     Ampersand,
//     Asterisk,
//     Colon,
//     Comma,
//     Dot,
//     LeftBracket,
//     ArrayType,
//     RightBracket,
//     LeftCurly,
//     DefaultObject,
//     RightCurly,
//     LeftParen,
//     RightParen,

//     // Complex Tokens
//     Identifier(String),
//     Interface,
//     TypesDot,
//     IntType,
//     NewInt,
//     False,
//     True,
//     Nil,
//     Map,
//     Big,
// }

// impl Display for TokenKind {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TokenKind::Eof => write!(f, "<eof>"),
//             TokenKind::Decimal(decimal) => write!(f, "{decimal}"),
//             TokenKind::Integer(int) => write!(f, "{int}"),
//             TokenKind::Comment(comment) => write!(f, "// {comment}"),
//             TokenKind::String(string) => write!(f, "\"{string}\""),

//             TokenKind::Ampersand => write!(f, "&"),
//             TokenKind::Asterisk => write!(f, "Option<"),
//             TokenKind::Colon => write!(f, ":"),
//             TokenKind::Comma => write!(f, ","),
//             TokenKind::Dot => write!(f, "."),
//             TokenKind::LeftBracket => write!(f, "["),
//             TokenKind::ArrayType => write!(f, "Vec<"),
//             TokenKind::RightBracket => write!(f, "]"),
//             TokenKind::LeftCurly => write!(f, "{{"),
//             TokenKind::DefaultObject => write!(f, "{{}}"),
//             TokenKind::RightCurly => write!(f, "}}"),
//             TokenKind::LeftParen => write!(f, "("),
//             TokenKind::RightParen => write!(f, ")"),

//             TokenKind::Identifier(ident) => write!(f, "{ident}"),
//             TokenKind::Interface => write!(f, "serde_json::Value"),
//             TokenKind::TypesDot => write!(f, ""),
//             TokenKind::IntType => write!(f, "todo"),
//             TokenKind::NewInt => write!(f, "todo"),
//             TokenKind::False => write!(f, "false"),
//             TokenKind::True => write!(f, "true"),
//             TokenKind::Nil => write!(f, "None"),
//             TokenKind::Map => write!(f, "IndexMap"),
//             TokenKind::Big => write!(f, "todo"),
//         }
//     }
// }

// impl From<String> for TokenKind {
//     fn from(other: String) -> TokenKind {
//         TokenKind::Identifier(other)
//     }
// }

// impl<'a> From<&'a str> for TokenKind {
//     fn from(other: &'a str) -> TokenKind {
//         TokenKind::Identifier(other.to_string())
//     }
// }

// impl From<usize> for TokenKind {
//     fn from(other: usize) -> TokenKind {
//         TokenKind::Integer(Integer::UInt(other))
//     }
// }

// impl From<isize> for TokenKind {
//     fn from(other: isize) -> TokenKind {
//         TokenKind::Integer(Integer::Int(other))
//     }
// }

// impl From<f64> for TokenKind {
//     fn from(other: f64) -> TokenKind {
//         TokenKind::Decimal(other)
//     }
// }

/// A trait to represent a Token type.
pub trait TokenKind: Clone + Display + PartialEq + Eq + Sized {
    /// Returns a dummy `TokenKind` for a `Span`.
    fn dummy() -> Self;
    /// Returns true if self is a EOF `TokenKind`.
    fn is_eof(&self) -> bool;
    /// Returns a EOF `TokenKind`.
    fn eof() -> Self;
    type IdentifierType;
    /// Returns the `Self::Identifier` if self is a Identifier `TokenKind`.
    fn is_identifier(&self) -> Option<Self::IdentifierType>;
    /// Checks if another `TokenKind` is equal to self.
    fn check(&self, tok: &Self) -> bool {
        self == tok
    }
}

pub(crate) struct Tokenizer<'a> {
    current_index: usize,
    remaining_text: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub(crate) fn new(src: &str) -> Tokenizer {
        Tokenizer {
            current_index: 0,
            remaining_text: src,
        }
    }

    pub(crate) fn next_token<L: Lexer>(&mut self) -> Result<Option<(L, usize, usize)>> {
        self.skip_whitespace();

        if self.remaining_text.is_empty() {
            Ok(None)
        } else {
            let start = self.current_index;
            let tok = self._next_token().unwrap_or_else(|_| {
                todo!("Error Couldn't read the next token {}", self.current_index)
            });
            let end = self.current_index;
            Ok(Some((tok, start, end)))
        }
    }

    fn skip_whitespace(&mut self) {
        let skipped = self
            .remaining_text
            .chars()
            .take_while(|c| c.is_ascii_whitespace())
            .fold(0, |sum, c| {
                if matches!(c, '\t') {
                    // the tab lang we use
                    sum + 2
                } else {
                    sum + c.len_utf8()
                }
            });
        self.chomp(skipped);
    }

    fn _next_token<L: Lexer>(&mut self) -> Result<L> {
        let (tok, bytes_read) = L::tokenize_single(self.remaining_text)?;
        self.chomp(bytes_read);

        Ok(tok)
    }

    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_text = &self.remaining_text[num_bytes..];
        self.current_index += num_bytes;
    }
}
