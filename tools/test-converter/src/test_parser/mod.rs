use self::{
    span::Span,
    tokens::{TokenKind, Tokenizer},
};
use crate::errors::Result;
mod lexer;
mod span;
mod tokens;

// #[derive(Clone, Copy)]
// enum Delimiter {
//     Brace,
//     Bracket,
//     Parenthesis,
// }

// impl Delimiter {
//     fn pair(self) -> (Token, Token) {
//         match self {
//             Self::Brace => (Token::LeftCurly, Token::RightCurly),
//             Self::Bracket => (Token::LeftBracket, Token::RightBracket),
//             Self::Parenthesis => (Token::LeftParen, Token::RightParen),
//         }
//     }
// }

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
}

pub(crate) fn tokenize(src: &str) -> Result<Vec<Token>> {
    let mut tokenizer = Tokenizer::new(src);
    let mut tokens = Vec::new();

    while let Some((kind, start, stop)) = tokenizer.next_token()? {
        tokens.push(Token::new(kind, start, stop));
    }

    Ok(tokens)
}
