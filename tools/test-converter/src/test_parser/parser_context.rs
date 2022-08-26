// TODO

use super::{span::Span, tokens::TokenKind, Token};
use crate::errors::{ParserError, Result};

#[derive(Debug, Clone, Copy)]
pub(super) enum Delimiter {
    Brace,
    Bracket,
    Parenthesis,
}

impl Delimiter {
    fn pair(self) -> (TokenKind, TokenKind) {
        match self {
            Self::Brace => (TokenKind::LeftCurly, TokenKind::RightCurly),
            Self::Bracket => (TokenKind::LeftBracket, TokenKind::RightBracket),
            Self::Parenthesis => (TokenKind::LeftParen, TokenKind::RightParen),
        }
    }
}

pub(super) struct ParserContext {
    tokens: Vec<Token>,
    pub(super) curr_token: Token,
    pub(super) prev_token: Token,
}

impl ParserContext {
    pub(super) fn new(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();

        let mut context = Self {
            tokens,
            curr_token: Token::dummy(),
            prev_token: Token::dummy(),
        };
        context.bump();

        context
    }

    pub(super) fn bump(&mut self) {
        if let TokenKind::Eof = self.prev_token.kind {
            panic!("attempted to bump the parser past EOF (may be stuck in a loop)");
        }

        // Extract next token, or `Eof` if there was none.
        let next_token = self.tokens.pop().unwrap_or(Token {
            kind: TokenKind::Eof,
            span: self.curr_token.span,
        });

        // Set the new token.
        self.prev_token = std::mem::replace(&mut self.curr_token, next_token);
    }

    pub(super) fn check(&self, tok: &TokenKind) -> bool {
        &self.curr_token.kind == tok
    }

    pub(super) fn eat(&mut self, token: &TokenKind) -> bool {
        self.check(token).then(|| self.bump()).is_some()
    }

    pub(super) fn look_ahead<R>(&self, dist: usize, looker: impl FnOnce(&Token) -> R) -> R {
        if dist == 0 {
            return looker(&self.curr_token);
        }

        let idx = match self.tokens.len().checked_sub(dist) {
            None => return looker(&Token::dummy()),
            Some(idx) => idx,
        };

        looker(self.tokens.get(idx).unwrap_or(&Token::dummy()))
    }

    // pub(super) fn eat_identifier(&mut self) -> Option<Identifier> {
    //     if let TokenKind::Identifier(name) = self.token.token {
    //         self.bump();
    //         return Some(self.mk_ident_prev(name));
    //     }
    //     None
    // }

    // pub(super) fn expect_identifier(&mut self) -> Result<Identifier> {
    //     self.eat_identifier().ok_or_else(|| {
    //         ParserError::unexpected_str(&self.token.token, "identifier",
    // self.token.span).into()     })
    // }

    pub(super) fn eat_any(&mut self, tokens: &[TokenKind]) -> bool {
        tokens
            .iter()
            .any(|x| self.check(x))
            .then(|| self.bump())
            .is_some()
    }

    fn unexpected<T>(&self, expected: impl std::fmt::Display) -> Result<T> {
        ParserError::unexpected_token(&self.curr_token.kind, expected, self.curr_token.span)
    }

    pub(super) fn expect(&mut self, token: &TokenKind) -> Result<Span> {
        if self.eat(token) {
            Ok(self.prev_token.span)
        } else {
            self.unexpected(token)
        }
    }

    pub(super) fn expect_any(&mut self, tokens: &[TokenKind]) -> Result<Span> {
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

    pub(super) fn parse_list<T>(
        &mut self,
        delimiter: Delimiter,
        sep: Option<TokenKind>,
        mut inner: impl FnMut(&mut Self) -> Result<Option<T>>,
    ) -> Result<(Vec<T>, bool, Span)> {
        let (open, close) = delimiter.pair();
        let mut list = Vec::new();
        let mut trailing = false;
        let open_span = self.expect(&open)?;

        while !self.check(&close) {
            if let Some(elem) = inner(self)? {
                list.push(elem);
            }

            if sep.as_ref().filter(|sep| !self.eat(sep)).is_some() {
                trailing = false;
                break;
            }

            trailing = true;
        }

        let span = open_span + self.expect(&close)?;
        Ok((list, trailing, span))
    }

    pub(super) fn parse_paren_comma_list<T>(
        &mut self,
        f: impl FnMut(&mut Self) -> Result<Option<T>>,
    ) -> Result<(Vec<T>, bool, Span)> {
        self.parse_list(Delimiter::Parenthesis, Some(TokenKind::Comma), f)
    }
}
