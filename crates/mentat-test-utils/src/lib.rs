use std::fmt::{self};

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
