use std::{
    error::Error,
    fmt::{self},
};

pub trait Test<Input>: core::fmt::Display + Sized {
    fn run(tests: &[Self], input: Input);
}

pub struct ErrorTest {
    pub name: &'static str,
    pub err: Box<dyn std::error::Error>,
    pub is: bool,
}

impl ErrorTest {
    pub fn run<Input>(tests: Vec<Self>, mut func: Input)
    where
        Input: FnMut(Box<dyn std::error::Error>) -> (bool, &'static str),
    {
        let len = tests.len();
        let failed = tests
            .into_iter()
            .map(|test| {
                print!("{test}: ");
                let (is, name) = func(test.err);
                if test.is != is && test.name != name {
                    println!(
                        "test returned wrong value: `{:?}` != `{:?}`",
                        test.name, name
                    );
                    false
                } else {
                    true
                }
            })
            .filter(|t| !t)
            .count();

        status_message(failed, len);
    }
}

impl fmt::Display for ErrorTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test `{}`", self.name)
    }
}

pub struct FnTest<Payload, Res> {
    pub name: &'static str,
    pub payload: Payload,
    pub result: Res,
}

impl<Payload, Res> FnTest<Payload, Res> {
    pub fn run_is_err<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: PartialEq<bool>,
        F: Fn(Payload) -> Result<Ok, Err>,
        Ok: fmt::Debug,
        Err: Error,
    {
        let len = tests.len();
        let failed = tests
            .into_iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(test.payload);
                check_is_err(test.result, &res)
            })
            .filter(|t| !t)
            .count();

        status_message(failed, len);
    }

    pub fn run_err_match<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<Option<Err>>,
        F: Fn(Payload) -> Result<Ok, Err>,
        Ok: fmt::Debug,
        Err: fmt::Display,
    {
        let len = tests.len();
        let failed = tests
            .into_iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(test.payload);
                check_err_match(&test.result.into(), &res)
            })
            .filter(|t| !t)
            .count();

        status_message(failed, len);
    }

    pub fn run_result_match<F, Ok, Err>(tests: Vec<Self>, func: F)
    where
        Res: Into<Result<Ok, Err>>,
        F: Fn(Payload) -> Res,
        Ok: fmt::Debug + PartialEq,
        Err: fmt::Debug + PartialEq,
    {
        let len = tests.len();
        let failed = tests
            .into_iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(test.payload);
                check_results_match(&test.result.into(), &res.into())
            })
            .filter(|t| !t)
            .count();

        status_message(failed, len);
    }

    pub fn run_output_match<F>(tests: Vec<Self>, func: F)
    where
        Res: PartialEq + fmt::Debug,
        F: Fn(Payload) -> Res,
    {
        let len = tests.len();
        let failed = tests
            .into_iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(test.payload);
                check_output_match(&test.result, &res)
            })
            .filter(|t| !t)
            .count();

        status_message(failed, len);
    }
}

impl<Payload, Res> fmt::Display for FnTest<Payload, Res> {
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

pub fn check_is_err<B, T, E>(err: B, res: &Result<T, E>) -> bool
where
    B: PartialEq<bool>,
    T: fmt::Debug,
    E: Error,
{
    if err == res.is_err() {
        println!("ok!");
        true
    } else if err == true {
        println!(
            "test passed when it shouldn't have. returned error `{}`",
            res.as_ref().unwrap_err()
        );
        false
    } else {
        println!(
            "test failed when it shouldnt have. returned result `{:?}`",
            res.as_ref().unwrap()
        );
        false
    }
}

pub fn check_output_match<T>(expected: &T, res: &T) -> bool
where
    T: fmt::Debug + PartialEq,
{
    if expected == res {
        println!("ok!");
        true
    } else {
        println!("test returned wrong value: `{expected:?}` != `{res:?}`");
        false
    }
}

pub fn check_results_match<T, E>(expected: &Result<T, E>, res: &Result<T, E>) -> bool
where
    T: fmt::Debug + PartialEq,
    E: fmt::Debug + PartialEq,
{
    match (expected, res) {
        (Err(expected), Ok(res)) => {
            println!(
                "test passed when it shouldn't have. returned value: `{res:?}`, but expected err: `{expected:?}`"
            );
            false
        }
        (Ok(expected), Err(res)) => {
            println!(
                "test failed when it shouldn't have. returned error: `{res:?}`, but expected valued `{expected:?}`"
            );
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
        (Ok(_), Some(exp)) => {
            println!("test passed when it shouldn't have. expected error: `{exp}`");
            false
        }
        _ => {
            println!("ok!");
            true
        }
    }
}
