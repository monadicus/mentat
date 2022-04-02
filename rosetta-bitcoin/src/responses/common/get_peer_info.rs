use mentat::{misc::Peer, serde::Deserialize, IndexMap};

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct PeerInfo {
    // id: usize,
    addr: String,
    // addrbind: String,
    // addrlocal: String,
    // network: String,
    // mapped_as: usize,
    // services: String,
    // servicesnames: Vec<String>,
    relaytxes: bool,
    lastsend: usize,
    lastrecv: usize,
    // last_transaction: usize,
    // last_block: usize,
    // bytessent: usize,
    // bytesrecv: usize,
    // conntime: usize,
    // timeoffset: usize,
    // pingtime: usize,
    // minping: usize,
    // pingwait: usize,
    version: usize,
    subver: String,
    // inbound: bool,
    // addnode: bool,
    // connection_type: String,
    startingheight: usize,
    banscore: usize,
    synced_headers: usize,
    synced_blocks: usize,
    // inflight: Vec<usize>,
    // whitelisted: bool,
    // permissions: Vec<String>,
    // minfeefilter: usize,
    // bytessent_per_msg: BytesPerMsg,
    // bytesrecv_per_msg: BytesPerMsg,
}

impl From<PeerInfo> for Peer {
    fn from(peer: PeerInfo) -> Self {
        Self {
            peer_id: peer.addr.clone(),
            metadata: {
                let mut map = IndexMap::new();
                map.insert(String::from("addr"), peer.addr.into());
                map.insert(String::from("banscore"), peer.banscore.into());
                map.insert(String::from("lastrecv"), peer.lastrecv.into());
                map.insert(String::from("lastsend"), peer.lastsend.into());
                map.insert(String::from("relaytxes"), peer.relaytxes.into());
                map.insert(String::from("startingheight"), peer.startingheight.into());
                map.insert(String::from("subver"), peer.subver.into());
                map.insert(String::from("synced_blocks"), peer.synced_blocks.into());
                map.insert(String::from("synced_headers"), peer.synced_headers.into());
                map.insert(String::from("version"), peer.version.into());
                map
            },
        }
    }
}
