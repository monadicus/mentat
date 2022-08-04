use std::fmt;

use super::*;

pub(crate) struct CustomParserTest<Payload, AE, PE> {
    pub name: &'static str,
    pub payload: Payload,
    pub asserter_extras: AE,
    pub parser_extras: PE,
    pub err: Option<ParserError>,
}

impl<Payload, AE, PE> CustomParserTest<Payload, AE, PE> {
    pub(crate) fn run<A, P, F, ExemptOperation>(tests: &[Self], asserter: A, parser: P, mut func: F)
    where
        A: Fn(AE) -> Asserter,
        P: Fn(Asserter, PE, Vec<Option<BalanceExemption>>) -> Parser<ExemptOperation>,
        F: FnMut(Parser<ExemptOperation>, &Payload) -> bool,
        ExemptOperation: Fn(&Operation) -> bool,
    {
        let failed = tests
            .iter()
            .map(|test| {
                print!("{test}: ");
                let asserter = asserter(test.asserter_extras);
                let parser: Parser<ExemptOperation> =
                    parser(asserter, test.parser_extras, Vec::new());
                func(parser, &test.payload)
            })
            .filter(|t| !t)
            .count();

        status_message(failed, tests.len());
    }
}

impl<Payload, AE, PE> fmt::Display for CustomParserTest<Payload, AE, PE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test `{}`", self.name)
    }
}
