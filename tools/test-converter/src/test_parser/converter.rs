use core::panic;

use mentat_tokenizer::ContextError;

use super::*;
use crate::parse_rules::{TestStructPayload, TestStructPayloadField};

const INDENT: &str = "  ";

macro_rules! emit {
    ($parser:ident, $($arg:tt)*) => {
        $parser.emit(format_args!($($arg)*), false)
    };
}

macro_rules! emitln {
    ($parser:ident, $($arg:tt)*) => {
        $parser.emit(format_args!($($arg)*), true)
    };
}

// TODO SPANS
impl Parser {
    #[track_caller]
    fn parse_list(
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
    fn parse_curly_comma_list(&mut self, f: impl FnMut(&mut Self) -> Result<()>) -> Result<()> {
        self.parse_list(Delimiter::Brace, Some(TokenKind::Comma), f)
    }

    fn parse_object(&mut self, type_: &str, optionify: bool) -> Result<()> {
        if self.context.check(&TokenKind::DefaultObject) {
            self.context.eat(&TokenKind::DefaultObject);
            self.context.eat(&TokenKind::Comma);
            if optionify {
                emitln!(self, "Some(Default::default()),");
            } else {
                emitln!(self, "Default::default(),");
            }
            return Ok(());
        }

        match (optionify, type_) {
            (true, "int") => {
                emitln!(self, "Some(");
            }
            (false, "int") => {}
            (true, _) => emitln!(self, "Some({type_} {{"),
            (false, _) => println!("{type_} {{"),
        }
        self.inc_indent();

        self.context.expect(&TokenKind::LeftCurly)?;
        // println!("{}", self.context.curr_token.span);

        match type_ {
            "int" => {
                loop {
                    // dbg!(&self.context.curr_token.kind);
                    print!("{}, ", self.context.curr_token.kind);
                    self.context.bump();
                    if self.context.check(&TokenKind::Comma) {
                        self.context.expect(&TokenKind::Comma)?;
                    } else {
                        break;
                    }
                }
            }
            t => {
                let mut field_count = 0;
                while matches!(self.context.curr_token.kind, TokenKind::Identifier(_)) {
                    let ident = self.context.expect_identifier()?;
                    self.context.expect(&TokenKind::Colon)?;
                    // dbg!(&ident);
                    emit!(self, "{ident}: ");
                    self.parse_object_or_simple()?;
                    field_count += 1;
                    // self.dec_indent();
                }

                if field_count != *self.struct_max_fields_str.get(type_).unwrap() {
                    emitln!(self, "  ...Default::default(),");
                }
            }
        }

        self.context.expect(&TokenKind::RightCurly)?;
        self.context.eat(&TokenKind::Comma);

        if optionify {
            emitln!(self, "}}");
            self.dec_indent();
            emitln!(self, "),");
        } else {
            emitln!(self, "}},");
        }

        Ok(())
    }

    fn parse_type_context(&mut self) -> Result<(usize, bool, String)> {
        let mut close_vec_count = 0;
        let mut optionify = false;
        let mut type_ = String::new();
        while !self.context.check(&TokenKind::LeftCurly) {
            match &self.context.curr_token.kind {
                TokenKind::ArrayType => {
                    emitln!(self, "vec![");
                    close_vec_count += 1
                }
                TokenKind::Asterisk => optionify = true,
                TokenKind::Identifier(type_name) => type_ = type_name.to_string(),
                t => {
                    // dbg!(t);
                }
            }

            self.context.bump();
        }

        Ok((close_vec_count, optionify, type_))
    }

    // TODO need a way to emit with no ident
    fn parse_object_or_simple(&mut self) -> Result<()> {
        // Object type:
        // &TypeIdent
        // &types.TypeIdent
        // []*types.TypeIdent || []*OperationDescription
        // dbg!(self
        // .context
        // .look_ahead(3, |t| (dbg!(&t.kind) == &TokenKind::LeftCurly)));
        if self
            .context
            .look_ahead(2, |t| (t.kind == TokenKind::LeftCurly))
            || self
                .context
                .look_ahead(3, |t| (t.kind == TokenKind::LeftCurly))
            || self
                .context
                .look_ahead(4, |t| (t.kind == TokenKind::LeftCurly))
        {
            let (vecs_to_close, optionify, type_) = self.parse_type_context()?;
            // dbg!(&type_);
            self.inc_indent();

            if vecs_to_close > 0 {
                self.context.expect(&TokenKind::LeftCurly)?;
                while !self.context.check(&TokenKind::RightCurly) {
                    self.parse_object(&type_, optionify)?;
                    if matches!(self.context.curr_token.kind, TokenKind::Comment(_)) {
                        self.context.bump();
                    }
                }

                for i in 0..vecs_to_close {
                    if i > 0 {
                        self.context.expect(&TokenKind::RightCurly)?;
                    }
                    // self.dec_indent();
                    emitln!(self, "],");
                }
                self.context.eat(&TokenKind::Comma);
            } else {
                self.parse_object(&type_, optionify)?;
            }
        } else {
            // direct type
            emitln!(self, "{},", self.context.curr_token.kind);
            self.context.bump();
            self.context.expect(&TokenKind::Comma)?;
        }
        self.dec_indent();
        Ok(())
    }

    fn parse_dynamic_payload(&mut self) -> Result<()> {
        self.parse_object_or_simple()?;

        emitln!(self, "}},");
        Ok(())
    }

    fn parse_test_struct(&mut self) -> Result<()> {
        emitln!(self, "{} {{", self.rules.test_struct.struct_name);
        self.inc_indent();
        if matches!(self.context.curr_token.kind, TokenKind::String(_)) {
            emitln!(self, "name: {},", self.context.curr_token.kind);
        } else {
            ContextError::unexpected_token(
                &self.context.curr_token.kind,
                "String",
                self.context.curr_token.span,
            )?;
        }

        self.context.bump();

        self.context.expect(&TokenKind::Colon)?;

        match self.rules.test_struct.payload.clone() {
            TestStructPayload::Dynamic {
                struct_name,
                fields,
            } => {
                emitln!(self, "payload:");
                self.inc_indent();
                self.context.expect(&TokenKind::LeftCurly)?;

                emitln!(self, "{} {{", struct_name);
                self.inc_indent();

                while matches!(self.context.curr_token.kind, TokenKind::Identifier(_)) {
                    let ident = &self.context.expect_identifier()?;
                    // dbg!(&ident);
                    self.context.expect(&TokenKind::Colon)?;

                    // This is a dynamic Payload field
                    match ident {
                        field if fields.contains_key(ident) => {
                            emitln!(self, "{ident}: ");
                            self.parse_dynamic_payload()?;
                        }
                        criteria if criteria == &self.rules.test_struct.criteria.from => {
                            emitln!(self, "criteria: ");
                            self.parse_dynamic_payload()?;
                        }
                        err if err == "err" => {
                            self.context.bump(); // value
                            self.context.bump(); // comma
                        }
                        _ => panic!("not sure!"),
                    }

                    self.context.expect(&TokenKind::RightCurly)?;
                    self.context.expect(&TokenKind::Comma)?;
                    self.dec_indent();
                    emitln!(self, "}},");
                }
            }
            TestStructPayload::Single { struct_name, value } => {
                todo!("Non custom payload objects not yet supported.")
            }
        }

        self.dec_indent();

        Ok(())
    }

    pub(super) fn convert(&mut self) -> Result<()> {
        emitln!(self, "#[test]");
        emitln!(self, "fn {}() {{", self.rules.test_struct.test_fn_name);
        self.inc_indent();
        emitln!(self, "let tests = vec![");
        self.parse_curly_comma_list(Parser::parse_test_struct)?;
        emitln!(self, "];");
        emitln!(
            self,
            "{}::{}(",
            self.rules.test_struct.struct_name,
            self.rules.test_struct.test_fn_name
        );
        self.inc_indent();
        emitln!(self, "tests,");
        for line in self.rules.test_struct.closure.lines() {
            // exception for alignment
            println!("{:2}{line}", INDENT.repeat(2));
        }
        self.dec_indent();
        emitln!(self, ");");
        self.dec_indent();
        emitln!(self, "}}");
        Ok(())
    }
}
