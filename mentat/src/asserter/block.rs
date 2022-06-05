use crate::models::{Amount, Currency};

fn currency(currency: &Currency) -> Result<(), String> {
    // we use a usize this error doesn't apply?
    if currency.decimals < 0 {
        return Err("".to_string());
    }

    if currency.symbol == "" {
        return Err("".to_string());
    }

    Ok(())
}

pub(crate) fn amount(amount: &Amount) -> Result<(), String> {
    if amount.value == "" {
        return Err("".to_string());
    }

    if amount.value.parse::<i128>().is_err() {
        return Err("".to_string());
    }

    currency(&amount.currency)
}
