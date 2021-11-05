mod dummy_data;
mod dummy_construction;
mod dummy_indexer;

use std::{convert::Infallible, net::SocketAddr, pin::Pin, sync::Arc};

use http::StatusCode;
use warp::{Filter, Future, reply::{Json, WithStatus}};

use crate::{api::*, requests::*, responses::*, misc::Error};

use self::{dummy_construction::DummyConstructionApi, dummy_data::DummyDataApi, dummy_indexer::DummyIndexerApi};


pub struct Server {
    construction_api: Arc<dyn ConstructionApi>,
    data_api: Arc<dyn DataApi>,
    indexer_api: Arc<dyn IndexerApi>,
}

fn not_implemented<T>() -> Result<T> {
    Err(Error {
        code: 501,
        message: "Not Implemented".to_string(),
        description: None,
        retriable: false,
        details: Default::default(),
    })
}

impl Default for Server {
    fn default() -> Self {
        Self {
            construction_api: Arc::new(DummyConstructionApi),
            data_api: Arc::new(DummyDataApi),
            indexer_api: Arc::new(DummyIndexerApi),
        }
    }
}

macro_rules! route {
    ($path:expr, $api:expr, $method:ident) => {
        {
            let api = $api.clone();
            $path.and(warp::addr::remote())
            .and(warp::body::json())
            .and_then(move |ip: Option<SocketAddr>, data: _| -> Pin<Box<dyn Future<Output=Result<WithStatus<Json>, Infallible>> + Send>> {
                let api = api.clone();
                Box::pin(async move {
                    let response = api.$method(Caller {
                        ip: ip.unwrap(),
                    }, data).await;
                    match response {
                        Ok(response) => Ok(warp::reply::with_status(warp::reply::json(&response), StatusCode::OK)),
                        Err(error) => Ok(warp::reply::with_status(warp::reply::json(&error), StatusCode::INTERNAL_SERVER_ERROR)),
                    }
                })
            })
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_data_api<T: DataApi + 'static>(&mut self, data_api: T) {
        self.with_dyn_data_api(Arc::new(data_api));
    }

    pub fn with_dyn_data_api(&mut self, data_api: Arc<dyn DataApi>) {
        self.data_api = data_api;
    }

    pub fn with_construction_api<T: ConstructionApi + 'static>(&mut self, construction_api: T) {
        self.with_dyn_construction_api(Arc::new(construction_api));
    }

    pub fn with_dyn_construction_api(&mut self, construction_api: Arc<dyn ConstructionApi>) {
        self.construction_api = construction_api;
    }

    pub fn with_indexer_api<T: IndexerApi + 'static>(&mut self, indexer_api: T) {
        self.with_dyn_indexer_api(Arc::new(indexer_api));
    }

    pub fn with_dyn_indexer_api(&mut self, indexer_api: Arc<dyn IndexerApi>) {
        self.indexer_api = indexer_api;
    }

    pub async fn serve(self, address: impl Into<SocketAddr>) {
        let network_list = route!(warp::path!("network" / "list"), self.data_api, network_list);
        let network_options = route!(warp::path!("network" / "options"), self.data_api, network_options);
        let network_status = route!(warp::path!("network" / "status"), self.data_api, network_status);

        let account_balance = route!(warp::path!("account" / "balance"), self.data_api, account_balance);
        let account_coins = route!(warp::path!("account" / "coins"), self.data_api, account_coins);

        let block = route!(warp::path!("block"), self.data_api, block);
        let block_transaction = route!(warp::path!("block" / "transaction"), self.data_api, block_transaction);

        let mempool = route!(warp::path!("mempool"), self.data_api, mempool);
        let mempool_transaction = route!(warp::path!("mempool" / "transaction"), self.data_api, mempool_transaction);

        let construction_combine = route!(warp::path!("construction" / "combine"), self.construction_api, combine);
        let construction_derive = route!(warp::path!("construction" / "derive"), self.construction_api, derive);
        let construction_hash = route!(warp::path!("construction" / "hash"), self.construction_api, hash);
        let construction_metadata = route!(warp::path!("construction" / "metadata"), self.construction_api, metadata);
        let construction_parse = route!(warp::path!("construction" / "parse"), self.construction_api, parse);
        let construction_payloads = route!(warp::path!("construction" / "payloads"), self.construction_api, payloads);
        let construction_preprocess = route!(warp::path!("construction" / "preprocess"), self.construction_api, preprocess);
        let construction_submit = route!(warp::path!("construction" / "submit"), self.construction_api, submit);

        let events_blocks = route!(warp::path!("events" / "blocks"), self.indexer_api, events_blocks);

        let search_transactions = route!(warp::path!("search" / "transactions"), self.indexer_api, search_transactions);

        warp::serve(warp::post()
            .and(
                network_list
                .or(network_options)
                .or(network_status)
                .or(account_balance)
                .or(account_coins)
                .or(block)
                .or(block_transaction)
                .or(mempool)
                .or(mempool_transaction)
                .or(construction_combine)
                .or(construction_derive)
                .or(construction_hash)
                .or(construction_metadata)
                .or(construction_parse)
                .or(construction_payloads)
                .or(construction_preprocess)
                .or(construction_submit)
                .or(events_blocks)
                .or(search_transactions)
            ))
            .run(address).await
    }
}