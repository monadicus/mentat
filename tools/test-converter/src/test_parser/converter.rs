use super::*;
use crate::parse_rules::{RulesFile, TestStructPayload};

const INDENT: &str = "  ";

impl ParserContext {
    fn parse_type(&mut self, indent: usize, type_: String, optionify: bool) -> Result<()> {
        if self.check(&TokenKind::DefaultObject) {
            self.eat(&TokenKind::DefaultObject);
            self.eat(&TokenKind::Comma);
            println!("Default::default(),");
            return Ok(());
        }

        if optionify {
            println!("Some({type_} {{");
        } else {
            println!("{type_} {{");
        }

        match self.curr_token.kind.clone() {
            TokenKind::LeftCurly => {
                self.bump();
                self.parse_field(indent + 1)?;
            }
            TokenKind::Identifier(field) => {
                self.bump();
                self.expect(&TokenKind::Colon)?;
                print!(
                    "{}{}: {}",
                    INDENT.repeat(indent),
                    field.to_ascii_lowercase(),
                    self.curr_token.kind
                );
                self.bump();
            }
            _ => todo!("error"),
        }

        self.eat(&TokenKind::Comma);
        Ok(())
    }

    fn parse_type_context(&mut self) -> Result<(usize, bool, String)> {
        let mut close_vec_count = 0;
        let mut optionify = false;
        let mut is_type = false;
        let mut type_ = String::new();
        while !self.check(&TokenKind::LeftCurly) {
            match &self.curr_token.kind {
                TokenKind::ArrayType => {
                    println!("vec![");
                    close_vec_count += 1
                }
                TokenKind::Asterisk => optionify = true,
                TokenKind::TypesDot => is_type = true,
                TokenKind::Identifier(type_name) if is_type => type_ = type_name.to_string(),
                _ => {}
            }

            self.bump();
        }

        self.expect(&TokenKind::LeftCurly)?;
        Ok((close_vec_count, optionify, type_))
    }

    fn parse_field(&mut self, indent: usize) -> Result<()> {
        let ident = self.expect_identifier()?;
        self.expect(&TokenKind::Colon)?;

        print!("{}{}: ", INDENT.repeat(indent), ident.to_ascii_lowercase());

        let (vecs_to_close, optionify, type_) = self.parse_type_context()?;
        self.parse_type(indent + 1, type_, optionify)?;
        print!("{}", INDENT.repeat(indent));
        for i in 0..(vecs_to_close) {
            self.expect(&TokenKind::RightCurly)?;
            print!("]");
        }

        Ok(())
    }

    fn parse_dynamic_payload(&mut self, struct_format: &str) -> Result<()> {
        println!("{struct_format}");
        let ident = self.expect_identifier()?;
        self.expect(&TokenKind::Colon)?;

        print!("{}{ident}: ", INDENT.repeat(4));

        let (vecs_to_close, optionify, type_) = self.parse_type_context()?;
        print!("{}", INDENT.repeat(5));
        // self.parse_type(5, type_, optionify)?;
        // print!("{}", INDENT.repeat(4));
        // for i in 0..vecs_to_close {
        //     self.expect(&TokenKind::RightCurly)?;
        //     print!("]");
        // }
        // self.expect(&TokenKind::Comma)?;
        // println!(",");

        Ok(())
    }

    fn parse_test_struct(&mut self, rules: &RulesFile) -> Result<()> {
        println!("{}{} {{", INDENT.repeat(2), rules.test_struct.struct_name);
        if matches!(self.curr_token.kind, TokenKind::String(_)) {
            println!("{}name: {},", INDENT.repeat(3), self.curr_token.kind);
        } else {
            ParserError::unexpected_token(&self.curr_token.kind, "String", self.curr_token.span)?;
        }

        self.bump();

        self.expect(&TokenKind::Colon)?;

        print!("{}payload: ", INDENT.repeat(3));
        match &rules.test_struct.payload {
            TestStructPayload::Dynamic {
                struct_name,
                fields,
            } => {
                let struct_format = format!("{} {{", struct_name);
                self.parse_curly_comma_list(|c| {
                    ParserContext::parse_dynamic_payload(c, &struct_format)
                })?;
                Ok(())
            }
            TestStructPayload::Single { struct_name, value } => todo!(),
        }
    }

    pub(super) fn convert(&mut self, rules: RulesFile) -> Result<()> {
        println!("#[test]");
        println!("fn {}() {{", rules.test_struct.test_fn_name);
        println!("{INDENT}let tests = vec![");
        self.parse_curly_comma_list(|c| ParserContext::parse_test_struct(c, &rules))?;
        println!("{INDENT}];");
        println!(
            "{INDENT}{}::{}(",
            rules.test_struct.struct_name, rules.test_struct.test_fn_name
        );
        println!("{}tests,", INDENT.repeat(2));
        for line in rules.test_struct.closure.lines() {
            println!("{:2}{line}", INDENT.repeat(2));
        }
        println!("{INDENT});");
        println!("}}");
        Ok(())
    }
}
