use clap::Parser;
use mentat::{identifiers::AccountIdentifier, models::Currency};

#[derive(Parser, Clone)]
pub(crate) struct AccountOpts {
    #[clap(short, long)]
    currencies: Vec<String>,

    #[clap(subcommand)]
    pub(crate) subcmd: AccountSubCommand,
}

impl AccountOpts {
    pub(crate) fn account_id(&self) -> AccountIdentifier {
        AccountIdentifier {
            address: match &self.subcmd {
                AccountSubCommand::Balance(AccountSubCommandOpts { address }) => address.clone(),
                AccountSubCommand::Coins(AccountSubCommandOpts { address }) => address.clone(),
            },
            ..Default::default()
        }
    }

    pub(crate) fn get_currencies(&self) -> Option<Vec<Currency>> {
        if self.currencies.len() == 0 {
            return None;
        }

        Some(
            self.currencies
                .iter()
                .cloned()
                .map(|c| Currency {
                    symbol: c,
                    ..Default::default()
                })
                .collect(),
        )
    }
}

#[derive(Parser, Clone)]
pub(crate) enum AccountSubCommand {
    Balance(AccountSubCommandOpts),
    Coins(AccountSubCommandOpts),
}

#[derive(Parser, Clone)]
pub(crate) struct AccountSubCommandOpts {
    pub(crate) address: String,
}
