use indexmap::{indexmap, IndexMap};
use serde_json::json;

use crate::{asserter::account::contains_currency, models::Currency};

struct CurrencyTest {
    currencies: Vec<Currency>,
    currency: Currency,
    contains: bool,
}

#[test]
fn test_contains_currency() {
    let tests: IndexMap<&str, CurrencyTest> = indexmap!(
        "simple contains" => CurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            },
            contains: true,
        },
        "complex contains" => CurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah".to_string() => json!("hello")),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah".to_string() => json!("hello")),
            },
            contains: true,
        },
        "more complex contains" => CurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah2".to_string() => json!("bye"), "blah".to_string() => json!("hello")),
            },
            contains: true,
        },
        "empty" => CurrencyTest {
            currencies: Vec::new(),
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            },
            contains: false,
        },
        "symbol mismatch" => CurrencyTest {
            currencies: vec![Currency {
                symbol: "ERX".to_string(),
                decimals: 8,
                metadata: Default::default(),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            },
            contains: false,
        },
        "decimal mismatch" => CurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: Default::default(),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 6,
                metadata: Default::default(),
            },
            contains: false,
        },
        "metadata mismatch" => CurrencyTest {
            currencies: vec![Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah".to_string() => json!("hello")),
            }],
            currency: Currency {
                symbol: "BTC".to_string(),
                decimals: 8,
                metadata: indexmap!("blah".to_string() => json!("bye")),
            },
            contains: false,
        },
    );

    tests.iter().for_each(|(_, test)| {
        let exists = contains_currency(&test.currencies, &test.currency);
        assert_eq!(test.contains, exists);
    });
}
