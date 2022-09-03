use mentat_tokenizer::ParserError;

use super::*;
use crate::parse_rules::{RulesFile, TestStructPayload};

const INDENT: &str = "  ";

// TODO SPANS
impl Parser {
    #[track_caller]
    pub fn parse_list(
        &mut self,
        delimiter: Delimiter,
        sep: Option<TokenKind>,
        mut inner: impl FnMut(&mut Self) -> Result<()>,
    ) -> Result<()> {
        let (open, close) = delimiter.pair();
        let open_span = self.context.expect(&open)?;

        while !self.context.check(&close) {
            inner(self)?;

            if sep.as_ref().filter(|sep| !self.context.eat(sep)).is_some() {
                break;
            }
        }

        let span = open_span + self.context.expect(&close)?;
        Ok(())
    }

    #[track_caller]
    pub fn parse_curly_comma_list(&mut self, f: impl FnMut(&mut Self) -> Result<()>) -> Result<()> {
        self.parse_list(Delimiter::Brace, Some(TokenKind::Comma), f)
    }

    fn parse_type(&mut self, indent: usize, type_: String, optionify: bool) -> Result<()> {
        if self.context.check(&TokenKind::DefaultObject) {
            self.context.eat(&TokenKind::DefaultObject);
            self.context.eat(&TokenKind::Comma);
            println!("Default::default(),");
            return Ok(());
        }

        if optionify {
            println!("Some({type_} {{");
        } else {
            println!("{type_} {{");
        }

        match self.context.curr_token.kind.clone() {
            TokenKind::LeftCurly => {
                self.context.bump();
                self.parse_field(indent + 1)?;
            }
            TokenKind::Identifier(field) => {
                self.context.bump();
                self.context.expect(&TokenKind::Colon)?;
                print!(
                    "{}{}: {}",
                    INDENT.repeat(indent),
                    field.to_ascii_lowercase(),
                    self.context.curr_token.kind
                );
                self.context.bump();
            }
            _ => todo!("error"),
        }

        self.context.eat(&TokenKind::Comma);
        Ok(())
    }

    fn parse_type_context(&mut self) -> Result<(usize, bool, String)> {
        let mut close_vec_count = 0;
        let mut optionify = false;
        let mut is_type = false;
        let mut type_ = String::new();
        while !self.context.check(&TokenKind::LeftCurly) {
            match &self.context.curr_token.kind {
                TokenKind::ArrayType => {
                    println!("vec![");
                    close_vec_count += 1
                }
                TokenKind::Asterisk => optionify = true,
                TokenKind::TypesDot => is_type = true,
                TokenKind::Identifier(type_name) if is_type => type_ = type_name.to_string(),
                _ => {}
            }

            self.context.bump();
        }

        self.context.expect(&TokenKind::LeftCurly)?;
        Ok((close_vec_count, optionify, type_))
    }

    fn parse_field(&mut self, indent: usize) -> Result<()> {
        let ident = self.context.expect_identifier()?;
        self.context.expect(&TokenKind::Colon)?;

        print!("{}{}: ", INDENT.repeat(indent), ident.to_ascii_lowercase());

        let (vecs_to_close, optionify, type_) = self.parse_type_context()?;
        self.parse_type(indent + 1, type_, optionify)?;
        print!("{}", INDENT.repeat(indent));
        for i in 0..(vecs_to_close) {
            self.context.expect(&TokenKind::RightCurly)?;
            print!("]");
        }

        Ok(())
    }

    fn parse_dynamic_payload(&mut self, struct_format: &str) -> Result<()> {
        println!("{struct_format}");
        let ident = self.context.expect_identifier()?;
        self.context.expect(&TokenKind::Colon)?;

        print!("{}{ident}: ", INDENT.repeat(4));

        let (vecs_to_close, optionify, type_) = self.parse_type_context()?;
        print!("{}", INDENT.repeat(5));
        // self.context.parse_type(5, type_, optionify)?;
        // print!("{}", INDENT.repeat(4));
        // for i in 0..vecs_to_close {
        //     self.context.expect(&TokenKind::RightCurly)?;
        //     print!("]");
        // }
        // self.context.expect(&TokenKind::Comma)?;
        // println!(",");

        Ok(())
    }

    fn parse_test_struct(&mut self) -> Result<()> {
        println!(
            "{}{} {{",
            INDENT.repeat(2),
            self.rules.test_struct.struct_name
        );
        if matches!(self.context.curr_token.kind, TokenKind::String(_)) {
            println!(
                "{}name: {},",
                INDENT.repeat(3),
                self.context.curr_token.kind
            );
        } else {
            ParserError::unexpected_token(
                &self.context.curr_token.kind,
                "String",
                self.context.curr_token.span,
            )?;
        }

        self.context.bump();

        self.context.expect(&TokenKind::Colon)?;

        print!("{}payload: ", INDENT.repeat(3));
        match &self.rules.test_struct.payload {
            TestStructPayload::Dynamic {
                struct_name,
                fields,
            } => {
                let struct_format = format!("{} {{", struct_name);
                self.parse_curly_comma_list(|c| Parser::parse_dynamic_payload(c, &struct_format))?;
                Ok(())
            }
            TestStructPayload::Single { struct_name, value } => todo!(),
        }
    }

    pub(super) fn convert(&mut self) -> Result<()> {
        println!("#[test]");
        println!("fn {}() {{", self.rules.test_struct.test_fn_name);
        println!("{INDENT}let tests = vec![");
        self.parse_curly_comma_list(|c| Parser::parse_test_struct(c))?;
        println!("{INDENT}];");
        println!(
            "{INDENT}{}::{}(",
            self.rules.test_struct.struct_name, self.rules.test_struct.test_fn_name
        );
        println!("{}tests,", INDENT.repeat(2));
        for line in self.rules.test_struct.closure.lines() {
            println!("{:2}{line}", INDENT.repeat(2));
        }
        println!("{INDENT});");
        println!("}}");
        Ok(())
    }
}
