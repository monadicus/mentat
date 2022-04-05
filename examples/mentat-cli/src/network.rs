use chrono::{DateTime, NaiveDateTime, Utc};
use clap::Parser;
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};
use mentat::{
    identifiers::{NetworkIdentifier, SubNetworkIdentifier},
    requests::NetworkRequest,
    responses::NetworkStatusResponse,
};

#[derive(Parser)]
pub(crate) struct NetworkOpts {
    #[clap(subcommand)]
    pub(crate) subcmd: NetworkSubCommand,
}

#[derive(Parser)]
pub(crate) enum NetworkSubCommand {
    List(NetworkListOpts),
    Status(NetworkStatusOpts),
    Options(NetworkStatusOpts),
}

#[derive(Parser)]
pub(crate) struct NetworkListOpts;

#[derive(Parser)]
pub(crate) struct NetworkStatusOpts {
    pub(crate) blockchain: String,
    pub(crate) network: String,
    #[clap(default_value = "")]
    pub(crate) subnetwork: String,
}

impl Into<NetworkRequest> for NetworkStatusOpts {
    fn into(self) -> NetworkRequest {
        NetworkRequest {
            network_identifier: NetworkIdentifier {
                blockchain: self.blockchain,
                network: self.network,
                sub_network_identifier: if self.subnetwork.len() > 0 {
                    Some(SubNetworkIdentifier {
                        network: self.subnetwork,
                        ..Default::default()
                    })
                } else {
                    None
                },
            },
            ..Default::default()
        }
    }
}

pub(crate) fn list_table(nets: Vec<NetworkIdentifier>) {
    let table = nets
        .into_iter()
        .map(|net| {
            vec![
                net.blockchain.cell(),
                net.network.cell(),
                match net.sub_network_identifier {
                    Some(id) => id.network.cell(),
                    None => "None".cell(),
                },
            ]
        })
        .collect::<Vec<Vec<CellStruct>>>()
        .table()
        .title(vec![
            "Blockchain".cell().bold(true),
            "Network".cell().bold(true),
            "Sub Network".cell().bold(true),
        ]);

    print_stdout(table).unwrap();
}

pub(crate) fn status_table(status: NetworkStatusResponse) {
    let current_timestamp_naive =
        NaiveDateTime::from_timestamp(status.current_block_timestamp as i64, 0);
    let current_timestamp =
        DateTime::<Utc>::from_utc(current_timestamp_naive, Utc).format("%Y-%m-%d %H:%M:%S");

    let blocks_table = vec![
        vec![
            "Current".cell(),
            status
                .current_block_identifier
                .index
                .cell()
                .justify(Justify::Right),
            status.current_block_identifier.hash.cell(),
            current_timestamp.cell(),
        ],
        vec![
            "Genesis".cell(),
            status
                .genesis_block_identifier
                .index
                .cell()
                .justify(Justify::Right),
            status.genesis_block_identifier.hash.cell(),
        ],
        if let Some(oldest) = status.oldest_block_identifier {
            vec![
                "Oldest".cell(),
                oldest.index.cell().justify(Justify::Right),
                oldest.hash.cell(),
            ]
        } else {
            vec![
                "Oldest".cell(),
                "n/a".cell().justify(Justify::Right),
                "n/a".cell(),
            ]
        },
    ]
    .table()
    .title(vec![
        "Name".cell().bold(true),
        "Index".cell().bold(true),
        "Hash".cell().bold(true),
        "Timestamp".cell().bold(true),
    ]);

    print_stdout(blocks_table).unwrap();

    if let Some(sync_status) = status.sync_status {
        println!(
            "\nSync Status:\n  \
        Current Index: {}\n  \
         Target Index: {}\n  \
                Stage: {}\n  \
                Synced: {}
        ",
            sync_status
                .current_index
                .map(|i| i.to_string())
                .unwrap_or("none".to_string()),
            sync_status
                .target_index
                .map(|i| i.to_string())
                .unwrap_or("none".to_string()),
            sync_status.stage.unwrap_or("none".to_string()),
            sync_status
                .synced
                .map(|i| i.to_string())
                .unwrap_or("n/a".to_string()),
        );
    } else {
        println!("\nSync Status: none")
    }

    println!("\nPeers:");
    if status.peers.len() == 0 {
        println!(" - none");
    }
    for peer in status.peers {
        println!(" - {}", peer.peer_id);
    }
}
