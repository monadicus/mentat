// TODO temporary
#![allow(clippy::missing_docs_in_private_items)]

use std::fmt::Debug;

use proc_macro::TokenStream;
use proc_macro2::{Punct, Spacing, Span, TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, token::Brace, Field, FieldValue, Fields, FieldsNamed,
    Ident, ItemImpl, ItemStruct, Meta, MetaList, NestedMeta, Type,
};

#[derive(Default, Debug)]
enum Argument {
    #[default]
    None,
    Retain,
    Bytes,
    OptionEnum,
}

impl From<&Ident> for Argument {
    fn from(other: &Ident) -> Self {
        if other == "retain" {
            Self::Retain
        } else if other == "bytes" {
            Self::Bytes
        } else if other == "option_enum" {
            Self::OptionEnum
        } else {
            panic!("unsupported argument {}", other)
        }
    }
}

impl From<&Field> for Argument {
    fn from(other: &Field) -> Self {
        other
            .attrs
            .iter()
            .find(|f| matches!(f.path.get_ident(), Some(i) if i == "nullable"))
            .map(|f| match f.parse_meta().unwrap() {
                Meta::List(MetaList { nested, .. }) => {
                    if let NestedMeta::Meta(n) = &nested[0] {
                        n.path().get_ident().unwrap().into()
                    } else {
                        panic!("unsupported argument")
                    }
                }
                _ => panic!("unsupported argument"),
            })
            .unwrap_or_default()
    }
}

#[derive(Debug)]
enum FieldType {
    VecOption,
    Vec,
    Option,
    Other,
}

impl From<&[TokenTree]> for FieldType {
    fn from(stream: &[TokenTree]) -> Self {
        match stream.get(0).unwrap() {
            TokenTree::Ident(id) if id == "Option" => Self::Option,
            TokenTree::Ident(id)
                if id == "Vec"
                    && matches!(stream.get(2), Some(TokenTree::Ident(i)) if i == "Option") =>
            {
                Self::VecOption
            }
            TokenTree::Ident(id) if id == "Vec" => Self::Vec,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum FieldBehavior {
    Retain,
    VecOption,
    Option,
    RetainVecOption,
    RetainOption,
    Enum,
    Bytes,
}

impl From<(FieldType, Argument)> for FieldBehavior {
    fn from(other: (FieldType, Argument)) -> Self {
        match other {
            (FieldType::Other, Argument::None)
            | (FieldType::Other, Argument::Retain)
            | (FieldType::Vec, Argument::None)
            | (FieldType::Vec, Argument::Retain) => FieldBehavior::Retain,
            (FieldType::VecOption, Argument::None) => FieldBehavior::VecOption,
            (FieldType::Option, Argument::None) => FieldBehavior::Option,
            (FieldType::VecOption, Argument::Retain) => FieldBehavior::RetainVecOption,
            (FieldType::Option, Argument::Retain) => FieldBehavior::RetainOption,
            (FieldType::Other, Argument::OptionEnum) => FieldBehavior::Enum,
            (FieldType::Vec, Argument::Bytes) => FieldBehavior::Bytes,
            (f, a) => panic!("unsupported argument for field type: {f:?}, {a:?}"),
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
            FieldBehavior::VecOption => {
                parse_quote!(#field_name: other.#field_name.into_iter().flatten().map(|v| v.into()).collect())
            }
            FieldBehavior::Option => parse_quote!(#field_name: other.#field_name.unwrap().into()),
            FieldBehavior::RetainVecOption => {
                parse_quote!(#field_name: other.#field_name.into_iter().flatten().map(|v| Some(v.into())).collect())
            }
            FieldBehavior::RetainOption => {
                parse_quote!(#field_name: other.#field_name.map(|v| v.into()))
            }
            FieldBehavior::Enum => {
                parse_quote!(#field_name: if other.#field_name.is_empty() {
                    None
                } else {
                    Some(other.#field_name.into())
                })
            }
            FieldBehavior::Bytes => {
                parse_quote!(#field_name: crate::types::utils::encode_to_hex_string(&other.#field_name))
            }
        }
    }

    fn gen_to_nullable(&self) -> FieldValue {
        let field_name = &self.field.ident.as_ref().unwrap();
        match &self.behavior {
            FieldBehavior::Retain => {
                parse_quote!(#field_name: other.#field_name.into())
            }
            FieldBehavior::VecOption => {
                parse_quote!(#field_name: other.#field_name.into_iter().map(|v| Some(v.into())).collect())
            }
            FieldBehavior::Option => parse_quote!(#field_name: Some(other.#field_name.into())),
            FieldBehavior::RetainVecOption => {
                parse_quote!(#field_name: other.#field_name.into_iter().flatten().map(|v| Some(v.into())).collect())
            }
            FieldBehavior::RetainOption => {
                parse_quote!(#field_name: other.#field_name.map(|v| v.into()))
            }
            FieldBehavior::Enum => {
                parse_quote!(#field_name: other.#field_name.unwrap_or_default().into())
            }
            FieldBehavior::Bytes => {
                parse_quote!(#field_name: crate::types::utils::decode_from_hex_string(other.#field_name).unwrap())
            }
        }
    }

    fn mutate_type(ty: &Type, arg: Argument) -> (Type, FieldBehavior) {
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

        let field_type: FieldType = stream.as_slice().into();
        let behavior = (field_type, arg).into();

        match behavior {
            FieldBehavior::Option => {
                stream.drain(0..2);
                stream.pop();
            }
            FieldBehavior::VecOption => {
                stream.drain(2..4);
                stream.pop();
            }
            FieldBehavior::Enum => {
                stream.insert(0, TokenTree::Ident(Ident::new("Option", Span::call_site())));
                stream.insert(1, TokenTree::Punct(Punct::new('<', Spacing::Alone)));
                stream.push(TokenTree::Punct(Punct::new('>', Spacing::Alone)));
            }
            FieldBehavior::Bytes => {
                stream = vec![TokenTree::Ident(Ident::new("String", Span::call_site()))]
            }
            _ => {}
        }

        let stream = stream.into_iter().collect::<TokenStream2>();

        (parse_quote!(#stream), behavior)
    }
}

impl From<&Field> for FieldData {
    fn from(field: &Field) -> Self {
        let arg = field.into();

        let attrs = field
            .attrs
            .iter()
            .filter(|f| matches!(f.path.get_ident(), Some(i) if i == "doc"))
            .cloned()
            .collect();

        let (ty, behavior) = FieldData::mutate_type(&field.ty, arg);

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
