use std::path::Path;

use crate::{errors::Result, parse_rules::RulesFile};

mod converter;
use indexmap::IndexMap;
use mentat_tokenizer::*;

pub(crate) struct Parser {
    pub context: ParserContext,
    pub rules: RulesFile,
    pub struct_max_fields_str: IndexMap<String, usize>,
    pub indent: usize,
}

impl Parser {
    const INDENT: &'static str = "  ";

    fn new(
        context: ParserContext,
        rules: RulesFile,
        struct_max_fields_str: IndexMap<String, usize>,
    ) -> Self {
        Self {
            context,
            rules,
            struct_max_fields_str,
            indent: 0,
        }
    }

    fn dec_indent(&mut self) {
        self.indent -= 1;
    }

    fn inc_indent(&mut self) {
        self.indent += 1;
    }

    fn emit(&self, out: impl core::fmt::Display, lnbreak: bool) {
        print!("{}{out}", Self::INDENT.repeat(self.indent));
        if lnbreak {
            println!();
        }
    }
}

pub(crate) fn parse(
    src: &Path,
    struct_max_fields_str: IndexMap<String, usize>,
    rules: RulesFile,
) -> Result<()> {
    let sf = with_source_map(|s| s.load_file(src))?;
    let tokens = tokenize(&sf.src)?;
    let context = ParserContext::new(tokens);
    Parser::new(context, rules, struct_max_fields_str).convert()
}
