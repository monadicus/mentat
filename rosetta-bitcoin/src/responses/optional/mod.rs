use mentat::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct Address {
    pub address: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct Connections {
    pub connections: u64,
    pub connections_in: u64,
    pub connections_out: u64,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct Network {
    pub totalbytesrecv: u64,
    pub totalbytessent: u64,
}
