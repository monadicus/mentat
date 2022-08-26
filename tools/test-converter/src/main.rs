use std::path::PathBuf;

use clap::Parser;
use parse_rules::RulesFile;
use test_parser::{parse, set_source_map_if_not_set};

mod converter;
mod errors;
mod parse_rules;
mod test_parser;
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

fn handle_error<T>(res: Result<T>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn main() -> Result<()> {
    let options = Options::parse();
    let input_ext = options.rules.extension().and_then(|ext| ext.to_str());
    if matches!(input_ext, Some("toml")) {
        let rules = dbg!(RulesFile::from_toml_file(options.rules)?);
        set_source_map_if_not_set(|_| handle_error(parse(&options.tests)));
        Ok(())
    } else {
        RulesFileError::unknown_input_file_extension(input_ext.unwrap_or_default())
    }
}
