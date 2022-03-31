extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{self, parse_macro_input, Ident, ItemFn, ItemStruct};

#[proc_macro_attribute]
pub fn mentat(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    format!(
        "\
#[derive(Clone)]
{}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    serve!({}, {})
}}",
        input.to_token_stream(),
        input.ident,
        attr,
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

    format!(
        "\
{}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    {}().await;
    serve!({})
}}",
        input.to_token_stream(),
        input.sig.ident,
        attr
    )
    .parse()
    .unwrap()
}
