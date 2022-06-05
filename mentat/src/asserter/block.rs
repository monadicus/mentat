use crate::{
    identifiers::BlockIdentifier,
    models::{Amount, Currency},
};

/// Currency ensures a *types.Currency is valid.
fn currency(currency: &Currency) -> Result<(), String> {
	//TODO if currency nil
    // we use a usize this error doesn't apply?
    if currency.decimals < 0 {
        return Err(todo!());
    }

    if currency.symbol.is_empty(){
        return Err(todo!());
    }

    Ok(())
}

/// Amount ensures a types.Amount has an
/// integer value, specified precision, and symbol.
pub(crate) fn amount(amount: &Amount) -> Result<(), String> {
	// or if currency nil
    if amount.value.is_empty() {
        return Err(todo!());
    }

    if amount.value.parse::<i128>().is_err() {
        return Err(todo!());
    }

    currency(&amount.currency)
}

/// BlockIdentifier ensures a types.BlockIdentifier
/// is well-formatte
pub(crate) fn block_identifier(block: &BlockIdentifier) -> Result<(), String> {
    // todo if block nil
    if block.hash.is_empty() {
        return Err(todo!());
    }

    if block.index < 0 {
        return Err(todo!());
    }

    Ok(())
}

static MIN_UNIX_EPOCH: i64 = 946713600000;
static MAX_UNIX_EPOCH: i64 = 2209017600000;


/// Timestamp returns an error if the timestamp
/// on a block is less than or equal to 0.
pub(crate) fn timestamp(timestamp: i64) -> Result<(), String> {
	if timestamp < MIN_UNIX_EPOCH {
		Err(format!("{}: {timestamp}", todo!()))
	} else if timestamp > MAX_UNIX_EPOCH {
		Err(format!("{}: {timestamp}", todo!()))
	} else {
		Ok(())
	}
}