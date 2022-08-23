use std::fmt::Display;

use crate::errors::Result;

#[derive(Debug, Clone)]
pub enum TokenKind {
    // Complex non ident tokens.
    Integer(usize),
    Comment(String),
    String(String),

    // Symbol Tokens.
    Ampersand,
    Asterisk,
    Colon,
    Comma,
    LeftBracket,
    ArrayType,
    RightBracket,
    LeftCurly,
    DefaultObject,
    RightCurly,
    LeftParen,
    RightParen,

    // Complex Tokens
    Identifier(String),
    BigIntType,
    NewBigInt,
    Interface,
    False,
    True,
    Nil,
    Map,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Integer(_) => todo!(),
            TokenKind::Comment(comment) => write!(f, "// {comment}"),
            TokenKind::String(string) => write!(f, "\"{string}\""),

            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::ArrayType => write!(f, "[]"),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::LeftCurly => write!(f, "{{"),
            TokenKind::DefaultObject => write!(f, "{{}}"),
            TokenKind::RightCurly => write!(f, "}}"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),

            TokenKind::Identifier(ident) => write!(f, "{ident}"),
            TokenKind::BigIntType => write!(f, "todo"),
            TokenKind::NewBigInt => write!(f, "Big::from"),
            TokenKind::Interface => write!(f, "serde_json::Value"),
            TokenKind::False => write!(f, "false"),
            TokenKind::True => write!(f, "true"),
            TokenKind::Nil => write!(f, "None"),
            TokenKind::Map => write!(f, "IndexMap"),
        }
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

    pub(crate) fn next_token(&mut self) -> Result<Option<(TokenKind, usize, usize)>> {
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
            .fold(0, |sum, c| sum + c.len_utf8());
        self.chomp(skipped);
    }

    fn _next_token(&mut self) -> Result<TokenKind> {
        let (tok, bytes_read) = TokenKind::tokenize_single(self.remaining_text)?;
        self.chomp(bytes_read);

        Ok(tok)
    }

    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_text = &self.remaining_text[num_bytes..];
        self.current_index += num_bytes;
    }
}
