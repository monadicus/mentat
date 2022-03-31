extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{self, parse_macro_input, ItemStruct};

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
