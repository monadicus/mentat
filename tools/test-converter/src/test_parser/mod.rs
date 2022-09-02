use std::path::Path;

use crate::{errors::Result, parse_rules::RulesFile};

mod converter;
use indexmap::IndexMap;
use mentat_tokenizer::*;

pub(crate) fn parse(
    src: &Path,
    struct_max_fields_str: IndexMap<String, usize>,
    rules: RulesFile,
) -> Result<()> {
    let sf = with_source_map(|s| s.load_file(src))?;
    let tokens = tokenize(&sf.src)?;
    let mut context = ParserContext::new(tokens);
    context.convert(rules)
}
