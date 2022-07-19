use indexmap::IndexMap;

use crate::asserter::{
    asserter_tools::Asserter,
    errors::{AssertResult, AsserterError},
};

#[derive(Default)]
pub(crate) struct ServerTest<T: Default> {
    pub request: T,
    pub err: Option<AsserterError>,
}

pub(crate) fn non_asserter_tests<T, F>(tests: IndexMap<&str, ServerTest<T>>, mut func: F)
where
    T: Default,
    F: FnMut(&T) -> AssertResult<()>,
{
    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");
        let res = func(&test.request);
        if let Err(err) = res {
            assert!(
                test.err
                    .map(|e| err.to_string().contains(&e.to_string()))
                    .unwrap_or_default()
            );
        } else {
            assert_eq!(None, test.err);
        }
    });
}

pub(crate) fn singleton_asserter_tests<T, F>(
    asserter: Asserter,
    tests: IndexMap<&str, ServerTest<T>>,
    mut func: F,
) where
    T: Default,
    F: FnMut(&T) -> AssertResult<()>,
{
    tests.into_iter().for_each(|(name, test)| {
        println!("test: {name}");

        todo!()
    });
}
