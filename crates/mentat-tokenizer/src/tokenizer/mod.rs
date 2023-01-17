use crate::errors::Result;

mod lexer;
pub use lexer::Lexer;
mod parser_context;
pub use parser_context::*;
mod source_map;
pub use source_map::*;
mod span;
pub use span::*;
mod tokens;
pub use tokens::*;

#[derive(Debug, Clone)]
pub struct Token<T> {
    pub span: Span,
    pub kind: T,
}

impl<T: TokenKind> Token<T> {
    pub(crate) fn new(kind: T, start: usize, stop: usize) -> Self {
        Self {
            span: Span::new(start.into(), stop.into()),
            kind,
        }
    }

    #[inline]
    pub(crate) fn dummy() -> Self {
        Self {
            kind: T::dummy(),
            span: Span::dummy(),
        }
    }
}

pub fn tokenize<L: Lexer>(src: &str) -> Result<Vec<Token<L>>> {
    let mut tokenizer = Tokenizer::new(src);
    let mut tokens = Vec::new();

    while let Some((kind, start, stop)) = tokenizer.next_token::<L>()? {
        tokens.push(Token::new(kind, start, stop));
    }

    Ok(tokens)
}
