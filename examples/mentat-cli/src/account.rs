use clap::Parser;
use mentat::{
    identifiers::{
        AccountIdentifier,
        NetworkIdentifier,
        PartialBlockIdentifier,
        SubNetworkIdentifier,
    },
    models::Currency,
};

#[derive(Parser, Clone)]
pub(crate) struct AccountOpts {
    #[clap(short, long, default_value = "")]
    pub(crate) blockchain: String,
    #[clap(short, long, default_value = "")]
    pub(crate) network: String,
    #[clap(short, long, default_value = "")]
    pub(crate) subnetwork: String,

    #[clap(short, long)]
    currencies: Vec<String>,

    #[clap(subcommand)]
    pub(crate) subcmd: AccountSubCommand,

    pub(crate) address: String,
}

impl AccountOpts {
    pub(crate) fn net_id(&self) -> NetworkIdentifier {
        NetworkIdentifier {
            blockchain: self.blockchain.clone(),
            network: self.network.clone(),
            sub_network_identifier: if self.subnetwork.len() > 0 {
                Some(SubNetworkIdentifier {
                    network: self.subnetwork.clone(),
                    ..Default::default()
                })
            } else {
                None
            },
        }
    }

    pub(crate) fn account_id(&self) -> AccountIdentifier {
        AccountIdentifier {
            address: self.address.clone(),
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
    Balance(AccountBalanceOpts),
    Coins,
}

#[derive(Parser, Clone)]
pub(crate) struct AccountBalanceOpts {
    #[clap(long)]
    pub(crate) index: Option<u64>,
    #[clap(long)]
    pub(crate) hash: Option<String>,
}

impl Into<PartialBlockIdentifier> for AccountBalanceOpts {
    fn into(self) -> PartialBlockIdentifier {
        PartialBlockIdentifier {
            hash: self.hash,
            index: self.index,
        }
    }
}
