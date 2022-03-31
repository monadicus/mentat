extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    self, parse_macro_input, AttributeArgs, Ident, ItemFn, ItemStruct, Meta, NestedMeta,
};

#[proc_macro_attribute]
pub fn mentat(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    format!(
        "\
use ::mentat::server::serve_exports::*;

#[derive(Clone)]
{}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let app = serve!(@build {}, {});
    <{}>::build_server().serve(app).await
}}",
        input.to_token_stream(),
        input.ident,
        attr,
        input.ident,
    )
    .parse()
    .unwrap()
}

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    if input.sig.ident.to_string() != "main" {
        panic!("expected function name `main` found `{}`", input.sig.ident);
    } else {
        input.sig.ident = Ident::new("__mentat_main_preamble_fn", Span::call_site());
    }

    let attr_str = attr.to_string();
    let args = parse_macro_input!(attr as AttributeArgs);
    let server_type = if let Some(NestedMeta::Meta(Meta::Path(path))) = args.get(0) {
        path.get_ident().expect("expected ServerType")
    } else {
        panic!("expected ServerType")
    };

    format!(
        "\
use ::mentat::server::serve_exports::*;

{}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    {}().await;
    let app = serve!(@build {});
    <{}>::build_server().serve(app).await
}}",
        input.to_token_stream(),
        input.sig.ident,
        attr_str,
        server_type
    )
    .parse()
    .unwrap()
}
