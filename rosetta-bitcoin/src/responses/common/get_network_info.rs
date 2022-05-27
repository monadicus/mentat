use mentat::serde::Deserialize;

// #[derive(Clone, Debug, Deserialize)]
// #[serde(crate = "mentat::serde")]
// pub struct Network {
//     name: String,
//     limited: bool,
//     reachable: bool,
//     proxy: String,
//     proxy_randomize_credentials: bool,
// }

// #[derive(Clone, Debug, Deserialize)]
// #[serde(crate = "mentat::serde")]
// pub struct LocalAddress {
//     address: String,
//     port: usize,
//     score: usize,
// }

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetNetworkInfo {
    pub version: usize,
    // subversion: String,
    // protocol: usize,
    // localservices: String,
    // localservicesnames: Vec<String>,
    // localrelay: bool,
    // timeoffset: usize,
    // connections: usize,
    // connections_in: usize,
    // connections_out: usize,
    // networkactive: bool,
    // networks: Vec<Network>,
    // relayfee: usize,
    // incrementalfee: usize,
    // localaddresses: Vec<LocalAddress>,
    // warnings: String,
}
