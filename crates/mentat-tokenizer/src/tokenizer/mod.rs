use crate::errors::Result;

mod lexer;
mod parser_context;
pub use parser_context::*;
mod source_map;
pub use source_map::*;
mod span;
pub use span::*;
mod tokens;
pub use tokens::*;

#[derive(Debug, Clone)]
pub struct Token {
    span: Span,
    kind: TokenKind,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, start: usize, stop: usize) -> Self {
        Self {
            span: Span::new(start.into(), stop.into()),
            kind,
        }
    }

    #[inline]
    pub(crate) const fn dummy() -> Self {
        Self {
            kind: TokenKind::Dot,
            span: Span::dummy(),
        }
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
    let mut tokenizer = Tokenizer::new(src);
    let mut tokens = Vec::new();

    while let Some((kind, start, stop)) = tokenizer.next_token()? {
        tokens.push(Token::new(kind, start, stop));
    }

    Ok(tokens)
}
