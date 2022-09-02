use std::iter::Peekable;

use super::tokens::TokenKind;
use crate::errors::{LexerError, Result};

impl TokenKind {
    fn tokenize_number(
        input: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<(TokenKind, usize)> {
        let mut seen_dot = false;
        let mut bytes_read = 0;
        let mut first = true;
        let mut negated = false;

        let number: String = std::iter::from_fn(|| {
            input.next_if(|c| {
                if first && c == &'-' {
                    bytes_read += 1;
                    first = false;
                    negated = true;
                    true
                } else if c.is_ascii_digit() {
                    bytes_read += c.len_utf8();
                    first = false;
                    true
                } else if c == &'.' {
                    first = false;
                    if !seen_dot {
                        seen_dot = true;
                        bytes_read += c.len_utf8();
                        true
                    } else {
                        false
                    }
                } else {
                    first = false;
                    false
                }
            })
        })
        .collect();

        if seen_dot && negated {
            LexerError::illegal_negative_decimals(number)
        } else if seen_dot {
            let n: f64 = LexerError::could_not_lex_decimal_number(number.parse(), number)?;
            Ok((TokenKind::from(n), bytes_read))
        } else if negated {
            let n: isize = LexerError::could_not_lex_signed_number(number.parse(), number)?;
            Ok((TokenKind::from(n), bytes_read))
        } else {
            let n: usize = LexerError::could_not_lex_number(number.parse(), number)?;
            Ok((TokenKind::from(n), bytes_read))
        }
    }

    fn tokenize_ident(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(Self, usize)> {
        // identifiers can't start with a number
        match input.peek() {
            Some(ch) if ch.is_ascii_digit() => return LexerError::ident_started_with_a_number(),
            None => return LexerError::unexpected_eof(),
            _ => {}
        }

        let ident: String =
            std::iter::from_fn(|| input.next_if(|c| c.is_ascii_alphanumeric() || c == &'_'))
                .collect();

        if ident.is_empty() {
            return LexerError::unknown_token(
                input
                    .take_while(|c| !c.is_ascii_whitespace())
                    .collect::<String>(),
            );
        }

        let mut len = ident.len();
        Ok((
            match &*ident {
                "interface" => TokenKind::Interface,
                "types" if matches!(input.next(), Some('.')) => {
                    len += 1;
                    TokenKind::TypesDot
                }
                "NewInt" => TokenKind::NewInt,
                "Int" => TokenKind::IntType,
                "false" => TokenKind::False,
                "true" => TokenKind::True,
                "nil" => TokenKind::Nil,
                "map" => TokenKind::Map,
                "big" => TokenKind::Big,
                _ => TokenKind::Identifier(ident),
            },
            len,
        ))
    }

    #[inline]
    fn match_one(
        input: &mut Peekable<impl Iterator<Item = char>>,
        token: Self,
    ) -> Result<(Self, usize)> {
        input.next();
        Ok((token, 1))
    }

    #[inline]
    fn match_two(
        input: &mut Peekable<impl Iterator<Item = char>>,
        first_token: Self,
        second_char: char,
        second_token: Self,
    ) -> Result<(Self, usize)> {
        input.next();
        Ok(if input.next_if_eq(&second_char).is_some() {
            (second_token, 2)
        } else {
            (first_token, 1)
        })
    }

    pub(crate) fn tokenize_single(input: &str) -> Result<(Self, usize)> {
        if input.is_empty() {
            return LexerError::unexpected_eof();
        }

        let input_str = input;
        let mut input = input.chars().peekable();

        // checked above
        match *input.peek().unwrap() {
            c if c.is_ascii_digit() => Self::tokenize_number(&mut input),
            '-' => Self::tokenize_number(&mut input),
            '/' => {
                input.next();
                // Find the end of the comment line.
                if input.next_if_eq(&'/').is_some() {
                    let comment = match input_str.as_bytes().iter().position(|c| *c == b'\n') {
                        None => input_str,
                        Some(idx) => &input_str[..idx + 1],
                    };
                    Ok((TokenKind::Comment(comment.to_owned()), comment.len()))
                } else {
                    LexerError::expected_comment(input.next().unwrap_or_default())
                }
            }
            '"' => {
                let rest = &input_str[1..];
                let string = match rest.as_bytes().iter().position(|c| *c == b'"') {
                    None => {
                        return LexerError::unclosed_string(
                            input
                                .take_while(|c| !c.is_ascii_whitespace())
                                .collect::<String>(),
                        );
                    }
                    Some(idx) => rest[..idx].to_owned(),
                };

                let len = string.len() + 2;
                Ok((TokenKind::String(string), len))
            }
            '&' => Self::match_one(&mut input, TokenKind::Ampersand),
            '*' => Self::match_one(&mut input, TokenKind::Asterisk),
            '[' => Self::match_two(
                &mut input,
                TokenKind::LeftBracket,
                ']',
                TokenKind::ArrayType,
            ),
            ']' => Self::match_one(&mut input, TokenKind::RightBracket),
            ':' => Self::match_one(&mut input, TokenKind::Colon),
            ',' => Self::match_one(&mut input, TokenKind::Comma),
            '.' => Self::match_one(&mut input, TokenKind::Dot),
            '{' => Self::match_two(
                &mut input,
                TokenKind::LeftCurly,
                '}',
                TokenKind::DefaultObject,
            ),
            '}' => Self::match_one(&mut input, TokenKind::RightCurly),
            '(' => Self::match_one(&mut input, TokenKind::LeftParen),
            ')' => Self::match_one(&mut input, TokenKind::RightParen),
            _ => Self::tokenize_ident(&mut input),
        }
    }
}

macro_rules! lexer_test {
    (FAIL: $name:ident, $func:expr, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let func = $func;

            let got = func(&mut src.chars().peekable());
            assert!(got.is_err(), "{:?} should be an error", got);
        }
    };
    (@inner SINGLE $src:expr, $should_be:expr) => {
        let should_be = TokenKind::from($should_be);

        let (got, _bytes_read) = TokenKind::tokenize_single($src.into()).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", $src);
    };
    (@inner INT $src:expr, $should_be:expr) => {
        let should_be = TokenKind::from($should_be);

        let (got, _bytes_read) =
            TokenKind::tokenize_number(&mut $src.chars().peekable()).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", $src);
    };
    (@inner IDENT $src:expr, $should_be:expr) => {
        let should_be = TokenKind::from($should_be);

        let (got, _bytes_read) = TokenKind::tokenize_ident(&mut $src.chars().peekable()).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", $src);
    };
    ($kind:ident: $name:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            lexer_test!(@inner $kind src, $should_be);
        }
    };
}

