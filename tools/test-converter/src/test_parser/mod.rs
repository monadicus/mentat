use std::path::Path;

use self::{
    parser_context::ParserContext,
    span::Span,
    tokens::{TokenKind, Tokenizer},
};
use crate::{errors::Result, parse_rules::RulesFile};

mod converter;
mod lexer;
mod parser_context;
mod source_map;
pub(crate) use source_map::*;
mod span;
mod tokens;

// TODO spanned tokens
#[derive(Debug, Clone)]
pub(crate) struct Token {
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

fn tokenize(src: &str) -> Result<Vec<Token>> {
    let mut tokenizer = Tokenizer::new(src);
    let mut tokens = Vec::new();

    while let Some((kind, start, stop)) = tokenizer.next_token()? {
        tokens.push(Token::new(kind, start, stop));
    }

    Ok(tokens)
}

pub(crate) fn parse(src: &Path, rules: RulesFile) -> Result<()> {
    let sf = with_source_map(|s| s.load_file(src))?;
    let tokens = tokenize(&sf.src)?;
    let mut context = ParserContext::new(tokens);
    context.convert(rules)
}
