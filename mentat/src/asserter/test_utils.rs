use std::fmt;

use crate::asserter::{
    asserter_tools::RequestAsserter,
    errors::{AssertResult, AsserterError},
};

use super::server_test::request_asserter;

#[derive(Default)]
pub(crate) struct AsserterTest<P: Default> {
    pub name: &'static str,
    pub payload: P,
    pub err: Option<AsserterError>,
}

impl<P: Default> AsserterTest<P> {
    pub(crate) fn non_asserter_tests<F, O>(tests: &[Self], mut func: F)
    where
        F: FnMut(&P) -> AssertResult<O>,
    {
        for test in tests {
            println!("{test}");
            let res = func(&test.payload);
            assert_correct(&test.err, &res);
        }
    }

    pub(crate) fn default_request_asserter_tests<F, O>(tests: &[Self], mut func: F)
    where
        F: FnMut(&RequestAsserter, &P) -> AssertResult<O>,
    {
        let server = request_asserter();

        for test in tests {
            println!("{test}");
            let res = func(&server, &test.payload);
            assert_correct(&test.err, &res);
        }
    }
}

impl<P: Default> fmt::Display for AsserterTest<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test: {}", self.name)
    }
}

#[derive(Default)]
pub(crate) struct AsserterEqualityTest<P: Default, R: Default> {
    pub name: &'static str,
    pub payload: P,
    pub res: R,
}

impl<P: Default, R: Default> AsserterEqualityTest<P, R> {
    pub(crate) fn non_asserter_equality_tests<F>(tests: &[Self], mut func: F)
    where
        R: Eq + fmt::Debug,
        F: FnMut(&P) -> R,
    {
        for test in tests {
            println!("{test}");
            let res = func(&test.payload);
            assert_eq!(test.res, res)
        }
    }
}

impl<P: Default, R: Default> fmt::Display for AsserterEqualityTest<P, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test: {}", self.name)
    }
}

#[derive(Default)]
pub(crate) struct CustomAsserterTest<P: Default, E: Default> {
    pub name: &'static str,
    pub payload: P,
    pub extras: E,
    pub err: Option<AsserterError>,
}

impl<P: Default, E: Default> CustomAsserterTest<P, E> {
    pub(crate) fn custom_request_asserter_tests<A, F>(tests: &[Self], asserter: A, mut func: F)
    where
        A: Fn(&E) -> RequestAsserter,
        F: FnMut(&RequestAsserter, &P) -> AssertResult<()>,
    {
        for test in tests {
            println!("{test}");
            let server = asserter(&test.extras);
            let res = func(&server, &test.payload);
            assert_correct(&test.err, &res);
        }
    }
}

impl<P: Default, E: Default> fmt::Display for CustomAsserterTest<P, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test: {}", self.name)
    }
}

fn assert_correct<T>(err: &Option<AsserterError>, res: &Result<T, AsserterError>) {
    match (res, err) {
        (Err(err), Some(exp)) if !err.to_string().contains(&exp.to_string()) => {
            panic!("expected text fragment `{exp}` not found in error: `{err}`")
        }
        (Err(err), None) => {
            panic!("test failed when it shouldnt have. returned error: `{err}`")
        }
        (Ok(_), Some(exp)) => {
            panic!("test passed when it shouldnt have. expected error: `{exp}`")
        }
        _ => {}
    }
}
