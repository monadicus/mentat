extern crate proc_macro;

mod main_gen;

use main_gen::{get_cache_inner_type, gen_derive, gen_main, swap_main_name, get_function_return_server_type};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, parse_macro_input, AttributeArgs, ItemFn, ItemStruct};

/// A macro for generating mentat routes from a default `Server` instance.
#[proc_macro_attribute]
pub fn mentat(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let cache_type = match args.get(0).map(get_cache_inner_type) {
        Some(Ok(e)) => Some(e),
        Some(Err(e)) => return e,
        _ => None,
    };

    let server_def = parse_macro_input!(item as ItemStruct);
    let server_type = &server_def.ident;
    let server_call = quote!(Server::<#server_type>::default());

    let mut out = TokenStream2::new();
    out.extend(gen_derive(&server_def));
    out.extend(gen_main(&server_call, server_type, cache_type));
    out.into()
}

/// A macro for generating mentat routes from a user supplied `main` function.
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let cache_type = match args.get(0).map(get_cache_inner_type) {
        Some(Ok(e)) => Some(e),
        Some(Err(e)) => return e,
        _ => None,
    };

    let mut function = parse_macro_input!(item as ItemFn);
    if let Err(e) = swap_main_name(&mut function) {
        return e;
    }

    let server_type = match get_function_return_server_type(&function) {
        Ok(t) => t,
        Err(e) => return e,
    };
    let function_name = &function.sig.ident;
    let server_call = quote!(#function_name().await);

    let mut out = TokenStream2::new();
    out.extend(quote!(#function));
    out.extend(gen_main(&server_call, server_type, cache_type));
    out.into()
}
