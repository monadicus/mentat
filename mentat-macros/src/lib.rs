extern crate proc_macro;

mod route_builder;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    self,
    parse_macro_input,
    AttributeArgs,
    GenericArgument,
    Ident,
    ItemFn,
    ItemStruct,
    Meta,
    NestedMeta,
    PathArguments::{self},
    PathSegment,
    ReturnType,
    Type,
};

/// parses the provided macro argument for the optional cache type
fn get_cache_inner_type(arg: &NestedMeta) -> &Ident {
    match arg {
        NestedMeta::Meta(Meta::Path(path)) => match path.get_ident() {
            Some(id) => id,
            None => panic!("expected type `CacheInner`"),
        },
        _ => panic!("expected type `CacheInner`"),
    }
}

/// mutates the provided main function's name to `__mentat_main_server_call` so
/// that it can be called from mentat's `main` function
fn swap_main_name(f: &mut ItemFn) -> Result<(), TokenStream> {
    if f.sig.ident == "main" {
        f.sig.ident = Ident::new("__mentat_main_server_call", f.sig.ident.span());
        Ok(())
    } else {
        Err(syn::Error::new(
            f.sig.ident.span(),
            format!("expected function name `main` found `{}`", f.sig.ident),
        )
        .into_compile_error()
        .into())
    }
}

/// matches the provided main function for the `Server`'s `ServerType`
fn get_function_return_server_type(f: &ItemFn) -> Result<&Ident, TokenStream> {
    // TODO this is horribly nested but they dont provide any helper functions to
    // avoid this
    if let ReturnType::Type(_, t) = &f.sig.output {
        if let Type::Path(t) = &**t {
            if let Some(PathSegment {
                arguments: PathArguments::AngleBracketed(t),
                ..
            }) = t.path.segments.first()
            {
                if let Some(GenericArgument::Type(Type::Path(t))) = t.args.first() {
                    if let Some(t) = t.path.get_ident() {
                        return Ok(t);
                    }
                }
            }
        }
    }
    Err(syn::Error::new(
        f.sig.ident.span(),
        "expected function to return `Server<ServerType>` instance",
    )
    .into_compile_error()
    .into())
}

/// generates code to derive Clone and Default on a user supplied `ServerType`
fn gen_derive(server_def: &ItemStruct) -> TokenStream2 {
    quote!(
        #[::std::prelude::v1::derive(::std::clone::Clone, ::std::default::Default)]
        #server_def
    )
}

/// generates the main function for the given mentat implementation
fn gen_main(
    server_call: &TokenStream2,
    server_type: &Ident,
    cache_type: Option<&Ident>,
) -> TokenStream2 {
    let routes = route_builder::build_routes(server_type, cache_type);

    quote!(
        use ::mentat::macro_exports::tokio;
        #[tokio::main]
        async fn main() -> ::std::result::Result<(), ::std::boxed::Box<dyn ::std::error::Error>> {
            use ::mentat::macro_exports::*;
            let server = #server_call;
            let mut app = Router::new();
            #routes
            server.serve(app).await
        }
    )
}

/// a macro for generating mentat routes from a default `Server` instance
#[proc_macro_attribute]
pub fn mentat(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let cache_type = args.get(0).map(get_cache_inner_type);

    let server_def = parse_macro_input!(item as ItemStruct);
    let server_type = &server_def.ident;
    let server_call = quote!(Server::<#server_type>::default());

    let mut out = TokenStream2::new();
    out.extend(gen_derive(&server_def));
    out.extend(gen_main(&server_call, server_type, cache_type));
    out.into()
}

/// a macro for generating mentat routes from a user supplied `Server` instance
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let cache_type = args.get(0).map(get_cache_inner_type);

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
