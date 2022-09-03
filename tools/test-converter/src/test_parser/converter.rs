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

    fn parse_object(&mut self, type_: String, optionify: bool) -> Result<()> {
        if self.context.check(&TokenKind::DefaultObject) {
            self.context.eat(&TokenKind::DefaultObject);
            self.context.eat(&TokenKind::Comma);
            emitln!(self, "Default::default(),");
            return Ok(());
        }

        if optionify {
            emitln!(self, "Some({type_} {{");
        } else {
            emitln!(self, "{type_} {{");
        }

        self.context.expect(&TokenKind::LeftCurly)?;
        self.inc_indent();
        let ident = self.context.expect_identifier()?;
        self.context.expect(&TokenKind::Colon)?;
        emit!(self, "{ident}: ");
        dbg!("foo bar?");
        self.parse_object_or_simple()?;

        // match self.context.curr_token.kind.clone() {
        //     TokenKind::LeftCurly => {
        //         self.context.bump();
        //         self.parse_field(indent + 1)?;
        //     }
        //     TokenKind::Identifier(field) => {
        //         self.context.bump();
        //         self.context.expect(&TokenKind::Colon)?;
        //         print!(
        //             "{}{}: {}",
        //             INDENT.repeat(indent),
        //             field.to_ascii_lowercase(),
        //             self.context.curr_token.kind
        //         );
        //         self.context.bump();
        //     }
        //     _ => todo!("error"),
        // }

        // self.context.eat(&TokenKind::Comma);
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

    // fn parse_field(&mut self, indent: usize) -> Result<()> {
    //     let ident = self.context.expect_identifier()?;
    //     self.context.expect(&TokenKind::Colon)?;

    //     print!("{}{}: ", INDENT.repeat(indent), ident.to_ascii_lowercase());

    //     let (vecs_to_close, optionify, type_) = self.parse_type_context()?;
    //     self.inc_indent();
    //     self.parse_object(type_, optionify)?;
    //     print!("{}", INDENT.repeat(indent));
    //     for i in 0..(vecs_to_close) {
    //         self.context.expect(&TokenKind::RightCurly)?;
    //         print!("]");
    //     }

    //     Ok(())
    // }

    // TODO need a way to emit with no ident
    fn parse_object_or_simple(&mut self) -> Result<()> {
        // Object type
        // &types.TypeIdent or &TypeIdent
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
            dbg!("look ahead");
            let (vecs_to_close, optionify, type_) = self.parse_type_context()?;
            self.inc_indent();

            // self.emit("{}", false);
            self.parse_object(type_, optionify)?;
            // print!("{}", INDENT.repeat(4));
            for i in 0..vecs_to_close {
                self.context.expect(&TokenKind::RightCurly)?;
                emit!(self, "]");
            }
        } else {
            dbg!("direct");
            // direct type
            println!("{},", self.context.curr_token.kind);
            self.context.bump();
        }
        Ok(())
    }

    fn parse_dynamic_payload(
        &mut self,
        struct_format: &str,
        payload_fields: &IndexMap<String, TestStructPayloadField>,
    ) -> Result<()> {
        // TODO emit with no indent
        println!("{struct_format}");
        self.inc_indent();
        let ident = self.context.expect_identifier()?;
        self.context.expect(&TokenKind::Colon)?;
        emit!(self, "{ident}: ");

        // This is a dynamic Payload field
        if payload_fields.contains_key(&ident) {
            self.parse_object_or_simple()?;

        // self.context.expect(&TokenKind::Comma)?;
        // println!(",");
        } else {
            // We need to store this and emit it at the right time.
        }

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

        emit!(self, "payload: ");
        match self.rules.test_struct.payload.clone() {
            TestStructPayload::Dynamic {
                struct_name,
                fields,
            } => {
                let struct_format = format!("{} {{", struct_name);
                self.parse_curly_comma_list(|c| {
                    Parser::parse_dynamic_payload(c, &struct_format, &fields)
                })?;
            }
            TestStructPayload::Single { struct_name, value } => todo!(),
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
