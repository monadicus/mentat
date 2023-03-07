use super::{span::Span, tokens::TokenKind, Token};
use crate::errors::{ContextError, Result};

// #[derive(Debug, Clone, Copy)]
// pub enum Delimiter {
//     Brace,
//     Bracket,
//     Parenthesis,
// }

// impl Delimiter {
//     pub fn pair(self) -> (TokenKind, TokenKind) {
//         match self {
//             Self::Brace => (TokenKind::LeftCurly, TokenKind::RightCurly),
//             Self::Bracket => (TokenKind::LeftBracket,
// TokenKind::RightBracket),             Self::Parenthesis =>
// (TokenKind::LeftParen, TokenKind::RightParen),         }
//     }
// }

pub struct ParserContext<T> {
    tokens: Vec<Token<T>>,
    pub curr_token: Token<T>,
    pub prev_token: Token<T>,
}

impl<T: TokenKind> ParserContext<T> {
    pub fn new(mut tokens: Vec<Token<T>>) -> Self {
        tokens.reverse();

        let mut context = Self {
            tokens,
            curr_token: Token::dummy(),
            prev_token: Token::dummy(),
        };
        context.bump();

        context
    }

    pub fn bump(&mut self) {
        if self.prev_token.kind.is_eof() {
            panic!("attempted to bump the parser past EOF (may be stuck in a loop)");
        }

        // Extract next token, or `Eof` if there was none.
        let next_token = self.tokens.pop().unwrap_or(Token {
            kind: T::eof(),
            span: self.curr_token.span,
        });

        // Set the new token.
        self.prev_token = std::mem::replace(&mut self.curr_token, next_token);
    }

    pub fn check(&self, tok: &T) -> bool {
        self.curr_token.kind.check(tok)
    }

    #[track_caller]
    pub fn eat(&mut self, token: &T) -> bool {
        self.check(token).then(|| self.bump()).is_some()
    }

    pub fn look_ahead<R>(&self, dist: usize, looker: impl FnOnce(&Token<T>) -> R) -> R {
        if dist == 0 {
            return looker(&self.curr_token);
        }

        let idx = match self.tokens.len().checked_sub(dist) {
            None => return looker(&Token::dummy()),
            Some(idx) => idx,
        };

        looker(self.tokens.get(idx).unwrap_or(&Token::dummy()))
    }

    #[track_caller]
    pub fn eat_identifier(&mut self) -> Option<T::IdentifierType> {
        if let Some(ident) = self.curr_token.kind.is_identifier() {
            self.bump();
            return Some(ident);
        }
        None
    }

    #[track_caller]
    pub fn expect_identifier(&mut self) -> Result<T::IdentifierType> {
        if let Some(ident) = self.eat_identifier() {
            Ok(ident)
        } else {
            ContextError::unexpected_token(
                &self.curr_token.kind,
                "identifier",
                self.curr_token.span,
            )
        }
    }

    #[track_caller]
    pub fn eat_any(&mut self, tokens: &[T]) -> bool {
        tokens
            .iter()
            .any(|x| self.check(x))
            .then(|| self.bump())
            .is_some()
    }

    #[track_caller]
    fn unexpected<R>(&self, expected: impl std::fmt::Display) -> Result<R> {
        ContextError::unexpected_token(&self.curr_token.kind, expected, self.curr_token.span)
    }

    #[track_caller]
    pub fn expect(&mut self, token: &T) -> Result<Span> {
        if self.eat(token) {
            Ok(self.prev_token.span)
        } else {
            self.unexpected(token)
        }
    }

    #[track_caller]
    pub fn expect_any(&mut self, tokens: &[T]) -> Result<Span> {
        if self.eat_any(tokens) {
            Ok(self.prev_token.span)
        } else {
            self.unexpected(
                tokens
                    .iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }
    }
}