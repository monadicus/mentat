#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{env, net::Ipv4Addr, sync::Arc};
use mentat::{cache::DefaultCacheInner, serve, server::Server, tokio};
mod call_api {
    use mentat::{
        api::{CallApi, CallerCallApi},
        async_trait,
    };
    pub struct SnarkosCallApi;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for SnarkosCallApi {
        #[inline]
        fn default() -> SnarkosCallApi {
            SnarkosCallApi {}
        }
    }
    impl CallerCallApi for SnarkosCallApi {}
    impl CallApi for SnarkosCallApi {}
}
mod construction_api {
    use mentat::{
        api::{Caller, CallerConstructionApi, ConstructionApi, MentatResponse},
        async_trait,
        requests::ConstructionSubmitRequest,
        responses::TransactionIdentifierResponse,
        Client,
    };
    use super::SnarkosJrpc;
    use crate::{
        jsonrpc_call,
        responses::{construction::*, Response},
    };
    pub struct SnarkosConstructionApi;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for SnarkosConstructionApi {
        #[inline]
        fn default() -> SnarkosConstructionApi {
            SnarkosConstructionApi {}
        }
    }
    impl CallerConstructionApi for SnarkosConstructionApi {}
    impl ConstructionApi for SnarkosConstructionApi {
        #[allow(
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn submit<'life0, 'async_trait>(
            &'life0 self,
            _caller: Caller,
            data: ConstructionSubmitRequest,
            client: Client,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = MentatResponse<TransactionIdentifierResponse>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<MentatResponse<TransactionIdentifierResponse>>
                {
                    return __ret;
                }
                let __self = self;
                let _caller = _caller;
                let data = data;
                let client = client;
                let __ret: MentatResponse<TransactionIdentifierResponse> = {
                    {
                        let req = SnarkosJrpc::new(
                            "sendtransaction",
                            <[_]>::into_vec(box [data.signed_transaction]),
                        );
                        let response = client
                            .post("http://127.0.0.1:3032")
                            .json(&req)
                            .send()
                            .await?;
                        let snarkos_json: Response<SendTransactionResponse> =
                            response.json().await?;
                        match snarkos_json {
                            Response::Ok(inner) => inner.into(),
                            Response::Err(err) => err.into(),
                        }
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
}
mod data_api {
    use mentat::{
        api::{Caller, CallerDataApi, DataApi, MentatResponse},
        async_trait,
        errors::MentatError,
        requests::*,
        responses::*,
        Client,
    };
    use super::SnarkosJrpc;
    use crate::{
        jsonrpc_call,
        responses::{data::*, Response},
    };
    pub struct SnarkosDataApi;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for SnarkosDataApi {
        #[inline]
        fn default() -> SnarkosDataApi {
            SnarkosDataApi {}
        }
    }
    impl CallerDataApi for SnarkosDataApi {}
    impl DataApi for SnarkosDataApi {
        #[allow(
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn block<'life0, 'async_trait>(
            &'life0 self,
            _caller: Caller,
            data: BlockRequest,
            client: Client,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = MentatResponse<BlockResponse>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<MentatResponse<BlockResponse>>
                {
                    return __ret;
                }
                let __self = self;
                let _caller = _caller;
                let data = data;
                let client = client;
                let __ret: MentatResponse<BlockResponse> = {
                    if let Some(block_id) = data.block_identifier.index {
                        {
                            let req = SnarkosJrpc::new("getblock", <[_]>::into_vec(box [block_id]));
                            let response = client
                                .post("http://127.0.0.1:3032")
                                .json(&req)
                                .send()
                                .await?;
                            let snarkos_json: Response<GetBlockResponse> = response.json().await?;
                            match snarkos_json {
                                Response::Ok(inner) => inner.into(),
                                Response::Err(err) => err.into(),
                            }
                        }
                    } else {
                        Err(MentatError::from("wtf"))
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn block_transaction<'life0, 'async_trait>(
            &'life0 self,
            _caller: Caller,
            data: BlockTransactionRequest,
            client: Client,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = MentatResponse<BlockTransactionResponse>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<MentatResponse<BlockTransactionResponse>>
                {
                    return __ret;
                }
                let __self = self;
                let _caller = _caller;
                let data = data;
                let client = client;
                let __ret: MentatResponse<BlockTransactionResponse> = {
                    let first = {
                        let req = SnarkosJrpc::new(
                            "gettransaction",
                            <[_]>::into_vec(box [data.block_identifier.hash]),
                        );
                        let response = client
                            .post("http://127.0.0.1:3032")
                            .json(&req)
                            .send()
                            .await?;
                        let snarkos_json: Response<GetTransactionResponse> =
                            response.json().await?;
                        match snarkos_json {
                            Response::Ok(inner) => inner,
                            Response::Err(err) => return err.into(),
                        }
                    };
                    let second = {
                        let req = SnarkosJrpc::new(
                            "getblocktransactions",
                            <[_]>::into_vec(box [data.block_identifier.index]),
                        );
                        let response = client
                            .post("http://127.0.0.1:3032")
                            .json(&req)
                            .send()
                            .await?;
                        let snarkos_json: Response<GetBlockTransactionsResponse> =
                            response.json().await?;
                        match snarkos_json {
                            Response::Ok(inner) => inner,
                            Response::Err(err) => return err.into(),
                        }
                    };
                    first + second
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn mempool<'life0, 'async_trait>(
            &'life0 self,
            _caller: Caller,
            _data: NetworkRequest,
            client: Client,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = MentatResponse<MempoolResponse>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<MentatResponse<MempoolResponse>>
                {
                    return __ret;
                }
                let __self = self;
                let _caller = _caller;
                let _data = _data;
                let client = client;
                let __ret: MentatResponse<MempoolResponse> = {
                    let data: Vec<u8> = Vec::new();
                    {
                        let req = SnarkosJrpc::new("getmemorypool", data);
                        let response = client
                            .post("http://127.0.0.1:3032")
                            .json(&req)
                            .send()
                            .await?;
                        let snarkos_json: Response<GetMemoryPoolResponse> = response.json().await?;
                        match snarkos_json {
                            Response::Ok(inner) => inner.into(),
                            Response::Err(err) => err.into(),
                        }
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
}
mod indexer_api {
    use mentat::{
        api::{CallerIndexerApi, IndexerApi},
        async_trait,
    };
    pub struct SnarkosIndexerApi;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for SnarkosIndexerApi {
        #[inline]
        fn default() -> SnarkosIndexerApi {
            SnarkosIndexerApi {}
        }
    }
    impl CallerIndexerApi for SnarkosIndexerApi {}
    impl IndexerApi for SnarkosIndexerApi {}
}
mod macros {}
mod node {
    use std::{
        io::{BufRead, BufReader, Read},
        thread,
    };
    use mentat::{async_trait, server::NodeRunner, tracing};
    pub struct SnarkOSNode;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for SnarkOSNode {
        #[inline]
        fn default() -> SnarkOSNode {
            SnarkOSNode {}
        }
    }
    impl NodeRunner for SnarkOSNode {
        #[allow(
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn start_node<'life0, 'async_trait>(
            &'life0 self,
            address: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<(), Box<dyn std::error::Error>>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<(), Box<dyn std::error::Error>>>
                {
                    return __ret;
                }
                let __self = self;
                let address = address;
                let __ret: Result<(), Box<dyn std::error::Error>> = {
                    let snarkos =
                        std::env::var("NODE").unwrap_or_else(|_| "/app/node-runner".to_string());
                    let mut child = std::process::Command::new(snarkos)
                        .args(&[
                            "--node",
                            &{
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["", ":4132"],
                                    &[::core::fmt::ArgumentV1::new_display(&address)],
                                ));
                                res
                            },
                            "--rpc",
                            &{
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["", ":3032"],
                                    &[::core::fmt::ArgumentV1::new_display(&address)],
                                ));
                                res
                            },
                            "--trial",
                            "--verbosity",
                            "2",
                        ])
                        .stderr(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::piped())
                        .spawn()?;
                    let stdout = child.stdout.take().unwrap();
                    let stderr = child.stderr.take().unwrap();
                    fn spawn_reader<T: 'static + Read + Send>(out: T, err: bool) {
                        let mut reader = BufReader::new(out).lines();
                        thread::spawn(move || {
                            while let Some(Ok(line)) = reader.next() {
                                if err {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                            use ::tracing::__macro_support::MacroCallsite;
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event rosetta-snarkos\\src\\node.rs:41",
                                                    "rosetta_snarkos::node",
                                                    ::tracing::Level::ERROR,
                                                    Some("rosetta-snarkos\\src\\node.rs"),
                                                    Some(41u32),
                                                    Some("rosetta_snarkos::node"),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(
                                                            &CALLSITE,
                                                        ),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            MacroCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = CALLSITE.interest();
                                                !interest.is_never()
                                                    && CALLSITE.is_enabled(interest)
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => {
                                                        ::tracing::log::Level::Error
                                                    }
                                                    ::tracing::Level::WARN => {
                                                        ::tracing::log::Level::Warn
                                                    }
                                                    ::tracing::Level::INFO => {
                                                        ::tracing::log::Level::Info
                                                    }
                                                    ::tracing::Level::DEBUG => {
                                                        ::tracing::log::Level::Debug
                                                    }
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level =
                                                                match ::tracing::Level::ERROR {
                                                                    ::tracing::Level::ERROR => {
                                                                        ::tracing::log::Level::Error
                                                                    }
                                                                    ::tracing::Level::WARN => {
                                                                        ::tracing::log::Level::Warn
                                                                    }
                                                                    ::tracing::Level::INFO => {
                                                                        ::tracing::log::Level::Info
                                                                    }
                                                                    ::tracing::Level::DEBUG => {
                                                                        ::tracing::log::Level::Debug
                                                                    }
                                                                    _ => {
                                                                        ::tracing::log::Level::Trace
                                                                    }
                                                                };
                                                            if level <= log::max_level() {
                                                                let log_meta =
                                                                    log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(
                                                                            "rosetta_snarkos::node",
                                                                        )
                                                                        .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\node.rs")) . module_path (Some ("rosetta_snarkos::node")) . line (Some (41u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = CALLSITE.metadata().fields().iter();
                                                CALLSITE.metadata().fields().value_set(&[(
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&::core::fmt::Arguments::new_v1(
                                                        &["SnarkOS: "],
                                                        &[::core::fmt::ArgumentV1::new_display(
                                                            &line,
                                                        )],
                                                    )
                                                        as &Value),
                                                )])
                                            });
                                        } else {
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => {
                                                    ::tracing::log::Level::Error
                                                }
                                                ::tracing::Level::WARN => {
                                                    ::tracing::log::Level::Warn
                                                }
                                                ::tracing::Level::INFO => {
                                                    ::tracing::log::Level::Info
                                                }
                                                ::tracing::Level::DEBUG => {
                                                    ::tracing::log::Level::Debug
                                                }
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => {
                                                                ::tracing::log::Level::Error
                                                            }
                                                            ::tracing::Level::WARN => {
                                                                ::tracing::log::Level::Warn
                                                            }
                                                            ::tracing::Level::INFO => {
                                                                ::tracing::log::Level::Info
                                                            }
                                                            ::tracing::Level::DEBUG => {
                                                                ::tracing::log::Level::Debug
                                                            }
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target("rosetta_snarkos::node")
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\node.rs")) . module_path (Some ("rosetta_snarkos::node")) . line (Some (41u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["SnarkOS: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& line)]) as & Value))]) }))])) . build ()) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                } else {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                            use ::tracing::__macro_support::MacroCallsite;
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event rosetta-snarkos\\src\\node.rs:43",
                                                    "rosetta_snarkos::node",
                                                    ::tracing::Level::INFO,
                                                    Some("rosetta-snarkos\\src\\node.rs"),
                                                    Some(43u32),
                                                    Some("rosetta_snarkos::node"),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(
                                                            &CALLSITE,
                                                        ),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            MacroCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::INFO
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::INFO
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = CALLSITE.interest();
                                                !interest.is_never()
                                                    && CALLSITE.is_enabled(interest)
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => {
                                                        ::tracing::log::Level::Error
                                                    }
                                                    ::tracing::Level::WARN => {
                                                        ::tracing::log::Level::Warn
                                                    }
                                                    ::tracing::Level::INFO => {
                                                        ::tracing::log::Level::Info
                                                    }
                                                    ::tracing::Level::DEBUG => {
                                                        ::tracing::log::Level::Debug
                                                    }
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO
                                                            {
                                                                ::tracing::Level::ERROR => {
                                                                    ::tracing::log::Level::Error
                                                                }
                                                                ::tracing::Level::WARN => {
                                                                    ::tracing::log::Level::Warn
                                                                }
                                                                ::tracing::Level::INFO => {
                                                                    ::tracing::log::Level::Info
                                                                }
                                                                ::tracing::Level::DEBUG => {
                                                                    ::tracing::log::Level::Debug
                                                                }
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let log_meta =
                                                                    log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(
                                                                            "rosetta_snarkos::node",
                                                                        )
                                                                        .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\node.rs")) . module_path (Some ("rosetta_snarkos::node")) . line (Some (43u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = CALLSITE.metadata().fields().iter();
                                                CALLSITE.metadata().fields().value_set(&[(
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&::core::fmt::Arguments::new_v1(
                                                        &["SnarkOS: "],
                                                        &[::core::fmt::ArgumentV1::new_display(
                                                            &line,
                                                        )],
                                                    )
                                                        as &Value),
                                                )])
                                            });
                                        } else {
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => {
                                                    ::tracing::log::Level::Error
                                                }
                                                ::tracing::Level::WARN => {
                                                    ::tracing::log::Level::Warn
                                                }
                                                ::tracing::Level::INFO => {
                                                    ::tracing::log::Level::Info
                                                }
                                                ::tracing::Level::DEBUG => {
                                                    ::tracing::log::Level::Debug
                                                }
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => {
                                                                ::tracing::log::Level::Error
                                                            }
                                                            ::tracing::Level::WARN => {
                                                                ::tracing::log::Level::Warn
                                                            }
                                                            ::tracing::Level::INFO => {
                                                                ::tracing::log::Level::Info
                                                            }
                                                            ::tracing::Level::DEBUG => {
                                                                ::tracing::log::Level::Debug
                                                            }
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target("rosetta_snarkos::node")
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\node.rs")) . module_path (Some ("rosetta_snarkos::node")) . line (Some (43u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["SnarkOS: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& line)]) as & Value))]) }))])) . build ()) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                }
                            }
                        });
                    }
                    spawn_reader(stdout, false);
                    spawn_reader(stderr, true);
                    Ok(())
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
}
mod request {
    use mentat::serde::Serialize;
    #[serde(crate = "mentat::serde")]
    pub struct SnarkosJrpc<P: Serialize> {
        jsonrpc: String,
        id: String,
        method: String,
        params: Vec<P>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<P: ::core::fmt::Debug + Serialize> ::core::fmt::Debug for SnarkosJrpc<P> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SnarkosJrpc {
                    jsonrpc: ref __self_0_0,
                    id: ref __self_0_1,
                    method: ref __self_0_2,
                    params: ref __self_0_3,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "SnarkosJrpc");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "jsonrpc",
                        &&(*__self_0_0),
                    );
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_1));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "method",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "params",
                        &&(*__self_0_3),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use mentat::serde as _serde;
        #[automatically_derived]
        impl<P: Serialize> mentat::serde::Serialize for SnarkosJrpc<P>
        where
            P: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> mentat::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: mentat::serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "SnarkosJrpc",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "jsonrpc",
                    &self.jsonrpc,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "method",
                    &self.method,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "params",
                    &self.params,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl<P: Serialize> SnarkosJrpc<P> {
        pub fn new(method: &str, params: Vec<P>) -> Self {
            Self {
                jsonrpc: "2.0".to_string(),
                id: "1".to_string(),
                method: method.to_string(),
                params,
            }
        }
    }
}
mod responses {
    pub mod common {
        use mentat::serde::Deserialize;
        mod snarkos_event {
            use mentat::identifiers::OperationIdentifier;
            use super::*;
            #[serde(crate = "mentat::serde")]
            pub struct SnarkosEvent {
                pub id: u64,
                pub index: u64,
                pub record_view_key: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for SnarkosEvent {
                #[inline]
                fn clone(&self) -> SnarkosEvent {
                    match *self {
                        SnarkosEvent {
                            id: ref __self_0_0,
                            index: ref __self_0_1,
                            record_view_key: ref __self_0_2,
                        } => SnarkosEvent {
                            id: ::core::clone::Clone::clone(&(*__self_0_0)),
                            index: ::core::clone::Clone::clone(&(*__self_0_1)),
                            record_view_key: ::core::clone::Clone::clone(&(*__self_0_2)),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for SnarkosEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        SnarkosEvent {
                            id: ref __self_0_0,
                            index: ref __self_0_1,
                            record_view_key: ref __self_0_2,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "SnarkosEvent");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "id",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "index",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "record_view_key",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for SnarkosEvent {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "id" => _serde::__private::Ok(__Field::__field0),
                                    "index" => _serde::__private::Ok(__Field::__field1),
                                    "record_view_key" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"id" => _serde::__private::Ok(__Field::__field0),
                                    b"index" => _serde::__private::Ok(__Field::__field1),
                                    b"record_view_key" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<SnarkosEvent>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SnarkosEvent;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct SnarkosEvent",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match match _serde::de::SeqAccess::next_element::<u64>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct SnarkosEvent with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<u64>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct SnarkosEvent with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct SnarkosEvent with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(SnarkosEvent {
                                    id: __field0,
                                    index: __field1,
                                    record_view_key: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<u64> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<u64> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("id")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<u64>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("index")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<u64>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("record_view_key")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("index") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "record_view_key",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(SnarkosEvent {
                                    id: __field0,
                                    index: __field1,
                                    record_view_key: __field2,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["id", "index", "record_view_key"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SnarkosEvent",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<SnarkosEvent>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl From<SnarkosEvent> for OperationIdentifier {
                fn from(event: SnarkosEvent) -> Self {
                    Self {
                        index: event.index,
                        network_index: Some(event.id),
                    }
                }
            }
        }
        pub use snarkos_event::*;
        mod snarkos_transaction {
            use mentat::{identifiers::TransactionIdentifier, models::Transaction, IndexMap};
            use super::*;
            #[serde(crate = "mentat::serde")]
            pub struct SnarkosTransaction {
                pub inner_circuit_id: String,
                pub ledger_root: String,
                pub transaction_id: String,
                pub transitions: Vec<SnarkosTransition>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for SnarkosTransaction {
                #[inline]
                fn clone(&self) -> SnarkosTransaction {
                    match *self {
                        SnarkosTransaction {
                            inner_circuit_id: ref __self_0_0,
                            ledger_root: ref __self_0_1,
                            transaction_id: ref __self_0_2,
                            transitions: ref __self_0_3,
                        } => SnarkosTransaction {
                            inner_circuit_id: ::core::clone::Clone::clone(&(*__self_0_0)),
                            ledger_root: ::core::clone::Clone::clone(&(*__self_0_1)),
                            transaction_id: ::core::clone::Clone::clone(&(*__self_0_2)),
                            transitions: ::core::clone::Clone::clone(&(*__self_0_3)),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for SnarkosTransaction {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        SnarkosTransaction {
                            inner_circuit_id: ref __self_0_0,
                            ledger_root: ref __self_0_1,
                            transaction_id: ref __self_0_2,
                            transitions: ref __self_0_3,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "SnarkosTransaction");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "inner_circuit_id",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "ledger_root",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "transaction_id",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "transitions",
                                &&(*__self_0_3),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for SnarkosTransaction {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "inner_circuit_id" => _serde::__private::Ok(__Field::__field0),
                                    "ledger_root" => _serde::__private::Ok(__Field::__field1),
                                    "transaction_id" => _serde::__private::Ok(__Field::__field2),
                                    "transitions" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"inner_circuit_id" => _serde::__private::Ok(__Field::__field0),
                                    b"ledger_root" => _serde::__private::Ok(__Field::__field1),
                                    b"transaction_id" => _serde::__private::Ok(__Field::__field2),
                                    b"transitions" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<SnarkosTransaction>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SnarkosTransaction;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct SnarkosTransaction",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct SnarkosTransaction with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct SnarkosTransaction with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field2 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct SnarkosTransaction with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field3 = match match _serde::de::SeqAccess::next_element::<
                                    Vec<SnarkosTransition>,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                3usize,
                                                &"struct SnarkosTransaction with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(SnarkosTransaction {
                                    inner_circuit_id: __field0,
                                    ledger_root: __field1,
                                    transaction_id: __field2,
                                    transitions: __field3,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field3: _serde::__private::Option<
                                    Vec<SnarkosTransition>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("inner_circuit_id")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("ledger_root")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("transaction_id")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("transitions")) ;
                                            }
                                            __field3 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    Vec<SnarkosTransition>,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "inner_circuit_id",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("ledger_root") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("transaction_id")
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("transitions") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(SnarkosTransaction {
                                    inner_circuit_id: __field0,
                                    ledger_root: __field1,
                                    transaction_id: __field2,
                                    transitions: __field3,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "inner_circuit_id",
                            "ledger_root",
                            "transaction_id",
                            "transitions",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SnarkosTransaction",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<SnarkosTransaction>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl From<SnarkosTransaction> for Transaction {
                fn from(transaction: SnarkosTransaction) -> Self {
                    Transaction {
                        transaction_identifier: TransactionIdentifier {
                            hash: transaction.transaction_id,
                        },
                        operations: transaction
                            .transitions
                            .into_iter()
                            .map(|t| t.into())
                            .collect(),
                        related_transactions: None,
                        metadata: IndexMap::new(),
                    }
                }
            }
        }
        pub use snarkos_transaction::*;
        mod snarkos_transactions {
            use mentat::models::Transaction;
            use super::*;
            #[serde(crate = "mentat::serde")]
            pub struct SnarkosTransactions {
                pub transactions: Vec<SnarkosTransaction>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for SnarkosTransactions {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        SnarkosTransactions {
                            transactions: ref __self_0_0,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "SnarkosTransactions");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "transactions",
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for SnarkosTransactions {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "transactions" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"transactions" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<SnarkosTransactions>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SnarkosTransactions;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct SnarkosTransactions",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match match _serde::de::SeqAccess::next_element::<
                                    Vec<SnarkosTransaction>,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct SnarkosTransactions with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(SnarkosTransactions {
                                    transactions: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<
                                    Vec<SnarkosTransaction>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("transactions")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    Vec<SnarkosTransaction>,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("transactions") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(SnarkosTransactions {
                                    transactions: __field0,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["transactions"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SnarkosTransactions",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<SnarkosTransactions>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[allow(clippy::from_over_into)]
            impl Into<Vec<Transaction>> for SnarkosTransactions {
                fn into(self) -> Vec<Transaction> {
                    self.transactions.into_iter().map(|t| t.into()).collect()
                }
            }
        }
        pub use snarkos_transactions::*;
        mod snarkos_transition {
            use mentat::{
                identifiers::{CoinIdentifier, OperationIdentifier},
                models::{Amount, CoinAction, CoinChange, Currency, Operation},
                IndexMap,
            };
            use super::*;
            #[serde(crate = "mentat::serde")]
            pub struct SnarkosTransition {
                pub ciphertexts: Vec<String>,
                pub commitments: Vec<String>,
                pub events: Vec<SnarkosEvent>,
                pub proof: String,
                pub serial_numbers: Vec<String>,
                pub transition_id: String,
                pub value_balance: i32,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for SnarkosTransition {
                #[inline]
                fn clone(&self) -> SnarkosTransition {
                    match *self {
                        SnarkosTransition {
                            ciphertexts: ref __self_0_0,
                            commitments: ref __self_0_1,
                            events: ref __self_0_2,
                            proof: ref __self_0_3,
                            serial_numbers: ref __self_0_4,
                            transition_id: ref __self_0_5,
                            value_balance: ref __self_0_6,
                        } => SnarkosTransition {
                            ciphertexts: ::core::clone::Clone::clone(&(*__self_0_0)),
                            commitments: ::core::clone::Clone::clone(&(*__self_0_1)),
                            events: ::core::clone::Clone::clone(&(*__self_0_2)),
                            proof: ::core::clone::Clone::clone(&(*__self_0_3)),
                            serial_numbers: ::core::clone::Clone::clone(&(*__self_0_4)),
                            transition_id: ::core::clone::Clone::clone(&(*__self_0_5)),
                            value_balance: ::core::clone::Clone::clone(&(*__self_0_6)),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for SnarkosTransition {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        SnarkosTransition {
                            ciphertexts: ref __self_0_0,
                            commitments: ref __self_0_1,
                            events: ref __self_0_2,
                            proof: ref __self_0_3,
                            serial_numbers: ref __self_0_4,
                            transition_id: ref __self_0_5,
                            value_balance: ref __self_0_6,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "SnarkosTransition");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "ciphertexts",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "commitments",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "events",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "proof",
                                &&(*__self_0_3),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "serial_numbers",
                                &&(*__self_0_4),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "transition_id",
                                &&(*__self_0_5),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "value_balance",
                                &&(*__self_0_6),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for SnarkosTransition {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    5u64 => _serde::__private::Ok(__Field::__field5),
                                    6u64 => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "ciphertexts" => _serde::__private::Ok(__Field::__field0),
                                    "commitments" => _serde::__private::Ok(__Field::__field1),
                                    "events" => _serde::__private::Ok(__Field::__field2),
                                    "proof" => _serde::__private::Ok(__Field::__field3),
                                    "serial_numbers" => _serde::__private::Ok(__Field::__field4),
                                    "transition_id" => _serde::__private::Ok(__Field::__field5),
                                    "value_balance" => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"ciphertexts" => _serde::__private::Ok(__Field::__field0),
                                    b"commitments" => _serde::__private::Ok(__Field::__field1),
                                    b"events" => _serde::__private::Ok(__Field::__field2),
                                    b"proof" => _serde::__private::Ok(__Field::__field3),
                                    b"serial_numbers" => _serde::__private::Ok(__Field::__field4),
                                    b"transition_id" => _serde::__private::Ok(__Field::__field5),
                                    b"value_balance" => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<SnarkosTransition>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SnarkosTransition;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct SnarkosTransition",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<Vec<String>>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct SnarkosTransition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<Vec<String>>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct SnarkosTransition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field2 = match match _serde::de::SeqAccess::next_element::<
                                    Vec<SnarkosEvent>,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct SnarkosTransition with 7 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field3 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct SnarkosTransition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field4 =
                                    match match _serde::de::SeqAccess::next_element::<Vec<String>>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct SnarkosTransition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field5 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    5usize,
                                                    &"struct SnarkosTransition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field6 = match match _serde::de::SeqAccess::next_element::<i32>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                6usize,
                                                &"struct SnarkosTransition with 7 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(SnarkosTransition {
                                    ciphertexts: __field0,
                                    commitments: __field1,
                                    events: __field2,
                                    proof: __field3,
                                    serial_numbers: __field4,
                                    transition_id: __field5,
                                    value_balance: __field6,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Vec<String>> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<Vec<String>> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<Vec<SnarkosEvent>> =
                                    _serde::__private::None;
                                let mut __field3: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field4: _serde::__private::Option<Vec<String>> =
                                    _serde::__private::None;
                                let mut __field5: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field6: _serde::__private::Option<i32> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("ciphertexts")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Vec<String>>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("commitments")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Vec<String>>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("events")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    Vec<SnarkosEvent>,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("proof")) ;
                                            }
                                            __field3 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::__private::Option::is_some(&__field4) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("serial_numbers")) ;
                                            }
                                            __field4 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Vec<String>>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field5 => {
                                            if _serde::__private::Option::is_some(&__field5) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("transition_id")) ;
                                            }
                                            __field5 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field6 => {
                                            if _serde::__private::Option::is_some(&__field6) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("value_balance")) ;
                                            }
                                            __field6 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<i32>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("ciphertexts") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("commitments") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("events") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("proof") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::__private::Some(__field4) => __field4,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("serial_numbers")
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field5 = match __field5 {
                                    _serde::__private::Some(__field5) => __field5,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("transition_id")
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field6 = match __field6 {
                                    _serde::__private::Some(__field6) => __field6,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("value_balance")
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(SnarkosTransition {
                                    ciphertexts: __field0,
                                    commitments: __field1,
                                    events: __field2,
                                    proof: __field3,
                                    serial_numbers: __field4,
                                    transition_id: __field5,
                                    value_balance: __field6,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "ciphertexts",
                            "commitments",
                            "events",
                            "proof",
                            "serial_numbers",
                            "transition_id",
                            "value_balance",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SnarkosTransition",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<SnarkosTransition>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl From<SnarkosTransition> for Operation {
                fn from(transition: SnarkosTransition) -> Self {
                    Self {
                        operation_identifier: OperationIdentifier {
                            index: 0,
                            network_index: None,
                        },
                        related_operations: Some(
                            transition.events.into_iter().map(|e| e.into()).collect(),
                        ),
                        type_: "N/A".to_string(),
                        status: None,
                        account: None,
                        amount: Some(Amount {
                            value: transition.value_balance.to_string(),
                            currency: Currency {
                                symbol: "ALEO".to_string(),
                                decimals: 18,
                                metadata: IndexMap::new(),
                            },
                            metadata: IndexMap::new(),
                        }),
                        coin_change: Some(CoinChange {
                            coin_identifier: CoinIdentifier {
                                identifier: transition.transition_id,
                            },
                            coin_action: CoinAction::CoinCreated,
                        }),
                        metadata: IndexMap::new(),
                    }
                }
            }
        }
        pub use snarkos_transition::*;
    }
    pub mod construction {
        use mentat::serde::Deserialize;
        mod sendtransaction {
            use mentat::{
                api::MentatResponse, identifiers::TransactionIdentifier,
                responses::TransactionIdentifierResponse, IndexMap, Json,
            };
            use super::*;
            #[serde(crate = "mentat::serde")]
            pub struct SendTransactionResponse {
                _jsonrpc: String,
                result: String,
                _id: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for SendTransactionResponse {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        SendTransactionResponse {
                            _jsonrpc: ref __self_0_0,
                            result: ref __self_0_1,
                            _id: ref __self_0_2,
                        } => {
                            let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(
                                f,
                                "SendTransactionResponse",
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_jsonrpc",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "result",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_id",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for SendTransactionResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    "result" => _serde::__private::Ok(__Field::__field1),
                                    "_id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    b"result" => _serde::__private::Ok(__Field::__field1),
                                    b"_id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<SendTransactionResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SendTransactionResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct SendTransactionResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct SendTransactionResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct SendTransactionResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct SendTransactionResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(SendTransactionResponse {
                                    _jsonrpc: __field0,
                                    result: __field1,
                                    _id: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_jsonrpc")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("result")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_id")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_jsonrpc") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("result") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(SendTransactionResponse {
                                    _jsonrpc: __field0,
                                    result: __field1,
                                    _id: __field2,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["_jsonrpc", "result", "_id"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SendTransactionResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<SendTransactionResponse>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for SendTransactionResponse {
                #[inline]
                fn clone(&self) -> SendTransactionResponse {
                    match *self {
                        SendTransactionResponse {
                            _jsonrpc: ref __self_0_0,
                            result: ref __self_0_1,
                            _id: ref __self_0_2,
                        } => SendTransactionResponse {
                            _jsonrpc: ::core::clone::Clone::clone(&(*__self_0_0)),
                            result: ::core::clone::Clone::clone(&(*__self_0_1)),
                            _id: ::core::clone::Clone::clone(&(*__self_0_2)),
                        },
                    }
                }
            }
            impl From<SendTransactionResponse> for MentatResponse<TransactionIdentifierResponse> {
                fn from(
                    response: SendTransactionResponse,
                ) -> MentatResponse<TransactionIdentifierResponse> {
                    Ok(Json(TransactionIdentifierResponse {
                        transaction_identifier: TransactionIdentifier {
                            hash: response.result,
                        },
                        metadata: IndexMap::new(),
                    }))
                }
            }
        }
        pub use sendtransaction::*;
    }
    pub mod data {
        use mentat::serde::Deserialize;
        mod getblock {
            use mentat::{
                api::MentatResponse, identifiers::BlockIdentifier, models::Block,
                responses::BlockResponse, serde_json::Value, IndexMap, Json,
            };
            use super::*;
            use crate::responses::common::SnarkosTransactions;
            #[serde(crate = "mentat::serde")]
            struct Metadata {
                cumulative_weight: u64,
                difficulty_target: u64,
                height: u64,
                timestamp: u64,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Metadata {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Metadata {
                            cumulative_weight: ref __self_0_0,
                            difficulty_target: ref __self_0_1,
                            height: ref __self_0_2,
                            timestamp: ref __self_0_3,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "Metadata");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "cumulative_weight",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "difficulty_target",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "height",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "timestamp",
                                &&(*__self_0_3),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for Metadata {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "cumulative_weight" => _serde::__private::Ok(__Field::__field0),
                                    "difficulty_target" => _serde::__private::Ok(__Field::__field1),
                                    "height" => _serde::__private::Ok(__Field::__field2),
                                    "timestamp" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"cumulative_weight" => {
                                        _serde::__private::Ok(__Field::__field0)
                                    }
                                    b"difficulty_target" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    b"height" => _serde::__private::Ok(__Field::__field2),
                                    b"timestamp" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Metadata>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Metadata;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Metadata",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match match _serde::de::SeqAccess::next_element::<u64>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct Metadata with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<u64>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct Metadata with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match match _serde::de::SeqAccess::next_element::<u64>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct Metadata with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field3 = match match _serde::de::SeqAccess::next_element::<u64>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                3usize,
                                                &"struct Metadata with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(Metadata {
                                    cumulative_weight: __field0,
                                    difficulty_target: __field1,
                                    height: __field2,
                                    timestamp: __field3,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<u64> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<u64> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<u64> =
                                    _serde::__private::None;
                                let mut __field3: _serde::__private::Option<u64> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("cumulative_weight")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<u64>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("difficulty_target")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<u64>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("height")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<u64>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("timestamp")) ;
                                            }
                                            __field3 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<u64>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "cumulative_weight",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "difficulty_target",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("height") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("timestamp") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(Metadata {
                                    cumulative_weight: __field0,
                                    difficulty_target: __field1,
                                    height: __field2,
                                    timestamp: __field3,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "cumulative_weight",
                            "difficulty_target",
                            "height",
                            "timestamp",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Metadata",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Metadata>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl From<Metadata> for IndexMap<String, Value> {
                fn from(metadata: Metadata) -> Self {
                    let mut map = IndexMap::new();
                    map.insert(
                        "cumulative_weight".to_string(),
                        metadata.cumulative_weight.into(),
                    );
                    map.insert(
                        "difficulty_target".to_string(),
                        metadata.difficulty_target.into(),
                    );
                    map.insert("height".to_string(), metadata.height.into());
                    map.insert("timestamp".to_string(), metadata.timestamp.into());
                    map
                }
            }
            #[serde(crate = "mentat::serde")]
            struct Proof {
                _hiding: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Proof {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Proof {
                            _hiding: ref __self_0_0,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "Proof");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_hiding",
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for Proof {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_hiding" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_hiding" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Proof>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Proof;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(__formatter, "struct Proof")
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct Proof with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(Proof { _hiding: __field0 })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_hiding")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_hiding") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(Proof { _hiding: __field0 })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["_hiding"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Proof",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Proof>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[serde(crate = "mentat::serde")]
            struct Header {
                metadata: Metadata,
                _nonce: String,
                _previous_ledger_root: String,
                _proof: Proof,
                _transactions_root: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Header {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Header {
                            metadata: ref __self_0_0,
                            _nonce: ref __self_0_1,
                            _previous_ledger_root: ref __self_0_2,
                            _proof: ref __self_0_3,
                            _transactions_root: ref __self_0_4,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "Header");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "metadata",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_nonce",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_previous_ledger_root",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_proof",
                                &&(*__self_0_3),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_transactions_root",
                                &&(*__self_0_4),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for Header {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "metadata" => _serde::__private::Ok(__Field::__field0),
                                    "_nonce" => _serde::__private::Ok(__Field::__field1),
                                    "_previous_ledger_root" => {
                                        _serde::__private::Ok(__Field::__field2)
                                    }
                                    "_proof" => _serde::__private::Ok(__Field::__field3),
                                    "_transactions_root" => {
                                        _serde::__private::Ok(__Field::__field4)
                                    }
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"metadata" => _serde::__private::Ok(__Field::__field0),
                                    b"_nonce" => _serde::__private::Ok(__Field::__field1),
                                    b"_previous_ledger_root" => {
                                        _serde::__private::Ok(__Field::__field2)
                                    }
                                    b"_proof" => _serde::__private::Ok(__Field::__field3),
                                    b"_transactions_root" => {
                                        _serde::__private::Ok(__Field::__field4)
                                    }
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Header>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Header;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Header",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<Metadata>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct Header with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct Header with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field2 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct Header with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field3 =
                                    match match _serde::de::SeqAccess::next_element::<Proof>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct Header with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field4 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct Header with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(Header {
                                    metadata: __field0,
                                    _nonce: __field1,
                                    _previous_ledger_root: __field2,
                                    _proof: __field3,
                                    _transactions_root: __field4,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Metadata> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field3: _serde::__private::Option<Proof> =
                                    _serde::__private::None;
                                let mut __field4: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("metadata")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Metadata>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_nonce")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_previous_ledger_root")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_proof")) ;
                                            }
                                            __field3 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Proof>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::__private::Option::is_some(&__field4) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_transactions_root")) ;
                                            }
                                            __field4 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("metadata") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_nonce") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "_previous_ledger_root",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_proof") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::__private::Some(__field4) => __field4,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "_transactions_root",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(Header {
                                    metadata: __field0,
                                    _nonce: __field1,
                                    _previous_ledger_root: __field2,
                                    _proof: __field3,
                                    _transactions_root: __field4,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "metadata",
                            "_nonce",
                            "_previous_ledger_root",
                            "_proof",
                            "_transactions_root",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Header",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Header>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[serde(crate = "mentat::serde")]
            struct BlockResult {
                block_hash: String,
                header: Header,
                previous_block_hash: String,
                transactions: SnarkosTransactions,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for BlockResult {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        BlockResult {
                            block_hash: ref __self_0_0,
                            header: ref __self_0_1,
                            previous_block_hash: ref __self_0_2,
                            transactions: ref __self_0_3,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "BlockResult");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "block_hash",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "header",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "previous_block_hash",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "transactions",
                                &&(*__self_0_3),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for BlockResult {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "block_hash" => _serde::__private::Ok(__Field::__field0),
                                    "header" => _serde::__private::Ok(__Field::__field1),
                                    "previous_block_hash" => {
                                        _serde::__private::Ok(__Field::__field2)
                                    }
                                    "transactions" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"block_hash" => _serde::__private::Ok(__Field::__field0),
                                    b"header" => _serde::__private::Ok(__Field::__field1),
                                    b"previous_block_hash" => {
                                        _serde::__private::Ok(__Field::__field2)
                                    }
                                    b"transactions" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<BlockResult>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = BlockResult;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct BlockResult",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct BlockResult with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<Header>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct BlockResult with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field2 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct BlockResult with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field3 = match match _serde::de::SeqAccess::next_element::<
                                    SnarkosTransactions,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                3usize,
                                                &"struct BlockResult with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(BlockResult {
                                    block_hash: __field0,
                                    header: __field1,
                                    previous_block_hash: __field2,
                                    transactions: __field3,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<Header> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field3: _serde::__private::Option<SnarkosTransactions> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("block_hash")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("header")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Header>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("previous_block_hash")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("transactions")) ;
                                            }
                                            __field3 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    SnarkosTransactions,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("block_hash") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("header") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "previous_block_hash",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("transactions") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(BlockResult {
                                    block_hash: __field0,
                                    header: __field1,
                                    previous_block_hash: __field2,
                                    transactions: __field3,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "block_hash",
                            "header",
                            "previous_block_hash",
                            "transactions",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "BlockResult",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<BlockResult>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[serde(crate = "mentat::serde")]
            pub struct GetBlockResponse {
                _jsonrpc: String,
                result: BlockResult,
                _id: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for GetBlockResponse {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        GetBlockResponse {
                            _jsonrpc: ref __self_0_0,
                            result: ref __self_0_1,
                            _id: ref __self_0_2,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "GetBlockResponse");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_jsonrpc",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "result",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_id",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for GetBlockResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    "result" => _serde::__private::Ok(__Field::__field1),
                                    "_id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    b"result" => _serde::__private::Ok(__Field::__field1),
                                    b"_id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetBlockResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetBlockResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetBlockResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct GetBlockResponse with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<BlockResult>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct GetBlockResponse with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field2 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct GetBlockResponse with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(GetBlockResponse {
                                    _jsonrpc: __field0,
                                    result: __field1,
                                    _id: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<BlockResult> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_jsonrpc")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("result")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<BlockResult>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_id")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_jsonrpc") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("result") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(GetBlockResponse {
                                    _jsonrpc: __field0,
                                    result: __field1,
                                    _id: __field2,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["_jsonrpc", "result", "_id"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetBlockResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<GetBlockResponse>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl From<GetBlockResponse> for MentatResponse<BlockResponse> {
                fn from(response: GetBlockResponse) -> Self {
                    Ok(Json(BlockResponse {
                        block: Some(Block {
                            block_identifier: BlockIdentifier {
                                index: response.result.header.metadata.height,
                                hash: response.result.block_hash,
                            },
                            parent_block_identifier: BlockIdentifier {
                                index: response.result.header.metadata.height.saturating_sub(1),
                                hash: response.result.previous_block_hash,
                            },
                            timestamp: response.result.header.metadata.timestamp,
                            transactions: response.result.transactions.into(),
                            metadata: response.result.header.metadata.into(),
                        }),
                        other_transactions: None,
                    }))
                }
            }
        }
        pub use getblock::*;
        mod getmemorypool {
            use mentat::{
                api::MentatResponse, identifiers::TransactionIdentifier,
                responses::MempoolResponse, Json,
            };
            use super::*;
            #[serde(crate = "mentat::serde")]
            struct Transition {
                _ciphertexts: Vec<String>,
                _ciphertext_ids: Vec<String>,
                _commitments: Vec<String>,
                _proof: String,
                _serial_numbers: Vec<String>,
                _transition_id: String,
                _value_balance: i32,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Transition {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Transition {
                            _ciphertexts: ref __self_0_0,
                            _ciphertext_ids: ref __self_0_1,
                            _commitments: ref __self_0_2,
                            _proof: ref __self_0_3,
                            _serial_numbers: ref __self_0_4,
                            _transition_id: ref __self_0_5,
                            _value_balance: ref __self_0_6,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "Transition");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_ciphertexts",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_ciphertext_ids",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_commitments",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_proof",
                                &&(*__self_0_3),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_serial_numbers",
                                &&(*__self_0_4),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_transition_id",
                                &&(*__self_0_5),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_value_balance",
                                &&(*__self_0_6),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for Transition {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    5u64 => _serde::__private::Ok(__Field::__field5),
                                    6u64 => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_ciphertexts" => _serde::__private::Ok(__Field::__field0),
                                    "_ciphertext_ids" => _serde::__private::Ok(__Field::__field1),
                                    "_commitments" => _serde::__private::Ok(__Field::__field2),
                                    "_proof" => _serde::__private::Ok(__Field::__field3),
                                    "_serial_numbers" => _serde::__private::Ok(__Field::__field4),
                                    "_transition_id" => _serde::__private::Ok(__Field::__field5),
                                    "_value_balance" => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_ciphertexts" => _serde::__private::Ok(__Field::__field0),
                                    b"_ciphertext_ids" => _serde::__private::Ok(__Field::__field1),
                                    b"_commitments" => _serde::__private::Ok(__Field::__field2),
                                    b"_proof" => _serde::__private::Ok(__Field::__field3),
                                    b"_serial_numbers" => _serde::__private::Ok(__Field::__field4),
                                    b"_transition_id" => _serde::__private::Ok(__Field::__field5),
                                    b"_value_balance" => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Transition>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Transition;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Transition",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<Vec<String>>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct Transition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<Vec<String>>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct Transition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field2 =
                                    match match _serde::de::SeqAccess::next_element::<Vec<String>>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct Transition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field3 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct Transition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field4 =
                                    match match _serde::de::SeqAccess::next_element::<Vec<String>>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct Transition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field5 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    5usize,
                                                    &"struct Transition with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field6 = match match _serde::de::SeqAccess::next_element::<i32>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                6usize,
                                                &"struct Transition with 7 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(Transition {
                                    _ciphertexts: __field0,
                                    _ciphertext_ids: __field1,
                                    _commitments: __field2,
                                    _proof: __field3,
                                    _serial_numbers: __field4,
                                    _transition_id: __field5,
                                    _value_balance: __field6,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Vec<String>> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<Vec<String>> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<Vec<String>> =
                                    _serde::__private::None;
                                let mut __field3: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field4: _serde::__private::Option<Vec<String>> =
                                    _serde::__private::None;
                                let mut __field5: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field6: _serde::__private::Option<i32> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_ciphertexts")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Vec<String>>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_ciphertext_ids")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Vec<String>>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_commitments")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Vec<String>>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_proof")) ;
                                            }
                                            __field3 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::__private::Option::is_some(&__field4) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_serial_numbers")) ;
                                            }
                                            __field4 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Vec<String>>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field5 => {
                                            if _serde::__private::Option::is_some(&__field5) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_transition_id")) ;
                                            }
                                            __field5 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field6 => {
                                            if _serde::__private::Option::is_some(&__field6) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_value_balance")) ;
                                            }
                                            __field6 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<i32>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_ciphertexts") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "_ciphertext_ids",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_commitments") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_proof") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::__private::Some(__field4) => __field4,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "_serial_numbers",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field5 = match __field5 {
                                    _serde::__private::Some(__field5) => __field5,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_transition_id")
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field6 = match __field6 {
                                    _serde::__private::Some(__field6) => __field6,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_value_balance")
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(Transition {
                                    _ciphertexts: __field0,
                                    _ciphertext_ids: __field1,
                                    _commitments: __field2,
                                    _proof: __field3,
                                    _serial_numbers: __field4,
                                    _transition_id: __field5,
                                    _value_balance: __field6,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "_ciphertexts",
                            "_ciphertext_ids",
                            "_commitments",
                            "_proof",
                            "_serial_numbers",
                            "_transition_id",
                            "_value_balance",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Transition",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Transition>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[serde(crate = "mentat::serde")]
            struct GetMemoryPoolResult {
                _inner_circuit_id: String,
                _ledger_root: String,
                transaction_id: String,
                _transitions: Vec<Transition>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for GetMemoryPoolResult {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        GetMemoryPoolResult {
                            _inner_circuit_id: ref __self_0_0,
                            _ledger_root: ref __self_0_1,
                            transaction_id: ref __self_0_2,
                            _transitions: ref __self_0_3,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "GetMemoryPoolResult");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_inner_circuit_id",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_ledger_root",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "transaction_id",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_transitions",
                                &&(*__self_0_3),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for GetMemoryPoolResult {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_inner_circuit_id" => _serde::__private::Ok(__Field::__field0),
                                    "_ledger_root" => _serde::__private::Ok(__Field::__field1),
                                    "transaction_id" => _serde::__private::Ok(__Field::__field2),
                                    "_transitions" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_inner_circuit_id" => {
                                        _serde::__private::Ok(__Field::__field0)
                                    }
                                    b"_ledger_root" => _serde::__private::Ok(__Field::__field1),
                                    b"transaction_id" => _serde::__private::Ok(__Field::__field2),
                                    b"_transitions" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetMemoryPoolResult>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetMemoryPoolResult;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetMemoryPoolResult",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct GetMemoryPoolResult with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct GetMemoryPoolResult with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field2 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct GetMemoryPoolResult with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field3 = match match _serde::de::SeqAccess::next_element::<
                                    Vec<Transition>,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                3usize,
                                                &"struct GetMemoryPoolResult with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetMemoryPoolResult {
                                    _inner_circuit_id: __field0,
                                    _ledger_root: __field1,
                                    transaction_id: __field2,
                                    _transitions: __field3,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field3: _serde::__private::Option<Vec<Transition>> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_inner_circuit_id")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_ledger_root")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("transaction_id")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_transitions")) ;
                                            }
                                            __field3 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    Vec<Transition>,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "_inner_circuit_id",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_ledger_root") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("transaction_id")
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_transitions") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(GetMemoryPoolResult {
                                    _inner_circuit_id: __field0,
                                    _ledger_root: __field1,
                                    transaction_id: __field2,
                                    _transitions: __field3,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "_inner_circuit_id",
                            "_ledger_root",
                            "transaction_id",
                            "_transitions",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetMemoryPoolResult",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<GetMemoryPoolResult>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl From<GetMemoryPoolResult> for TransactionIdentifier {
                fn from(result: GetMemoryPoolResult) -> TransactionIdentifier {
                    TransactionIdentifier {
                        hash: result.transaction_id,
                    }
                }
            }
            #[serde(crate = "mentat::serde")]
            pub struct GetMemoryPoolResponse {
                _jsonrpc: String,
                result: Vec<GetMemoryPoolResult>,
                _id: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for GetMemoryPoolResponse {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        GetMemoryPoolResponse {
                            _jsonrpc: ref __self_0_0,
                            result: ref __self_0_1,
                            _id: ref __self_0_2,
                        } => {
                            let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(
                                f,
                                "GetMemoryPoolResponse",
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_jsonrpc",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "result",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_id",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for GetMemoryPoolResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    "result" => _serde::__private::Ok(__Field::__field1),
                                    "_id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    b"result" => _serde::__private::Ok(__Field::__field1),
                                    b"_id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetMemoryPoolResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetMemoryPoolResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetMemoryPoolResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct GetMemoryPoolResponse with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<
                                    Vec<GetMemoryPoolResult>,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct GetMemoryPoolResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct GetMemoryPoolResponse with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(GetMemoryPoolResponse {
                                    _jsonrpc: __field0,
                                    result: __field1,
                                    _id: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    Vec<GetMemoryPoolResult>,
                                > = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_jsonrpc")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("result")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    Vec<GetMemoryPoolResult>,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_id")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_jsonrpc") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("result") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(GetMemoryPoolResponse {
                                    _jsonrpc: __field0,
                                    result: __field1,
                                    _id: __field2,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["_jsonrpc", "result", "_id"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetMemoryPoolResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<GetMemoryPoolResponse>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl From<GetMemoryPoolResponse> for MentatResponse<MempoolResponse> {
                fn from(response: GetMemoryPoolResponse) -> MentatResponse<MempoolResponse> {
                    Ok(Json(MempoolResponse {
                        transaction_identifiers: response
                            .result
                            .into_iter()
                            .map(|r| r.into())
                            .collect(),
                    }))
                }
            }
        }
        pub use getmemorypool::*;
        mod gettransaction {
            use std::ops::Add;
            use mentat::{api::MentatResponse, responses::BlockTransactionResponse, Json};
            use super::*;
            use crate::responses::common::SnarkosTransaction;
            #[serde(crate = "mentat::serde")]
            struct DecryptedRecords {
                _commitment: String,
                _owner: String,
                _payload: String,
                _program_id: String,
                _randomizer: String,
                _record_view_key: String,
                _value: i64,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for DecryptedRecords {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        DecryptedRecords {
                            _commitment: ref __self_0_0,
                            _owner: ref __self_0_1,
                            _payload: ref __self_0_2,
                            _program_id: ref __self_0_3,
                            _randomizer: ref __self_0_4,
                            _record_view_key: ref __self_0_5,
                            _value: ref __self_0_6,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "DecryptedRecords");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_commitment",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_owner",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_payload",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_program_id",
                                &&(*__self_0_3),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_randomizer",
                                &&(*__self_0_4),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_record_view_key",
                                &&(*__self_0_5),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_value",
                                &&(*__self_0_6),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for DecryptedRecords {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    5u64 => _serde::__private::Ok(__Field::__field5),
                                    6u64 => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_commitment" => _serde::__private::Ok(__Field::__field0),
                                    "_owner" => _serde::__private::Ok(__Field::__field1),
                                    "_payload" => _serde::__private::Ok(__Field::__field2),
                                    "_program_id" => _serde::__private::Ok(__Field::__field3),
                                    "_randomizer" => _serde::__private::Ok(__Field::__field4),
                                    "_record_view_key" => _serde::__private::Ok(__Field::__field5),
                                    "_value" => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_commitment" => _serde::__private::Ok(__Field::__field0),
                                    b"_owner" => _serde::__private::Ok(__Field::__field1),
                                    b"_payload" => _serde::__private::Ok(__Field::__field2),
                                    b"_program_id" => _serde::__private::Ok(__Field::__field3),
                                    b"_randomizer" => _serde::__private::Ok(__Field::__field4),
                                    b"_record_view_key" => _serde::__private::Ok(__Field::__field5),
                                    b"_value" => _serde::__private::Ok(__Field::__field6),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<DecryptedRecords>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = DecryptedRecords;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct DecryptedRecords",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct DecryptedRecords with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct DecryptedRecords with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field2 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct DecryptedRecords with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field3 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct DecryptedRecords with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field4 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct DecryptedRecords with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field5 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    5usize,
                                                    &"struct DecryptedRecords with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field6 = match match _serde::de::SeqAccess::next_element::<i64>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                6usize,
                                                &"struct DecryptedRecords with 7 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(DecryptedRecords {
                                    _commitment: __field0,
                                    _owner: __field1,
                                    _payload: __field2,
                                    _program_id: __field3,
                                    _randomizer: __field4,
                                    _record_view_key: __field5,
                                    _value: __field6,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field3: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field4: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field5: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field6: _serde::__private::Option<i64> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_commitment")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_owner")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_payload")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_program_id")) ;
                                            }
                                            __field3 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::__private::Option::is_some(&__field4) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_randomizer")) ;
                                            }
                                            __field4 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field5 => {
                                            if _serde::__private::Option::is_some(&__field5) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_record_view_key")) ;
                                            }
                                            __field5 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field6 => {
                                            if _serde::__private::Option::is_some(&__field6) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_value")) ;
                                            }
                                            __field6 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<i64>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_commitment") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_owner") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_payload") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_program_id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::__private::Some(__field4) => __field4,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_randomizer") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field5 = match __field5 {
                                    _serde::__private::Some(__field5) => __field5,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "_record_view_key",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field6 = match __field6 {
                                    _serde::__private::Some(__field6) => __field6,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_value") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(DecryptedRecords {
                                    _commitment: __field0,
                                    _owner: __field1,
                                    _payload: __field2,
                                    _program_id: __field3,
                                    _randomizer: __field4,
                                    _record_view_key: __field5,
                                    _value: __field6,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "_commitment",
                            "_owner",
                            "_payload",
                            "_program_id",
                            "_randomizer",
                            "_record_view_key",
                            "_value",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "DecryptedRecords",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<DecryptedRecords>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[serde(crate = "mentat::serde")]
            struct Metadata {
                _block_hash: String,
                _block_height: u64,
                _block_timestamp: u64,
                transaction_index: usize,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Metadata {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Metadata {
                            _block_hash: ref __self_0_0,
                            _block_height: ref __self_0_1,
                            _block_timestamp: ref __self_0_2,
                            transaction_index: ref __self_0_3,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "Metadata");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_block_hash",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_block_height",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_block_timestamp",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "transaction_index",
                                &&(*__self_0_3),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for Metadata {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_block_hash" => _serde::__private::Ok(__Field::__field0),
                                    "_block_height" => _serde::__private::Ok(__Field::__field1),
                                    "_block_timestamp" => _serde::__private::Ok(__Field::__field2),
                                    "transaction_index" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_block_hash" => _serde::__private::Ok(__Field::__field0),
                                    b"_block_height" => _serde::__private::Ok(__Field::__field1),
                                    b"_block_timestamp" => _serde::__private::Ok(__Field::__field2),
                                    b"transaction_index" => {
                                        _serde::__private::Ok(__Field::__field3)
                                    }
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Metadata>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Metadata;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Metadata",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct Metadata with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<u64>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct Metadata with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match match _serde::de::SeqAccess::next_element::<u64>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct Metadata with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field3 =
                                    match match _serde::de::SeqAccess::next_element::<usize>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct Metadata with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                _serde::__private::Ok(Metadata {
                                    _block_hash: __field0,
                                    _block_height: __field1,
                                    _block_timestamp: __field2,
                                    transaction_index: __field3,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<u64> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<u64> =
                                    _serde::__private::None;
                                let mut __field3: _serde::__private::Option<usize> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_block_hash")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_block_height")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<u64>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_block_timestamp")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<u64>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("transaction_index")) ;
                                            }
                                            __field3 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<usize>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_block_hash") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_block_height")
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "_block_timestamp",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "transaction_index",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(Metadata {
                                    _block_hash: __field0,
                                    _block_height: __field1,
                                    _block_timestamp: __field2,
                                    transaction_index: __field3,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "_block_hash",
                            "_block_height",
                            "_block_timestamp",
                            "transaction_index",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Metadata",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Metadata>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[serde(crate = "mentat::serde")]
            struct GetTransactionResult {
                _decrypted_records: Vec<DecryptedRecords>,
                metadata: Metadata,
                _transaction: SnarkosTransaction,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for GetTransactionResult {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        GetTransactionResult {
                            _decrypted_records: ref __self_0_0,
                            metadata: ref __self_0_1,
                            _transaction: ref __self_0_2,
                        } => {
                            let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(
                                f,
                                "GetTransactionResult",
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_decrypted_records",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "metadata",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_transaction",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for GetTransactionResult {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_decrypted_records" => {
                                        _serde::__private::Ok(__Field::__field0)
                                    }
                                    "metadata" => _serde::__private::Ok(__Field::__field1),
                                    "_transaction" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_decrypted_records" => {
                                        _serde::__private::Ok(__Field::__field0)
                                    }
                                    b"metadata" => _serde::__private::Ok(__Field::__field1),
                                    b"_transaction" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetTransactionResult>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetTransactionResult;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetTransactionResult",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match match _serde::de::SeqAccess::next_element::<
                                    Vec<DecryptedRecords>,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetTransactionResult with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 =
                                    match match _serde::de::SeqAccess::next_element::<Metadata>(
                                        &mut __seq,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct GetTransactionResult with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field2 = match match _serde::de::SeqAccess::next_element::<
                                    SnarkosTransaction,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct GetTransactionResult with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetTransactionResult {
                                    _decrypted_records: __field0,
                                    metadata: __field1,
                                    _transaction: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Vec<DecryptedRecords>> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<Metadata> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<SnarkosTransaction> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_decrypted_records")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    Vec<DecryptedRecords>,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("metadata")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<Metadata>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_transaction")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    SnarkosTransaction,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field(
                                            "_decrypted_records",
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("metadata") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_transaction") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(GetTransactionResult {
                                    _decrypted_records: __field0,
                                    metadata: __field1,
                                    _transaction: __field2,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] =
                            &["_decrypted_records", "metadata", "_transaction"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetTransactionResult",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<GetTransactionResult>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[serde(crate = "mentat::serde")]
            pub struct GetTransactionResponse {
                _jsonrpc: String,
                result: GetTransactionResult,
                _id: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for GetTransactionResponse {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        GetTransactionResponse {
                            _jsonrpc: ref __self_0_0,
                            result: ref __self_0_1,
                            _id: ref __self_0_2,
                        } => {
                            let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(
                                f,
                                "GetTransactionResponse",
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_jsonrpc",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "result",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_id",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for GetTransactionResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "_jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    "result" => _serde::__private::Ok(__Field::__field1),
                                    "_id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"_jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    b"result" => _serde::__private::Ok(__Field::__field1),
                                    b"_id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetTransactionResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetTransactionResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetTransactionResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetTransactionResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<
                                    GetTransactionResult,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct GetTransactionResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct GetTransactionResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetTransactionResponse {
                                    _jsonrpc: __field0,
                                    result: __field1,
                                    _id: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<GetTransactionResult> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_jsonrpc")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("result")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    GetTransactionResult,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("_id")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_jsonrpc") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("result") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("_id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(GetTransactionResponse {
                                    _jsonrpc: __field0,
                                    result: __field1,
                                    _id: __field2,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["_jsonrpc", "result", "_id"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetTransactionResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<GetTransactionResponse>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl Add<GetBlockTransactionsResponse> for GetTransactionResponse {
                type Output = MentatResponse<BlockTransactionResponse>;
                fn add(self, other: GetBlockTransactionsResponse) -> Self::Output {
                    let transaction = other
                        .result
                        .transactions
                        .get(self.result.metadata.transaction_index)
                        .cloned()
                        .unwrap();
                    Ok(Json(BlockTransactionResponse {
                        transaction: transaction.into(),
                    }))
                }
            }
        }
        pub use gettransaction::*;
        mod getblocktransactions {
            use super::*;
            use crate::responses::common::SnarkosTransactions;
            #[serde(crate = "mentat::serde")]
            pub struct GetBlockTransactionsResponse {
                pub jsonrpc: String,
                pub result: SnarkosTransactions,
                pub id: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for GetBlockTransactionsResponse {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        GetBlockTransactionsResponse {
                            jsonrpc: ref __self_0_0,
                            result: ref __self_0_1,
                            id: ref __self_0_2,
                        } => {
                            let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(
                                f,
                                "GetBlockTransactionsResponse",
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "jsonrpc",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "result",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "id",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                use mentat::serde as _serde;
                #[automatically_derived]
                impl<'de> mentat::serde::Deserialize<'de> for GetBlockTransactionsResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> mentat::serde::__private::Result<Self, __D::Error>
                    where
                        __D: mentat::serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    "result" => _serde::__private::Ok(__Field::__field1),
                                    "id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                    b"result" => _serde::__private::Ok(__Field::__field1),
                                    b"id" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetBlockTransactionsResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetBlockTransactionsResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetBlockTransactionsResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "struct GetBlockTransactionsResponse with 3 elements")) ;
                                    }
                                };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<
                                    SnarkosTransactions,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "struct GetBlockTransactionsResponse with 3 elements")) ;
                                    }
                                };
                                let __field2 = match match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (2usize , & "struct GetBlockTransactionsResponse with 3 elements")) ;
                                    }
                                };
                                _serde::__private::Ok(GetBlockTransactionsResponse {
                                    jsonrpc: __field0,
                                    result: __field1,
                                    id: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<SnarkosTransactions> =
                                    _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("jsonrpc")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("result")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    SnarkosTransactions,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("id")) ;
                                            }
                                            __field2 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("jsonrpc") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("result") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(GetBlockTransactionsResponse {
                                    jsonrpc: __field0,
                                    result: __field1,
                                    id: __field2,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["jsonrpc", "result", "id"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetBlockTransactionsResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    GetBlockTransactionsResponse,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
        }
        pub use getblocktransactions::*;
    }
    mod error {
        use mentat::{
            api::MentatResponse,
            errors::{ApiError, MentatError},
            serde::Deserialize,
            serde_json::Value,
            IndexMap,
        };
        #[serde(crate = "mentat::serde")]
        pub struct ErrorResponse {
            pub jsonrpc: String,
            pub error: IndexMap<String, Value>,
            pub id: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ErrorResponse {
            #[inline]
            fn clone(&self) -> ErrorResponse {
                match *self {
                    ErrorResponse {
                        jsonrpc: ref __self_0_0,
                        error: ref __self_0_1,
                        id: ref __self_0_2,
                    } => ErrorResponse {
                        jsonrpc: ::core::clone::Clone::clone(&(*__self_0_0)),
                        error: ::core::clone::Clone::clone(&(*__self_0_1)),
                        id: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ErrorResponse {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    ErrorResponse {
                        jsonrpc: ref __self_0_0,
                        error: ref __self_0_1,
                        id: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "ErrorResponse");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "jsonrpc",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "error",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "id",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            use mentat::serde as _serde;
            #[automatically_derived]
            impl<'de> mentat::serde::Deserialize<'de> for ErrorResponse {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> mentat::serde::__private::Result<Self, __D::Error>
                where
                    __D: mentat::serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                "error" => _serde::__private::Ok(__Field::__field1),
                                "id" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"jsonrpc" => _serde::__private::Ok(__Field::__field0),
                                b"error" => _serde::__private::Ok(__Field::__field1),
                                b"id" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ErrorResponse>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ErrorResponse;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ErrorResponse",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ErrorResponse with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                IndexMap<String, Value>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ErrorResponse with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ErrorResponse with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ErrorResponse {
                                jsonrpc: __field0,
                                error: __field1,
                                id: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<IndexMap<String, Value>> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "jsonrpc",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "error",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                IndexMap<String, Value>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("jsonrpc") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("error") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(ErrorResponse {
                                jsonrpc: __field0,
                                error: __field1,
                                id: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["jsonrpc", "error", "id"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ErrorResponse",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ErrorResponse>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl<R> From<ErrorResponse> for MentatResponse<R> {
            fn from(response: ErrorResponse) -> Self {
                Err(MentatError::Internal(ApiError {
                    code: 500,
                    message: "Snarkos JsonRPC Error.".to_string(),
                    description: None,
                    retriable: true,
                    details: response.error,
                }))
            }
        }
    }
    pub use error::*;
    mod response {
        use mentat::serde::Deserialize;
        use super::ErrorResponse;
        #[serde(crate = "mentat::serde")]
        #[serde(untagged)]
        pub enum Response<R> {
            Ok(R),
            Err(ErrorResponse),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<R: ::core::clone::Clone> ::core::clone::Clone for Response<R> {
            #[inline]
            fn clone(&self) -> Response<R> {
                match (&*self,) {
                    (&Response::Ok(ref __self_0),) => {
                        Response::Ok(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&Response::Err(ref __self_0),) => {
                        Response::Err(::core::clone::Clone::clone(&(*__self_0)))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<R: ::core::fmt::Debug> ::core::fmt::Debug for Response<R> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&Response::Ok(ref __self_0),) => {
                        let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Ok");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&Response::Err(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Err");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            use mentat::serde as _serde;
            #[automatically_derived]
            impl<'de, R> mentat::serde::Deserialize<'de> for Response<R>
            where
                R: _serde::Deserialize<'de>,
            {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> mentat::serde::__private::Result<Self, __D::Error>
                where
                    __D: mentat::serde::Deserializer<'de>,
                {
                    let __content =
                        match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                        <R as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                                &__content,
                            ),
                        ),
                        Response::Ok,
                    ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                        <ErrorResponse as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                                &__content,
                            ),
                        ),
                        Response::Err,
                    ) {
                        return _serde::__private::Ok(__ok);
                    }
                    _serde::__private::Err(_serde::de::Error::custom(
                        "data did not match any variant of untagged enum Response",
                    ))
                }
            }
        };
    }
    pub use response::*;
}
use request::SnarkosJrpc;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        let mut server = Server::new(String::from("SNARKOS"));
        server.with_dyn_call_api(Arc::new(call_api::SnarkosCallApi::default()));
        server.with_dyn_construction_api(Arc::new(
            construction_api::SnarkosConstructionApi::default(),
        ));
        server.with_dyn_data_api(Arc::new(data_api::SnarkosDataApi::default()));
        server.with_dyn_indexer_api(Arc::new(indexer_api::SnarkosIndexerApi::default()));
        let address = env::var("ADDRESS")
            .unwrap_or_else(|_| "0.0.0.0".to_string())
            .parse()
            .unwrap_or(Ipv4Addr::new(0, 0, 0, 0));
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);
        {
            use ::mentat::server::serve_exports::*;
            let app = {
                let mut app = Router::new();
                async fn call_call(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<CallRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<CallResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_call",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . call_api . call_call (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/") , :: core :: fmt :: ArgumentV1 :: new_display (& "/call") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/") , :: core :: fmt :: ArgumentV1 :: new_display (& "/call") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/"),
                                ::core::fmt::ArgumentV1::new_display(&"/call"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_call),
                );
                async fn call_combine(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<ConstructionCombineRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<ConstructionCombineResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_combine",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . construction_api . call_combine (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/combine") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/combine") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/construction"),
                                ::core::fmt::ArgumentV1::new_display(&"/combine"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_combine),
                );
                async fn call_derive(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<ConstructionDeriveRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<ConstructionDeriveResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_derive",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . construction_api . call_derive (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/derive") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/derive") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/construction"),
                                ::core::fmt::ArgumentV1::new_display(&"/derive"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_derive),
                );
                async fn call_hash(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<ConstructionHashRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<TransactionIdentifierResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_hash",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . construction_api . call_hash (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/hash") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/hash") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/construction"),
                                ::core::fmt::ArgumentV1::new_display(&"/hash"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_hash),
                );
                async fn call_metadata(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<ConstructionMetadataRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<ConstructionMetadataResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_metadata",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . construction_api . call_metadata (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/metadata") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/metadata") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/construction"),
                                ::core::fmt::ArgumentV1::new_display(&"/metadata"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_metadata),
                );
                async fn call_parse(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<ConstructionParseRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<ConstructionParseResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_parse",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . construction_api . call_parse (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/parse") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/parse") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/construction"),
                                ::core::fmt::ArgumentV1::new_display(&"/parse"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_parse),
                );
                async fn call_payloads(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<ConstructionPayloadsRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<ConstructionPayloadsResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_payloads",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . construction_api . call_payloads (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/payloads") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/payloads") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/construction"),
                                ::core::fmt::ArgumentV1::new_display(&"/payloads"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_payloads),
                );
                async fn call_preprocess(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<ConstructionPreprocessRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<ConstructionPreprocessResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_preprocess",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . construction_api . call_preprocess (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/preprocess") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/preprocess") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/construction"),
                                ::core::fmt::ArgumentV1::new_display(&"/preprocess"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_preprocess),
                );
                async fn call_submit(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<ConstructionSubmitRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<TransactionIdentifierResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_submit",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . construction_api . call_submit (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/submit") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/construction") , :: core :: fmt :: ArgumentV1 :: new_display (& "/submit") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/construction"),
                                ::core::fmt::ArgumentV1::new_display(&"/submit"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_submit),
                );
                async fn call_network_list(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<MetadataRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<NetworkListResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_network_list",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . data_api . call_network_list (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/network") , :: core :: fmt :: ArgumentV1 :: new_display (& "/list") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/network") , :: core :: fmt :: ArgumentV1 :: new_display (& "/list") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/network"),
                                ::core::fmt::ArgumentV1::new_display(&"/list"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_network_list),
                );
                async fn call_network_options(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<NetworkRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<NetworkOptionsResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_network_options",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . data_api . call_network_options (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/network") , :: core :: fmt :: ArgumentV1 :: new_display (& "/options") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/network") , :: core :: fmt :: ArgumentV1 :: new_display (& "/options") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/network"),
                                ::core::fmt::ArgumentV1::new_display(&"/options"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_network_options),
                );
                async fn call_network_status(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<NetworkRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<NetworkStatusResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_network_status",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . data_api . call_network_status (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/network") , :: core :: fmt :: ArgumentV1 :: new_display (& "/status") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/network") , :: core :: fmt :: ArgumentV1 :: new_display (& "/status") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/network"),
                                ::core::fmt::ArgumentV1::new_display(&"/status"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_network_status),
                );
                async fn call_account_balance(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<AccountBalanceRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<AccountBalanceResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_account_balance",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . data_api . call_account_balance (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/account") , :: core :: fmt :: ArgumentV1 :: new_display (& "/balance") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/account") , :: core :: fmt :: ArgumentV1 :: new_display (& "/balance") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/account"),
                                ::core::fmt::ArgumentV1::new_display(&"/balance"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_account_balance),
                );
                async fn call_account_coins(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<AccountCoinsRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<AccountCoinsResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_account_coins",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . data_api . call_account_coins (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/account") , :: core :: fmt :: ArgumentV1 :: new_display (& "/coins") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/account") , :: core :: fmt :: ArgumentV1 :: new_display (& "/coins") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/account"),
                                ::core::fmt::ArgumentV1::new_display(&"/coins"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_account_coins),
                );
                async fn call_block(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<BlockRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<BlockResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_block",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . data_api . call_block (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/block") , :: core :: fmt :: ArgumentV1 :: new_display (& "/") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/block") , :: core :: fmt :: ArgumentV1 :: new_display (& "/") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/block"),
                                ::core::fmt::ArgumentV1::new_display(&"/"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_block),
                );
                async fn call_block_transaction(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<BlockTransactionRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<BlockTransactionResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_block_transaction",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . data_api . call_block_transaction (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/block") , :: core :: fmt :: ArgumentV1 :: new_display (& "/transaction") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/block") , :: core :: fmt :: ArgumentV1 :: new_display (& "/transaction") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/block"),
                                ::core::fmt::ArgumentV1::new_display(&"/transaction"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_block_transaction),
                );
                async fn call_mempool(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<NetworkRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<MempoolResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_mempool",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . data_api . call_mempool (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/mempool") , :: core :: fmt :: ArgumentV1 :: new_display (& "/") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/mempool") , :: core :: fmt :: ArgumentV1 :: new_display (& "/") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/mempool"),
                                ::core::fmt::ArgumentV1::new_display(&"/"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_mempool),
                );
                async fn call_mempool_transaction(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<MempoolTransactionRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<MempoolTransactionResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_mempool_transaction",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . data_api . call_mempool_transaction (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/mempool") , :: core :: fmt :: ArgumentV1 :: new_display (& "/transaction") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/mempool") , :: core :: fmt :: ArgumentV1 :: new_display (& "/transaction") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/mempool"),
                                ::core::fmt::ArgumentV1::new_display(&"/transaction"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_mempool_transaction),
                );
                async fn call_events_blocks(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<EventsBlocksRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<EventsBlocksResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_events_blocks",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . indexer_api . call_events_blocks (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/events") , :: core :: fmt :: ArgumentV1 :: new_display (& "/blocks") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/events") , :: core :: fmt :: ArgumentV1 :: new_display (& "/blocks") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/events"),
                                ::core::fmt::ArgumentV1::new_display(&"/blocks"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_events_blocks),
                );
                async fn call_search_transactions(
                    Extension(Configuration): Extension<Configuration>,
                    ConnectInfo(ip): ConnectInfo<SocketAddr>,
                    extract::Json(req_data): Json<SearchTransactionsRequest>,
                    Extension(mode): ModeState,
                    Extension(client): Extension<Client>,
                ) -> MentatResponse<SearchTransactionsResponse> {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "call_search_transactions",
                                    "rosetta_snarkos",
                                    tracing::Level::INFO,
                                    Some("rosetta-snarkos\\src\\main.rs"),
                                    Some(33u32),
                                    Some("rosetta_snarkos"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["Configuration", "ip", "req_data", "mode"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&Configuration) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&ip) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&req_data) as &Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&tracing::field::debug(&mode) as &Value),
                                    ),
                                ])
                            })
                        } else {
                            let span = CALLSITE.disabled_span();
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(&{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = CALLSITE.metadata().fields().iter();
                                            CALLSITE.metadata().fields().value_set(&[
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&Configuration)
                                                        as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&ip) as &Value),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(
                                                        &tracing::field::debug(&req_data) as &Value
                                                    ),
                                                ),
                                                (
                                                    &iter.next().expect(
                                                        "FieldSet corrupted (this is a bug)",
                                                    ),
                                                    Some(&tracing::field::debug(&mode) as &Value),
                                                ),
                                            ])
                                        });
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        {
                            let c = Caller { ip };
                            Cache :: < DefaultCacheInner < _ > > :: new (Default :: default () , None) . get_cached (move | | { Box :: pin (async move { let resp = server . indexer_api . call_search_transactions (c , req_data , & mode , client) . await ; { use :: tracing :: __macro_support :: Callsite as _ ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event rosetta-snarkos\\src\\main.rs:33" , "rosetta_snarkos" , :: tracing :: Level :: DEBUG , Some ("rosetta-snarkos\\src\\main.rs") , Some (33u32) , Some ("rosetta_snarkos") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let enabled = :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: DEBUG <= :: tracing :: level_filters :: LevelFilter :: current () && { let interest = CALLSITE . interest () ; ! interest . is_never () && CALLSITE . is_enabled (interest) } ; if enabled { (| value_set : :: tracing :: field :: ValueSet | { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & value_set) ; if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& value_set))])) . build ()) ; } } } } else { { } } } else { { } } ; }) ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/search") , :: core :: fmt :: ArgumentV1 :: new_display (& "/transactions") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }) ; } else { if match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: DEBUG { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("rosetta_snarkos") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("rosetta-snarkos\\src\\main.rs")) . module_path (Some ("rosetta_snarkos")) . line (Some (33u32)) . metadata (log_meta) . args (:: core :: fmt :: Arguments :: new_v1 (& [""] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: tracing :: __macro_support :: LogValueSet (& { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = CALLSITE . metadata () . fields () . iter () ; CALLSITE . metadata () . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["response " , "" , " "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "/search") , :: core :: fmt :: ArgumentV1 :: new_display (& "/transactions") , :: core :: fmt :: ArgumentV1 :: new_debug (& resp)]) as & Value))]) }))])) . build ()) ; } } } } else { { } } } else { { } } ; } } ; resp }) }) . await
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
                app = app.route(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&"/search"),
                                ::core::fmt::ArgumentV1::new_display(&"/transactions"),
                            ],
                        ));
                        res
                    },
                    routing::post(call_search_transactions),
                );
                app
            };
            server
                .serve(app, address, port, node::SnarkOSNode::default().borrow())
                .await
        }
    };
    #[allow(clippy::expect_used)]
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}
