extern crate proc_macro;

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

// todo: bad! you shouldnt import things in a proc macro
fn gen_imports() -> TokenStream2 {
    quote!(
        use ::mentat::server::serve_exports::*;
    )
}

fn derive_clone(struct_def: &ItemStruct) -> TokenStream2 {
    quote!(
        #[::std::prelude::v1::derive(::std::clone::Clone)]
        #struct_def
    )
}

fn gen_main(
    preamble: Option<&ItemFn>,
    struct_ident: &Ident,
    cacher: Option<&Ident>,
) -> TokenStream2 {
    let preamble_call = preamble.map(|f| {
        let sig = &f.sig.ident;
        quote!(
            #sig().await;
        )
    });

    quote!(
        #preamble

        #[::mentat::tokio::main]
        async fn main() -> ::std::result::Result<(), ::std::boxed::Box<dyn ::std::error::Error>> {
            #preamble_call
            let server = <#struct_ident>::build_server();
            let app = serve!(@build server, #struct_ident, #cacher);
            server.serve(app).await
        }
    )
}

#[proc_macro_attribute]
pub fn mentat(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let cache_type = args.get(0).map(|a| parse_cache_inner_type(a));

    let struct_def = parse_macro_input!(item as ItemStruct);
    let struct_ident = struct_def.ident.clone();

    let mut out = TokenStream2::new();
    out.extend(gen_imports());
    out.extend(derive_clone(&struct_def));
    out.extend(gen_main(None, &struct_ident, cache_type));
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
    let cache_type = args.get(1).map(|a| parse_cache_inner_type(a));

    let mut out = TokenStream2::new();
    out.extend(gen_imports());
    out.extend(gen_main(Some(&function), &server_type, cache_type));
    out.into()
}
