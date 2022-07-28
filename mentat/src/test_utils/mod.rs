pub trait Test<Input>: core::fmt::Display + Sized {
    fn run(tests: &[Self], input: Input);
}
