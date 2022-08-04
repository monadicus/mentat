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
    pub(crate) fn run<A, P, F, ExemptOperation>(tests: Vec<Self>, asserter: A, parser: P, func: F)
    where
        A: Fn(AE) -> Asserter,
        P: Fn(Asserter, PE, Vec<Option<BalanceExemption>>) -> Parser<ExemptOperation>,
        ExemptOperation: Fn(&Operation) -> bool,
        F: Fn(&Parser<ExemptOperation>, &Payload) -> bool,
    {
        let len = tests.len();
        let failed = tests
            .into_iter()
            .map(|test| {
                print!("{test}: ");
                let asserter = asserter(test.asserter_extras);
                let parser = parser(asserter, test.parser_extras, Vec::new());
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
