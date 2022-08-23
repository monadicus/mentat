use std::iter::Peekable;

use super::tokens::TokenKind;
use crate::errors::Result;

impl TokenKind {
    // fn tokenize_number(input: &mut Peekable<impl Iterator<Item = char>>) ->
    // Result<(TokenKind, usize)> {     let mut seen_dot = false;
    //     let mut negated = false;

    //     let (decimal, bytes_read) = take_while(data, |c| {
    //         if c.is_digit(10) {
    //             true
    //         } else if c == '.' {
    //             if !seen_dot {
    //                 seen_dot = true;
    //                 true
    //             } else {
    //                 false
    //             }
    //         } else {
    //             false
    //         }
    //     })?;

    //     if seen_dot {
    //         let n: f64 = decimal.parse()?;
    //         Ok((TokenKind::Decimal(n), bytes_read))
    //     } else {
    //         let n: usize = decimal.parse()?;
    //         Ok((TokenKind::Integer(n), bytes_read))
    //     }
    // }

    fn tokenize_ident(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(Self, usize)> {
        // identifiers can't start with a number
        match input.peek() {
            Some(ch) if ch.is_ascii_digit() => todo!("Identifiers can't start with a number"),
            None => todo!("eof"),
            _ => {}
        }

        let ident: String =
            std::iter::from_fn(|| input.next_if(|c| c.is_ascii_alphanumeric() || c == &'_'))
                .collect();
        let len = ident.len();
        Ok((
            match &*ident {
                "big.Int" => TokenKind::BigIntType,
                "big.NewInt" => TokenKind::NewBigInt,
                "interface" => TokenKind::Interface,
                "false" => TokenKind::False,
                "true" => TokenKind::True,
                "nil" => TokenKind::Nil,
                "map" => TokenKind::Map,
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
            todo!("No input to parse")
        }

        let input_str = input;
        let mut input = input.chars().peekable();

        match *input.peek().ok_or_else(|| todo!()).unwrap() {
            c if c.is_ascii_digit() => todo!("eat some ints"),
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
                    todo!("error expected a /")
                }
            }
            '"' => {
                let rest = &input_str[1..];
                let string = match rest.as_bytes().iter().position(|c| *c == b'"') {
                    None => todo!("no closing \" found"),
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