lexer_test!(IDENT: tokenize_a_single_letter, "f" => "f");
lexer_test!(IDENT: tokenize_an_identifier, "Foo" => "Foo");
lexer_test!(IDENT: tokenize_ident_containing_an_underscore, "Foo_bar" => "Foo_bar");
lexer_test!(
    FAIL: tokenize_ident_cant_start_with_number,
    TokenKind::tokenize_ident,
    "7Foo_bar"
);
lexer_test!(
    FAIL: tokenize_ident_cant_start_with_dot,
    TokenKind::tokenize_ident,
    ".Foo_bar"
);

lexer_test!(INT: tokenize_a_negative_single_digit_integer, "-1" => -1isize);
lexer_test!(INT: tokenize_a_single_digit_integer, "1" => 1usize);
lexer_test!(INT: tokenize_a_longer_integer, "1234567890" => 1234567890usize);
lexer_test!(INT: tokenize_a_longer_negative_integer, "-1234567890" => -1234567890isize);
lexer_test!(INT: tokenize_basic_decimal, "12.3" => 12.3);
lexer_test!(INT: tokenize_string_with_multiple_decimal_points, "12.3.456" => 12.3);
lexer_test!(
    FAIL: cant_tokenize_a_string_as_a_decimal,
    TokenKind::tokenize_number,
    "asdfghj"
);
lexer_test!(
    FAIL: cant_tokenize_a_negative_decimal,
    TokenKind::tokenize_number,
    "-12.3"
);
lexer_test!(INT: tokenizing_decimal_stops_at_alpha, "123.4asdfghj" => 123.4);

lexer_test!(SINGLE: central_tokenizer_decimal, "123.4" => 123.4);
lexer_test!(SINGLE: central_tokenizer_integer, "1234" => 1234usize);
lexer_test!(SINGLE: central_tokenizer_comment, "// comment" => TokenKind::Comment("// comment".into()));
lexer_test!(SINGLE: central_tokenizer_ampersand, "&" => TokenKind::Ampersand);
lexer_test!(SINGLE: central_tokenizer_asterisk, "*" => TokenKind::Asterisk);
lexer_test!(SINGLE: central_tokenizer_colon, ":" => TokenKind::Colon);
lexer_test!(SINGLE: central_tokenizer_dot, "." => TokenKind::Dot);
lexer_test!(SINGLE: central_tokenizer_open_square, "[" => TokenKind::LeftBracket);
lexer_test!(SINGLE: central_tokenizer_open_close_square, "[]" => TokenKind::ArrayType);
lexer_test!(SINGLE: central_tokenizer_close_square, "]" => TokenKind::RightBracket);
lexer_test!(SINGLE: central_tokenizer_open_brace, "{" => TokenKind::LeftCurly);
lexer_test!(SINGLE: central_tokenizer_open_close_brace, "{}" => TokenKind::DefaultObject);
lexer_test!(SINGLE: central_tokenizer_close_brace, "}" => TokenKind::RightCurly);
lexer_test!(SINGLE: central_tokenizer_open_paren, "(" => TokenKind::LeftParen);
lexer_test!(SINGLE: central_tokenizer_close_paren, ")" => TokenKind::RightParen);
lexer_test!(SINGLE: central_tokenizer_ident, "Foo_bar" => "Foo_bar");
lexer_test!(SINGLE: central_tokenizer_int_type, "Int" => TokenKind::IntType);
lexer_test!(SINGLE: central_tokenizer_new_int, "NewInt" => TokenKind::NewInt);
lexer_test!(SINGLE: central_tokenizer_interface, "interface" => TokenKind::Interface);
lexer_test!(SINGLE: central_tokenizer_false, "false" => TokenKind::False);
lexer_test!(SINGLE: central_tokenizer_true, "true" => TokenKind::True);
lexer_test!(SINGLE: central_tokenizer_nil, "nil" => TokenKind::Nil);
lexer_test!(SINGLE: central_tokenizer_map, "map" => TokenKind::Map);
lexer_test!(SINGLE: central_tokenizer_big, "big" => TokenKind::Big);
