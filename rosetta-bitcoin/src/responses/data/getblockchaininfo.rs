use mentat::{identifiers::NetworkIdentifier, responses::NetworkListResponse, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetBlockchainInfoResponse {
    chain: String,
}

impl From<GetBlockchainInfoResponse> for NetworkListResponse {
    fn from(info: GetBlockchainInfoResponse) -> Self {
        Self {
            network_identifiers: vec![NetworkIdentifier {
                blockchain: String::from("bitcoin"),
                network: match info.chain.as_ref() {
                    "main" => String::from("mainnet"),
                    "test" => String::from("testnet"),
                    _ => info.chain,
                },
                // rosetta didnt include this in their btc impl
                sub_network_identifier: None,
            }],
        }
    }
}
