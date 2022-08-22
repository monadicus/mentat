use std::path::PathBuf;

use clap::Parser;
use parse_rules::RulesFile;

mod converter;
mod errors;
mod parse_rules;
use crate::errors::{Result, RulesFileError};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Options {
    #[clap(short, long, parse(from_os_str))]
    rules: PathBuf,
    #[clap(short, long, parse(from_os_str))]
    tests: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let input_ext = options.rules.extension().and_then(|ext| ext.to_str());
    if matches!(input_ext, Some("toml")) {
        dbg!(RulesFile::from_toml_file(options.rules)?);
        Ok(())
    } else {
        RulesFileError::unknown_input_file_extension(input_ext.unwrap_or_default())
    }
}
