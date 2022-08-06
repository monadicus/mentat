use std::{
    error::Error,
    fmt::{self},
};

pub trait Test<Input>: core::fmt::Display + Sized {
    fn run(tests: &[Self], input: Input);
}

pub struct EqualityTest<P, R> {
    pub name: &'static str,
    pub payload: P,
    pub res: R,
}

impl<P, Input, R> Test<Input> for EqualityTest<P, R>
where
    Input: FnMut(&P) -> R,
    R: Eq + fmt::Debug,
{
    fn run(tests: &[Self], mut func: Input) {
        let failed = tests
            .iter()
            .map(|test| {
                print!("{test}: ");
                let res = func(&test.payload);
                if test.res != res {
                    println!("test returned wrong value: `{:?}` != `{:?}`", test.res, res);
                    false
                } else {
                    println!("ok!");
                    true
                }
            })
            .filter(|t| !t)
            .count();

        status_message(failed, tests.len());
    }
}

impl<P, R> fmt::Display for EqualityTest<P, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test `{}`", self.name)
    }
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

pub fn status_message(failed: usize, total: usize) {
    if failed == 0 {
        println!("all passed!")
    } else {
        panic!("failed {}/{} tests", failed, total)
    }
}

pub fn assert_results_correct<T, E>(expected: &Result<T, E>, res: &Result<T, E>) -> bool
where
    T: fmt::Display + Eq,
    E: fmt::Display + Eq,
{
    match (expected, res) {
        (Err(expected), Ok(res)) => {
            println!(
                "test passed when it shouldn't have. returned value: `{res}`, but expected err: `{expected}`"
            );
            false
        }
        (Ok(expected), Err(res)) => {
            println!(
                "test failed when it shouldn't have. returned error: `{res}`, but expected valued `{expected}`"
            );
            false
        }
        _ => {
            println!("ok!");
            true
        }
    }
}

pub fn check_test_result<T, E>(err: &Option<E>, res: &Result<T, E>) -> bool
where
    E: Error,
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
