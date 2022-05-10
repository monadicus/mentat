mod route_builder;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    self, GenericArgument, Ident, ItemFn, ItemStruct, Meta, NestedMeta,
    PathArguments::{self},
    PathSegment, ReturnType, Type,
};

/// Matches the provided macro argument for the optional `CacheInner` type.
pub fn get_cache_inner_type(arg: &NestedMeta) -> Result<&Ident, TokenStream> {
    if let NestedMeta::Meta(Meta::Path(path)) = arg {
        if let Some(id) = path.get_ident() {
            return Ok(id);
        }
    }
    Err(
        syn::Error::new(Span::call_site(), "expected type `CacheInner`")
            .into_compile_error()
            .into(),
    )
}

/// Mutates the provided main function's name to `__mentat_main_server_call` so
/// that it can be called from mentat's `main` function.
pub fn swap_main_name(f: &mut ItemFn) -> Result<(), TokenStream> {
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

/// Matches the provided main function for the `Server`'s `ServerType`.
pub fn get_function_return_server_type(f: &ItemFn) -> Result<&Ident, TokenStream> {
    // TODO this is horribly nested but they don't provide any helper functions to
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

/// Generates code to derive Clone on a user supplied `ServerType`.
pub fn gen_derive(server_def: &ItemStruct) -> TokenStream2 {
    quote!(
        #[::std::prelude::v1::derive(::std::clone::Clone)]
        #server_def
    )
}

/// Generates the main function for the given mentat implementation.
pub fn gen_main(
    server_call: &TokenStream2,
    server_type: &Ident,
    cache_type: Option<&Ident>,
) -> TokenStream2 {
    let routes = route_builder::build_routes(server_type, cache_type);

    quote!(
        use ::mentat::{conf::NodePid, macro_exports::tokio, sysinfo::Pid};
        #[tokio::main]
        async fn main() {
            use ::mentat::macro_exports::*;
            let server = #server_call;
            let mut app = Router::new();
            #routes
            server.serve(app).await
        }
    )
}
