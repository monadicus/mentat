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
        match &self.subcmd {
            AccountSubCommand::Balance(AccountSubCommandOpts { address }) => address.clone(),
            AccountSubCommand::Coins(AccountSubCommandOpts { address }) => address.clone(),
        }
        .into()
    }

    pub(crate) fn get_currencies(&self) -> Option<Vec<Currency>> {
        if self.currencies.is_empty() {
            return None;
        }

        Some(self.currencies.iter().cloned().map(|c| c.into()).collect())
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
