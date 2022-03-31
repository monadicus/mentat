extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{self, parse_macro_input, AttributeArgs, Ident, ItemFn, ItemStruct, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn mentat(attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_def = parse_macro_input!(item as ItemStruct);

    format!(
        "\
use ::mentat::server::serve_exports::*;

#[::std::prelude::v1::derive(::std::clone::Clone)]
{}

#[::mentat::tokio::main]
async fn main() -> ::std::result::Result<(), ::std::boxed::Box<dyn ::std::error::Error>> {{
    let app = serve!(@build {}, {});
    <{}>::build_server().serve(app).await
}}",
        struct_def.to_token_stream(),
        struct_def.ident,
        attr,
        struct_def.ident,
    )
    .parse()
    .unwrap()
}

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    if function.sig.ident != "main" {
        panic!(
            "expected function name `main` found `{}`",
            function.sig.ident
        );
    } else {
        function.sig.ident = Ident::new("__mentat_main_preamble_fn", Span::call_site());
    }

    let args = parse_macro_input!(attr as AttributeArgs);
    let server_type = match args.get(0) {
        Some(NestedMeta::Meta(Meta::Path(path))) => {
            path.get_ident().expect("expected ServerType").to_string()
        }
        _ => panic!("expected type `ServerType`"),
    };
    let cache_type = match args.get(1) {
        Some(NestedMeta::Meta(Meta::Path(path))) => match path.get_ident() {
            Some(id) => id.to_string(),
            None => panic!("expected type `CacheInner`"),
        },
        Some(_) => panic!("expected type `CacheInner`"),
        None => String::new(),
    };

    format!(
        "\
use ::mentat::server::serve_exports::*;

{}

#[::mentat::tokio::main]
async fn main() -> ::std::result::Result<(), std::boxed::Box<dyn ::std::error::Error>> {{
    {}().await;
    let app = serve!(@build {}, {});
    <{}>::build_server().serve(app).await
}}",
        function.to_token_stream(),
        function.sig.ident,
        server_type,
        cache_type,
        server_type
    )
    .parse()
    .unwrap()
}
