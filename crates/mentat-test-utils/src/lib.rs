use std::{
    error::Error,
    fmt::{self},
};

pub struct MethodPayload<C, P> {
    pub caller: C,
    pub payload: P,
}

pub struct TestCase<Payload, Res> {
    pub name: &'static str,
    pub payload: Payload,
    pub result: Res,
}

impl<Payload, Res> TestCase<Payload, Res> {
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
                matcher(&test.result.into(), &res.into())
            })
            .filter(|t| !t)
            .count();

        status_message(failed, len);
    }

    pub fn run_is_err<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<bool>,
        F: Fn(Payload) -> Result<Ok, Err>,
        Ok: fmt::Debug,
        Err: Error,
    {
        Self::runner(tests, func, check_is_err::<Res, Ok, Err>);
    }

    pub fn run_err_match<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<Option<Err>>,
        F: Fn(Payload) -> Result<Ok, Err>,
        Ok: fmt::Debug,
        Err: fmt::Display,
    {
        Self::runner(tests, func, check_err_match::<Ok, Err>);
    }

    pub fn run_ok_match<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<Option<Ok>>,
        F: Fn(Payload) -> Result<Ok, Err>,
        Ok: fmt::Debug + PartialEq,
        Err: fmt::Display,
    {
        Self::runner(tests, func, check_ok_match::<Ok, Err>);
    }

    pub fn run_result_match<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<Result<Ok, Err>>,
        F: Fn(Payload) -> Res,
        Ok: fmt::Debug + PartialEq,
        Err: fmt::Debug + PartialEq,
    {
        Self::runner(tests, func, check_results_match::<Ok, Err>);
    }

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

pub fn status_message(failed: usize, total: usize) {
    if failed == 0 {
        println!("all passed!")
    } else {
        panic!("failed {}/{} tests", failed, total)
    }
}

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
