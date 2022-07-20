use crate::asserter::{
    asserter_tools::RequestAsserter,
    errors::{AssertResult, AsserterError},
};

use super::server_test::request_asserter;

pub(crate) trait AsserterTester {
    type P;
    type E;
    fn name(&self) -> &'static str;
    fn payload(&self) -> &Self::P;
    fn error(&self) -> Option<&AsserterError>;
    fn print(&self) {
        println!("test: {}", self.name())
    }
    fn extras(&self) -> &Self::E {
        unimplemented!()
    }
    fn assert_correct<T>(&self, res: &Result<T, AsserterError>) {
        match (res, self.error()) {
            (Ok(_), Some(exp)) => {
                panic!("test passed when it shouldnt have. expected error: `{exp}`")
            }
            (Err(err), Some(exp)) if !err.to_string().contains(&exp.to_string()) => {
                panic!("expected text fragment `{exp}` not found in error: `{err}`")
            }
            (Err(err), None) => panic!("test failed when it shouldnt have. returned error: `{err}`"),
            _ => {}
        }
    }
}

macro_rules! tester_impl {
    () => {
        fn name(&self) -> &'static str {
            self.name
        }

        fn payload(&self) -> &Self::P {
            &self.payload
        }

        fn error(&self) -> Option<&AsserterError> {
            self.err.as_ref()
        }
    };
}

#[derive(Default)]
pub(crate) struct AsserterTest<P: Default> {
    pub name: &'static str,
    pub payload: P,
    pub err: Option<AsserterError>,
}

impl<P: Default> AsserterTester for AsserterTest<P> {
    type P = P;
    type E = ();

    tester_impl!();
}

#[derive(Default)]
pub(crate) struct CustomAsserterTest<P: Default, E: Default> {
    pub name: &'static str,
    pub payload: P,
    pub extras: E,
    pub err: Option<AsserterError>,
}

impl<P: Default, E: Default> AsserterTester for CustomAsserterTest<P, E> {
    type P = P;
    type E = E;

    tester_impl!();

    fn extras(&self) -> &Self::E {
        &self.extras
    }
}

pub(crate) fn non_asserter_tests<T, F, O>(tests: &[T], mut func: F)
where
    T: AsserterTester,
    F: FnMut(&T::P) -> AssertResult<O>,
{
    for test in tests {
        test.print();
        let res = func(test.payload());
        test.assert_correct(&res)
    }
}

pub(crate) fn default_request_asserter_tests<T, F>(tests: &[T], mut func: F)
where
    T: AsserterTester,
    F: FnMut(&RequestAsserter, &T::P) -> AssertResult<()>,
{
    let server = request_asserter();

    for test in tests {
        test.print();
        let res = func(&server, test.payload());
        test.assert_correct(&res);
    }
}

pub(crate) fn custom_request_asserter_tests<A, T, F>(asserter: A, tests: &[T], mut func: F)
where
    A: Fn(&T::E) -> RequestAsserter,
    T: AsserterTester,
    F: FnMut(&RequestAsserter, &T::P) -> AssertResult<()>,
{
    for test in tests {
        test.print();
        let server = asserter(test.extras());
        let res = func(&server, test.payload());
        test.assert_correct(&res);
    }
}
