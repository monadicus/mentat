use std::fmt;

use super::*;

pub(crate) struct AsserterRequestDefaultTest<P> {
    pub name: &'static str,
    pub payload: Option<P>,
    pub err: Option<AsserterError>,
}

impl<P, Input, O> Test<Input> for AsserterRequestDefaultTest<P>
where
    Input: FnMut(&Asserter, Option<&P>) -> AssertResult<O>,
{
    fn run(tests: &[Self], mut func: Input) {
        let asserter = request_asserter();

        let failed = tests
            .iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(&asserter, test.payload.as_ref());
                check_err_match(&test.err, &res)
            })
            .filter(|t| !t)
            .count();

        status_message(failed, tests.len());
    }
}

impl<P> fmt::Display for AsserterRequestDefaultTest<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test `{}`", self.name)
    }
}

pub(crate) struct CustomAsserterTest<P, E> {
    pub name: &'static str,
    pub payload: Option<P>,
    pub extras: E,
    pub err: Option<AsserterError>,
}

impl<P, E> CustomAsserterTest<P, E> {
    pub(crate) fn custom_asserter_tests<A, F>(tests: &[Self], asserter: A, mut func: F)
    where
        A: Fn(&E) -> Asserter,
        F: FnMut(&Asserter, Option<&P>) -> AssertResult<()>,
    {
        let failed = tests
            .iter()
            .map(|test| {
                print!("{test}: ");
                let asserter = asserter(&test.extras);
                let res = func(&asserter, test.payload.as_ref());
                check_err_match(&test.err, &res)
            })
            .filter(|t| !t)
            .count();

        status_message(failed, tests.len());
    }
}

impl<P, E> fmt::Display for CustomAsserterTest<P, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test `{}`", self.name)
    }
}
