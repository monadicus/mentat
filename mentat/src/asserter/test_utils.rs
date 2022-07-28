use std::fmt;

use super::server_test::request_asserter;
use crate::{
    asserter::{
        asserter_tools::{Asserter, RequestAsserter},
        errors::{AssertResult, AsserterError},
    },
    tests::Test,
};

pub(crate) struct AsserterTest<P> {
    pub name: &'static str,
    pub payload: Option<P>,
    pub err: Option<AsserterError>,
}

impl<P, Input, O> Test<Input> for AsserterTest<P>
where
    Input: FnMut(Option<&P>) -> AssertResult<O>,
{
    fn run(tests: &[Self], mut func: Input) {
        let failed = tests
            .iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(test.payload.as_ref());
                assert_correct(&test.err, &res)
            })
            .filter(|t| !t)
            .count();

        status_message(failed, tests.len());
    }
}

impl<P> fmt::Display for AsserterTest<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test `{}`", self.name)
    }
}

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
                assert_correct(&test.err, &res)
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

pub struct AsserterEqualityTest<P, R> {
    pub name: &'static str,
    pub payload: P,
    pub res: R,
}

impl<P, Input, R> Test<Input> for AsserterEqualityTest<P, R>
where
    Input: FnMut(&P) -> R,
    R: Eq + fmt::Display,
{
    fn run(tests: &[Self], mut func: Input) {
        let failed = tests
            .iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(&test.payload);
                if test.res != res {
                    println!("test returned wrong value: `{}` != `{}`", test.res, res);
                    false
                } else {
                    true
                }
            })
            .filter(|t| !t)
            .count();

        status_message(failed, tests.len());
    }
}

impl<P, R> fmt::Display for AsserterEqualityTest<P, R> {
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
                assert_correct(&test.err, &res)
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

pub(crate) fn assert_correct<T>(
    err: &Option<AsserterError>,
    res: &Result<T, AsserterError>,
) -> bool {
    match (res, err) {
        (Err(err), Some(exp)) if !err.to_string().contains(&exp.to_string()) => {
            println!("expected text fragment `{exp}` not found in error: `{err}`");
            false
        }
        (Err(err), None) => {
            println!("test failed when it shouldnt have. returned error: `{err}`");
            false
        }
        (Ok(_), Some(exp)) => {
            println!("test passed when it shouldnt have. expected error: `{exp}`");
            false
        }
        _ => {
            println!("ok!");
            true
        }
    }
}

pub(crate) fn status_message(failed: usize, total: usize) {
    if failed == 0 {
        println!("all passed!")
    } else {
        panic!("failed {}/{} tests", failed, total)
    }
}
