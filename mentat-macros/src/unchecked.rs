//! generates the checked counterparts of an unchecked struct

use std::fmt::Debug;

use proc_macro2::{Punct, Spacing, Span, TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
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
    Meta,
    MetaList,
    NestedMeta,
    Type,
};

/// the unchecked argument over a struct field
#[derive(Default, Debug, Clone, Copy)]
enum Argument {
    /// no argument
    #[default]
    None,
    /// the field should not be changed
    Retain,
    /// the field contains bytes
    Bytes,
    /// the field should be converted to an Option<Enum>
    OptionEnum,
    /// the field should be converted to a usize
    Usize,
    /// the field should be converted to an Option<usize>
    OptionUsize,
}

impl From<&Ident> for Argument {
    fn from(other: &Ident) -> Self {
        if other == "retain" {
            Self::Retain
        } else if other == "bytes" {
            Self::Bytes
        } else if other == "option_enum" {
            Self::OptionEnum
        } else if other == "usize" {
            Self::Usize
        } else if other == "option_usize" {
            Self::OptionUsize
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
            .find(|f| matches!(f.path.get_ident(), Some(i) if i == "unchecked"))
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

/// the type contained within the field
#[derive(Debug, Clone, Copy)]
enum FieldType {
    /// Vec<Option<T>>
    VecOption,
    /// Vec<T>
    Vec,
    /// Option<T>
    Option,
    /// none of the above
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

/// How the field should be changed
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FieldBehavior {
    /// T -> T
    Retain,
    /// Vec<Option<T>> -> Vec<T>
    VecOption,
    /// Option<T> -> T
    Option,
    /// Vec<Option<T>> -> Vec<Option<T>>
    RetainVecOption,
    /// Option<T> -> Option<T>
    RetainOption,
    /// Enum -> Option<Enum>
    OptionEnum,
    /// Vec<u8> -> Vec<u8>
    Bytes,
    /// Option<isize> -> usize
    Usize,
    /// Option<isize> -> Option<usize>
    OptionUsize,
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
            (FieldType::Other, Argument::OptionEnum) => FieldBehavior::OptionEnum,
            (FieldType::Vec, Argument::Bytes) => FieldBehavior::Bytes,
            (FieldType::Other, Argument::Usize) => FieldBehavior::Usize,
            (FieldType::Option, Argument::OptionUsize) => FieldBehavior::OptionUsize,
            (f, a) => panic!("unsupported argument for field type: {f:?}, {a:?}"),
        }
    }
}

/// Data for each field element
#[derive(Debug)]
struct FieldData {
    /// the target field
    field: Field,
    /// the unchecked behavior of the field
    behavior: FieldBehavior,
}

impl FieldData {
    /// generates the `impl From<UncheckedT> for T` for the struct
    fn gen_from_unchecked(&self) -> FieldValue {
        let field_name = &self.field.ident.as_ref().unwrap();
        match &self.behavior {
            FieldBehavior::Retain | FieldBehavior::Bytes => {
                parse_quote!(#field_name: other.#field_name.into())
            }
            FieldBehavior::VecOption => {
                parse_quote!(#field_name: other.#field_name.into_iter().flatten().map(|v| v.into()).collect())
            }
            FieldBehavior::Option => {
                parse_quote!(#field_name: other.#field_name.unwrap().into())
            }
            FieldBehavior::RetainVecOption => {
                parse_quote!(#field_name: other.#field_name.into_iter().flatten().map(|v| Some(v.into())).collect())
            }
            FieldBehavior::RetainOption => {
                parse_quote!(#field_name: other.#field_name.map(|v| v.into()))
            }
            FieldBehavior::OptionEnum => {
                parse_quote!(#field_name: if other.#field_name.is_empty() {
                    None
                } else {
                    Some(other.#field_name.into())
                })
            }
            FieldBehavior::Usize => {
                parse_quote!(#field_name: other.#field_name.try_into().unwrap())
            }
            FieldBehavior::OptionUsize => {
                parse_quote!(#field_name: other.#field_name.map(|v| v.try_into().unwrap()))
            }
        }
    }

    /// generates the `impl From<T> for UncheckedT` for the struct
    fn gen_to_unchecked(&self) -> FieldValue {
        let field_name = &self.field.ident.as_ref().unwrap();
        match &self.behavior {
            FieldBehavior::Retain | FieldBehavior::Bytes => {
                parse_quote!(#field_name: other.#field_name.into())
            }
            FieldBehavior::VecOption => {
                parse_quote!(#field_name: other.#field_name.into_iter().map(|v| Some(v.into())).collect())
            }
            FieldBehavior::Option => {
                parse_quote!(#field_name: Some(other.#field_name.into()))
            }
            FieldBehavior::RetainVecOption => {
                parse_quote!(#field_name: other.#field_name.into_iter().flatten().map(|v| Some(v.into())).collect())
            }
            FieldBehavior::RetainOption => {
                parse_quote!(#field_name: other.#field_name.map(|v| v.into()))
            }
            FieldBehavior::OptionEnum => {
                parse_quote!(#field_name: other.#field_name.unwrap_or_default().into())
            }
            FieldBehavior::Usize => {
                parse_quote!(#field_name: other.#field_name.try_into().unwrap())
            }
            FieldBehavior::OptionUsize => {
                parse_quote!(#field_name: other.#field_name.map(|v| v.try_into().unwrap()))
            }
        }
    }

    /// mutates the given field type and also gets its behavior
    fn mutate_type(ty: &Type, arg: Argument) -> (Type, FieldBehavior) {
        let mut stream = ty
            .clone()
            .into_token_stream()
            .into_iter()
            .map(|t| {
                if let Some(id) = t.to_string().strip_prefix("Unchecked") {
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
            FieldBehavior::OptionEnum => {
                stream.insert(0, TokenTree::Ident(Ident::new("Option", Span::call_site())));
                stream.insert(1, TokenTree::Punct(Punct::new('<', Spacing::Alone)));
                stream.push(TokenTree::Punct(Punct::new('>', Spacing::Alone)));
            }
            FieldBehavior::OptionUsize => {
                stream = vec![
                    TokenTree::Ident(Ident::new("Option", Span::call_site())),
                    TokenTree::Punct(Punct::new('<', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("usize", Span::call_site())),
                    TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                ];
            }
            FieldBehavior::Usize => {
                stream = vec![TokenTree::Ident(Ident::new("usize", Span::call_site()))]
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

        let (ty, behavior) = FieldData::mutate_type(&field.ty, arg);

        let mut docs = field
            .attrs
            .iter()
            .filter(|a| matches!(a.path.get_ident(), Some(i) if i == "doc"))
            .cloned()
            .collect::<Vec<_>>();

        let attrs = match behavior {
            FieldBehavior::Retain => field
                .attrs
                .iter()
                .filter(|a| matches!(a.path.get_ident(), Some(i) if i != "unchecked"))
                .cloned()
                .collect(),
            FieldBehavior::VecOption | FieldBehavior::RetainVecOption => {
                docs.push(parse_quote!(#[serde(skip_serializing_if = "Vec::is_empty")]));
                docs
            }
            FieldBehavior::Option | FieldBehavior::Usize => docs,
            FieldBehavior::OptionEnum
            | FieldBehavior::RetainOption
            | FieldBehavior::OptionUsize => {
                docs.push(parse_quote!(#[serde(skip_serializing_if = "Option::is_none")]));
                docs
            }
            FieldBehavior::Bytes => {
                docs.push(parse_quote!(#[serde(
                    rename = "hex_bytes",
                    skip_serializing_if = "Vec::is_empty",
                    serialize_with = "bytes_to_hex_str",
                    deserialize_with = "null_default_bytes_to_hex"
                )]));
                docs
            }
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

/// contains info to generate the different parts of the the non-unchecked
/// struct
pub struct StructBuilder {
    /// the struct identifier, with `Unchecked` removed
    ident: Ident,
    /// the fields of the struct
    fields: Vec<FieldData>,
}

impl StructBuilder {
    /// creates a new StructBuilder instance
    pub fn new(item: &ItemStruct) -> Self {
        Self {
            ident: Ident::new(
                item.ident
                    .to_string()
                    .strip_prefix("Unchecked")
                    .expect("struct must start with \"Unchecked\""),
                // TODO span
                item.ident.span(),
            ),
            fields: item.fields.iter().map(|f| f.into()).collect(),
        }
    }

    /// generates the non-unchecked struct
    pub fn gen_struct(&self, original: &ItemStruct) -> ItemStruct {
        let tmp = ItemStruct {
            attrs: original.attrs.clone(),
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
            #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
            #tmp
        )
    }

    /// generates the `impl From<UncheckedT> for T` code
    pub fn gen_from_unchecked_impl(&self, original: &ItemStruct) -> ItemImpl {
        let unchecked_ident = &original.ident;
        let self_ident = &self.ident;
        let fields = self.fields.iter().map(|f| f.gen_from_unchecked());
        parse_quote!(
            impl From<#unchecked_ident> for #self_ident {
                fn from(other: #unchecked_ident) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        )
    }

    /// generates the `impl From<T> for UncheckedT` code
    pub fn gen_to_unchecked_impl(&self, original: &ItemStruct) -> ItemImpl {
        let unchecked_ident = &original.ident;
        let self_ident = &self.ident;
        let fields = self.fields.iter().map(|f| f.gen_to_unchecked());
        parse_quote!(
            impl From<#self_ident> for #unchecked_ident {
                fn from(other: #self_ident) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }
        )
    }
}
