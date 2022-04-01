extern crate proc_macro;

mod route_builder;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{self, parse_macro_input, AttributeArgs, Ident, ItemFn, ItemStruct, Meta, NestedMeta};

/// parses the provided meta argument for the optional cache type
fn parse_cache_inner_type(arg: &NestedMeta) -> &Ident {
    match arg {
        NestedMeta::Meta(Meta::Path(path)) => match path.get_ident() {
            Some(id) => id,
            None => panic!("expected type `CacheInner`"),
        },
        _ => panic!("expected type `CacheInner`"),
    }
}

fn gen_derive_clone(server_def: &ItemStruct) -> TokenStream2 {
    quote!(
        #[::std::prelude::v1::derive(::std::clone::Clone)]
        #server_def
    )
}

fn gen_main(
    preamble: Option<&ItemFn>,
    server_type: &Ident,
    cacher: Option<&Ident>,
) -> TokenStream2 {
    let pre_main_call = preamble.map(|f| {
        let sig = &f.sig.ident;
        quote!(
            #sig().await;
        )
    });

    let routes = route_builder::build_routes(server_type, cacher);

    quote!(
        use ::mentat::macro_exports::tokio;
        #[::mentat::macro_exports::tokio::main]
        async fn main() -> ::std::result::Result<(), ::std::boxed::Box<dyn ::std::error::Error>> {
            #pre_main_call
            use ::mentat::macro_exports::*;
            let server = <#server_type>::build_server();
            let mut app = Router::new();
            #routes
            server.serve(app).await
        }
    )
}

#[proc_macro_attribute]
pub fn mentat(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let cache_type = args.get(0).map(parse_cache_inner_type);

    let server_def = parse_macro_input!(item as ItemStruct);

    let mut out = TokenStream2::new();
    out.extend(gen_derive_clone(&server_def));
    out.extend(gen_main(None, &server_def.ident, cache_type));
    out.into()
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
        Some(NestedMeta::Meta(Meta::Path(path))) => path.get_ident().expect("expected ServerType"),
        _ => panic!("expected type `ServerType`"),
    };
    let cache_type = args.get(1).map(parse_cache_inner_type);

    let mut out = TokenStream2::new();
    out.extend(quote!(#function));
    out.extend(gen_main(Some(&function), server_type, cache_type));
    out.into()
}
