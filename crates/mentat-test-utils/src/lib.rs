use std::{
    error::Error,
    fmt::{self},
};

use futures::{future::join_all, Future};

/// helper struct used to hold custom instances during method tests
pub struct MethodPayload<C, P> {
    /// the instance making the method call
    pub caller: C,
    /// the payload being passed to the method call
    pub payload: P,
}

/// a test case in a rosetta test
pub struct TestCase<Payload, Res> {
    /// the name of the test
    pub name: &'static str,
    /// the data being passed into the test function
    pub payload: Payload,
    /// the criteria that the test's result should satisfy.
    /// this field requires different behavior depending which run method is
    /// used
    pub criteria: Res,
}

impl<Payload, Res> TestCase<Payload, Res> {
    /// runs the tests using the given async function and criteria matcher
    async fn async_runner<FnOut, F, Matcher, In1, In2, Fut>(
        tests: Vec<Self>,
        func: F,
        matcher: Matcher,
    ) where
        Res: Into<In1>,
        FnOut: Into<In2>,
        F: Fn(Payload) -> Fut,
        Fut: Future<Output = FnOut>,
        Matcher: Fn(&In1, &In2) -> bool,
    {
        let len = tests.len();
        let failed = join_all(tests.into_iter().map(|test| async {
            print!("{test}: ");
            let res = func(test.payload).await;
            matcher(&test.criteria.into(), &res.into())
        }))
        .await
        .into_iter()
        .filter(|t| !t)
        .count();

        status_message(failed, len);
    }

    /// runs the tests using the given function and criteria matcher
    fn runner<FnOut, F, Matcher, In1, In2>(tests: Vec<Self>, func: F, matcher: Matcher)
    where
        Res: Into<In1>,
        FnOut: Into<In2>,
        F: Fn(Payload) -> FnOut,
        Matcher: Fn(&In1, &In2) -> bool,
    {
        let len = tests.len();
        let failed = tests
            .into_iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(test.payload);
                matcher(&test.criteria.into(), &res.into())
            })
            .filter(|t| !t)
            .count();

        status_message(failed, len);
    }

    /// Runs all tests with the given function and asserts the following:
    ///     if `self.criteria` is true then the function should return `Err`.
    ///     if `self.criteria` is false then the function should return `Ok`.
    /// Requires `self.criteria` to either be a `bool` or support `Into<bool>`.
    /// Will panic if any tests fail to match the expected criteria.
    pub fn run_is_err<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<bool>,
        F: Fn(Payload) -> Result<Ok, Err>,
        Ok: fmt::Debug,
        Err: Error,
    {
        Self::runner(tests, func, check_is_err::<Res, Ok, Err>);
    }

    /// Runs all tests with the given function and asserts the following:
    ///     if `self.criteria` is `Some` then the function should return `Err`
    /// with content that contains the text from the error in criteria.
    ///     if `self.criteria` is `None` then the function should return `Ok`.
    /// Requires `self.criteria` to be `Option<E>` where `E` matches the
    /// `Result::Err` type of the given test function. Will panic if any
    /// tests fail to match the expected criteria.
    pub fn run_err_match<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<Option<Err>>,
        F: Fn(Payload) -> Result<Ok, Err>,
        Ok: fmt::Debug,
        Err: fmt::Display,
    {
        Self::runner(tests, func, check_err_match::<Ok, Err>);
    }

    /// Runs all tests with the given function and asserts the following:
    ///     if `self.criteria` is `Some` then the function should return `Ok`
    /// with content that matches the criteria.     if `self.criteria` is
    /// `None` then the function should return `Err`. Requires `self.
    /// criteria` to be `Option<T>` where `T` matches the `Result::Ok` type of
    /// the given test function. Will panic if any tests fail to match the
    /// expected criteria.
    pub fn run_ok_match<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<Option<Ok>>,
        F: Fn(Payload) -> Result<Ok, Err>,
        Ok: fmt::Debug + PartialEq,
        Err: fmt::Display,
    {
        Self::runner(tests, func, check_ok_match::<Ok, Err>);
    }

    /// Runs all tests with the given function and asserts the output exactly
    /// matches `self.criteria`. Requires `self.criteria` to be a `Result`
    /// type with the same type's as the output of the given test function.
    /// Will panic if any tests fail to match the expected criteria.
    /// If you want to match something other that a `Result` or don't care about
    /// having a detailed error message then try [`TestCase::run_output_match`].
    pub fn run_result_match<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<Result<Ok, Err>>,
        F: Fn(Payload) -> Res,
        Ok: fmt::Debug + PartialEq,
        Err: fmt::Debug + PartialEq,
    {
        Self::runner(tests, func, check_results_match::<Ok, Err>);
    }

    pub fn run_ok_match_err_contains<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<Result<Ok, Err>>,
        F: Fn(Payload) -> Res,
        Ok: fmt::Debug + PartialEq,
        Err: fmt::Display,
    {
        Self::runner(tests, func, check_ok_match_err_contains::<Ok, Err>);
    }

    /// TODO
    pub fn run_async_output_match<F, Fut>(tests: Vec<Self>, func: F)
    where
        Res: PartialEq + fmt::Debug,
        F: Fn(Payload) -> Fut,
        Fut: Future<Output = Res>,
    {
        futures::executor::block_on(Self::async_runner(tests, func, check_output_match::<Res>))
    }

    /// Runs all tests with the given function and asserts the output exactly
    /// matches `self.criteria`. Requires `self.criteria` to be the same
    /// type as the output of the given test function. Will panic if any
    /// tests fail to match the expected criteria. If you're matching a
    /// `Result` and want a more detailed error message, then try
    /// [`TestCase::run_result_match`]
    pub fn run_output_match<F>(tests: Vec<Self>, func: F)
    where
        Res: PartialEq + fmt::Debug,
        F: Fn(Payload) -> Res,
    {
        Self::runner(tests, func, check_output_match::<Res>);
    }
}

