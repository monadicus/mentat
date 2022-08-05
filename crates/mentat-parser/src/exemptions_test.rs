use super::*;

struct FindExemptionsTest {
    account: AccountIdentifier,
    currency: Option<Currency>,
    expected: Vec<BalanceExemption>,
}

#[test]
fn test_find_exemptions() {
    let tests = vec![CustomParserTest {
        name: "no exemptions",
        payload: FindExemptionsTest {
            account: AccountIdentifier {
                address: "test".into(),
                ..Default::default()
            },
            currency: Some(Currency {
                symbol: "BTC".into(),
                decimals: 8,
                metadata: Default::default(),
            }),
            expected: Vec::new(),
        },
        asserter_extras: (),
        parser_extras: Vec::new(),
        err: None,
    }];

    CustomParserTest::run(
        tests,
        |_| None,
        |a, pe| Parser::new(a, None, pe),
        |parser, payload| {
            let res = parser.find_exemptions(&payload.account, payload.currency.as_ref());

            if res != payload.expected {
                println!(
                    "test returned wrong value: `{:?}` != `{:?}`",
                    payload.expected, res
                );
                false
            } else {
                println!("ok!");
                true
            }
        },
    );
}
