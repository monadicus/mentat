// TODO temporary
#![allow(clippy::missing_docs_in_private_items)]

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse_quote,
    punctuated::Punctuated,
    token::Brace,
    Field,
    FieldValue,
    Fields,
    FieldsNamed,
    Ident,
    ItemImpl,
    ItemStruct,
    Type,
};

#[derive(Debug, PartialEq, Eq)]
enum FieldBehavior {
    Retain,
    Vec,
    Option,
}

impl From<&Ident> for FieldBehavior {
    fn from(i: &Ident) -> Self {
        if i == "Option" {
            Self::Option
        } else if i == "Vec" {
            Self::Vec
        } else {
            Self::Retain
        }
    }
}

#[derive(Debug)]
struct FieldData {
    field: Field,
    behavior: FieldBehavior,
}

impl FieldData {
    fn gen_from_nullable(&self) -> FieldValue {
        let field_name = &self.field.ident.as_ref().unwrap();
        match &self.behavior {
            FieldBehavior::Retain => parse_quote!(#field_name: other.#field_name.into()),
            FieldBehavior::Vec => {
                parse_quote!(#field_name: other.#field_name.into_iter().flatten().map(|v| v.into()).collect())
            }
            FieldBehavior::Option => parse_quote!(#field_name: other.#field_name.unwrap().into()),
        }
    }

    fn gen_to_nullable(&self) -> FieldValue {
        let field_name = &self.field.ident.as_ref().unwrap();
        match &self.behavior {
            FieldBehavior::Retain => parse_quote!(#field_name: other.#field_name.into()),
            FieldBehavior::Vec => {
                parse_quote!(#field_name: other.#field_name.into_iter().map(|c| Some(c.into())).collect())
            }
            FieldBehavior::Option => parse_quote!(#field_name: Some(other.#field_name.into())),
        }
    }

    fn mutate_type(ty: &Type) -> (Type, FieldBehavior) {
        let mut stream = ty
            .clone()
            .into_token_stream()
            .into_iter()
            .map(|t| {
                if let Some(id) = t.to_string().strip_prefix("Nullable") {
                    TokenTree::Ident(Ident::new(id, t.span()))
                } else {
                    t
                }
            })
            .collect::<Vec<_>>();

        let behavior = match stream.get(0).unwrap() {
            TokenTree::Ident(id) if id == "Option" => {
                stream.drain(0..2);
                stream.pop();
                FieldBehavior::Option
            }
            TokenTree::Ident(id)
                if id == "Vec"
                    && matches!(stream.get(2), Some(TokenTree::Ident(i)) if i == "Option") =>
            {
                stream.drain(2..4);
                stream.pop();
                FieldBehavior::Vec
            }
            _ => FieldBehavior::Retain,
        };

        let stream = stream.into_iter().collect::<TokenStream2>();

        (parse_quote!(#stream), behavior)
    }
}

impl From<&Field> for FieldData {
    fn from(field: &Field) -> Self {
        let retain = field
            .attrs
            .iter()
            .any(|f| matches!(f.path.get_ident(), Some(i) if i == "retain"));

        let attrs = field
            .attrs
            .iter()
            .filter(|f| matches!(f.path.get_ident(), Some(i) if i == "doc"))
            .cloned()
            .collect();

        let (ty, behavior) = if retain {
            (field.ty.clone(), FieldBehavior::Retain)
        } else {
            FieldData::mutate_type(&field.ty)
        };

        Self {
            field: Field {
                attrs,
                vis: field.vis.clone(),
                ident: field.ident.clone(),
                colon_token: field.colon_token,
                ty,
            },
            behavior,
        }
    }
}

struct StructBuilder {
    ident: Ident,
    fields: Vec<FieldData>,
}

impl StructBuilder {
    fn new(item: &ItemStruct) -> Self {
        Self {
            ident: Ident::new(
                item.ident
                    .to_string()
                    .strip_prefix("Nullable")
                    .expect("struct must start with \"Nullable\""),
                // TODO span
                item.ident.span(),
            ),
            fields: item.fields.iter().map(|f| f.into()).collect(),
        }
    }

    fn gen_struct(&self, original: &ItemStruct) -> ItemStruct {
        let tmp = ItemStruct {
            attrs: Vec::new(),
            vis: original.vis.clone(),
            struct_token: original.struct_token,
            ident: self.ident.clone(),
            generics: original.generics.clone(),
            fields: Fields::Named(FieldsNamed {
                // TODO span
                brace_token: Brace::default(),
                named: self
                    .fields
                    .iter()
                    .map(|f| f.field.clone())
                    .collect::<Punctuated<_, _>>(),
            }),
            semi_token: original.semi_token,
        };

        // TODO hack to get around lazy macro expansion for attributes
        parse_quote!(
            #[allow(clippy::missing_docs_in_private_items)]
            #[derive(Clone, Debug, Default)]
            #tmp
        )
    }

    fn gen_from_nullable_impl(&self, original: &ItemStruct) -> ItemImpl {
        let nullable_ident = &original.ident;
        let self_ident = &self.ident;
        let fields = self.fields.iter().map(|f| f.gen_from_nullable());
        parse_quote!(
            impl From<#nullable_ident> for #self_ident {
                fn from(other: #nullable_ident) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        )
    }

    fn gen_to_nullable_impl(&self, original: &ItemStruct) -> ItemImpl {
        let nullable_ident = &original.ident;
        let self_ident = &self.ident;
        let fields = self.fields.iter().map(|f| f.gen_to_nullable());
        parse_quote!(
            impl From<#self_ident> for #nullable_ident {
                fn from(other: #self_ident) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        )
    }
}

pub fn create_nullable_counterpart(item: ItemStruct) -> TokenStream {
    let builder: StructBuilder = StructBuilder::new(&item);
    let new_struct = builder.gen_struct(&item);
    let from_null = builder.gen_from_nullable_impl(&item);
    let to_null = builder.gen_to_nullable_impl(&item);

    quote!(
        #new_struct
        #from_null
        #to_null
    )
    .into()
}