impl<Payload, Res> fmt::Display for TestCase<Payload, Res> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test `{}`", self.name)
    }
}

/// asserts `failed` == `total` then prints the corresponding status message
pub fn status_message(failed: usize, total: usize) {
    if failed == 0 {
        println!("all passed!")
    } else {
        panic!("failed {}/{} tests", failed, total)
    }
}

/// checks if `res.is_err() == is_err` then prints the corresponding status
/// message
pub fn check_is_err<B, T, E>(is_err: &bool, res: &Result<T, E>) -> bool
where
    T: fmt::Debug,
    E: Error,
{
    match (is_err, res) {
        (false, Err(e)) => {
            println!("test failed when it shouldnt have. returned error `{e}`",);
            false
        }
        (true, Ok(v)) => {
            println!("test passed when it shouldn't have. returned result `{v:?}`",);
            false
        }
        _ => {
            println!("ok!");
            true
        }
    }
}

/// checks if `res == expected` then prints the corresponding status message
pub fn check_output_match<T>(expected: &T, res: &T) -> bool
where
    T: fmt::Debug + PartialEq,
{
    if expected != res {
        println!("test returned wrong value: `{expected:?}` != `{res:?}`");
        false
    } else {
        println!("ok!");
        true
    }
}

/// checks if `res == expected` then prints the corresponding status message
pub fn check_results_match<T, E>(expected: &Result<T, E>, res: &Result<T, E>) -> bool
where
    T: fmt::Debug + PartialEq,
    E: fmt::Debug + PartialEq,
{
    match (expected, res) {
        (Err(_), Ok(res)) => {
            println!("test passed when it shouldn't have. returned value: `{res:?}`");
            false
        }
        (Ok(_), Err(res)) => {
            println!("test failed when it shouldn't have. returned error: `{res:?}`");
            false
        }
        (Ok(exp), Ok(res)) if exp != res => {
            println!("test returned wrong value: `{expected:?}` != `{res:?}`");
            false
        }
        (Err(exp), Err(res)) if exp != res => {
            println!("test returned wrong value: `{expected:?}` != `{res:?}`");
            false
        }
        _ => {
            println!("ok!");
            true
        }
    }
}

/// checks if `res` is an error and that text from `err` or that `res.is_ok()`
/// and `err.is_none()` then prints the corresponding status message
pub fn check_err_match<T, E>(err: &Option<E>, res: &Result<T, E>) -> bool
where
    T: fmt::Debug,
    E: fmt::Display,
{
    match (res, err) {
        (Err(err), Some(exp)) if !err.to_string().contains(&exp.to_string()) => {
            println!("expected text fragment `{exp}` not found in error: `{err}`");
            false
        }
        (Err(err), None) => {
            println!("test failed when it shouldn't have. returned error: `{err}`");
            false
        }
        (Ok(r), Some(_)) => {
            println!("test passed when it shouldn't have. returned result: `{r:?}`");
            false
        }
        _ => {
            println!("ok!");
            true
        }
    }
}

/// checks if `res` is Ok and that the inner value matches `res` or that
/// `res.is_err()` and `res.is_none()` then prints the corresponding status
/// message
pub fn check_ok_match<T, E>(ok: &Option<T>, res: &Result<T, E>) -> bool
where
    T: fmt::Debug + PartialEq,
    E: fmt::Display,
{
    match (res, ok) {
        (Ok(r), Some(v)) if r != v => {
            println!("test returned wrong value: `{r:?}` != `{v:?}`");
            false
        }
        (Ok(r), None) => {
            println!("test passed when it shouldn't have. returned result: `{r:?}`");
            false
        }
        (Err(e), Some(_)) => {
            println!("test failed when it shouldn't have. returned error: `{e}`");
            false
        }
        _ => {
            println!("ok!");
            true
        }
    }
}

pub fn check_ok_match_err_contains<T, E>(crit: &Result<T, E>, res: &Result<T, E>) -> bool
where
    T: fmt::Debug + PartialEq,
    E: fmt::Display,
{
    match crit {
        Ok(v) => check_ok_match(&Some(v), &res.as_ref()),
        Err(e) => check_err_match(&Some(e), &res.as_ref()),
    }
}
