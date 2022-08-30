use indexmap::IndexMap;

use super::{parser_context::ParserContext, tokens::TokenKind, Token};
use crate::{
    errors::{ParserError, Result},
    parse_rules::{RulesFile, TestStructPayload, TestStructPayloadField},
};

const INDENT: &'static str = "  ";

impl ParserContext {
    fn parse_type(&mut self, indent: usize, type_: String, optionify: bool) -> Result<()> {
        print!("{}", INDENT.repeat(indent));
        if self.check(&TokenKind::DefaultObject) {
            self.eat(&TokenKind::DefaultObject);
            self.eat(&TokenKind::Comma);
            println!("Default::default(),");
            return Ok(());
        }

        self.expect(&TokenKind::LeftCurly)?;
        if optionify {
            println!("Some({type_} {{");
        } else {
            println!("{type_} {{");
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

    fn parse_dynamic_payload(
        &mut self,
        struct_format: &str,
        fields: &IndexMap<String, TestStructPayloadField>,
    ) -> Result<()> {
        println!("{struct_format}");
        let ident = self.expect_identifier()?;
        self.expect(&TokenKind::Colon)?;

        print!("{}{ident}: ", INDENT.repeat(5));

        let (vecs_to_close, optionify, type_) = self.parse_type_context()?;
        self.parse_type(6, type_, optionify)?;
        print!("{}", INDENT.repeat(5));
        for i in 0..vecs_to_close {
            self.expect(&TokenKind::RightCurly)?;
            print!("]");
        }
        self.expect(&TokenKind::Comma)?;
        println!(",");

        Ok(())
    }

    fn parse_test_struct(&mut self, rules: &RulesFile) -> Result<()> {
        println!("{}{} {{", INDENT.repeat(2), rules.test_struct.struct_name);
        if let TokenKind::String(test_name) = self.curr_token.kind.clone() {
            println!("{}name: \"{test_name}\",", INDENT.repeat(3));
        } else {
            ParserError::unexpected_token(&self.curr_token.kind, "String", self.curr_token.span)?;
        }

        self.bump();

        self.expect(&TokenKind::Colon)?;

        match &rules.test_struct.payload {
            TestStructPayload::Dynamic {
                struct_name,
                fields,
            } => {
                let struct_format = format!(
                    "{}{} {{",
                    INDENT.repeat(4),
                    struct_name.to_ascii_lowercase()
                );
                self.parse_curly_comma_list(|c| {
                    ParserContext::parse_dynamic_payload(c, &struct_format, fields)
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
