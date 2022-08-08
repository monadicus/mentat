use std::{error::Error, fmt};

use super::*;

pub(crate) struct CustomParserTest<Payload, AE, PE> {
    pub name: &'static str,
    pub payload: Payload,
    pub asserter_extras: AE,
    pub parser_extras: PE,
    pub err: Option<ParserError>,
}

impl<Payload, AE, PE> CustomParserTest<Payload, AE, PE> {
    pub(crate) fn run<A, P, F>(tests: Vec<Self>, asserter: A, parser: P, func: F)
    where
        A: Fn(AE) -> Option<Asserter>,
        P: Fn(Option<Asserter>, PE) -> Parser,
        F: Fn(&Parser, &Payload) -> bool,
    {
        let len = tests.len();
        let failed = tests
            .into_iter()
            .map(|test| {
                print!("{test}: ");
                let asserter = asserter(test.asserter_extras);
                let parser = parser(asserter, test.parser_extras);
                func(&parser, &test.payload)
            })
            .filter(|t| !t)
            .count();

        status_message(failed, len);
    }
}

impl<Payload, AE, PE> fmt::Display for CustomParserTest<Payload, AE, PE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test `{}`", self.name)
    }
}
