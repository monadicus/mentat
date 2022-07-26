// TODO temporary
#![allow(clippy::missing_docs_in_private_items)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Brace, AngleBracketedGenericArguments, Field,
    FieldValue, Fields, FieldsNamed, GenericArgument, Ident, ItemImpl, ItemStruct, Path,
    PathArguments, PathSegment, Type, TypePath,
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
            FieldBehavior::Retain => parse_quote!(#field_name: other.#field_name),
            FieldBehavior::Vec => {
                parse_quote!(#field_name: other.#field_name.into_iter().flatten().collect())
            }
            FieldBehavior::Option => parse_quote!(#field_name: other.#field_name.unwrap()),
        }
    }

    fn gen_to_nullable(&self) -> FieldValue {
        let field_name = &self.field.ident.as_ref().unwrap();
        match &self.behavior {
            FieldBehavior::Retain => parse_quote!(#field_name: other.#field_name),
            FieldBehavior::Vec => {
                parse_quote!(#field_name: other.#field_name.into_iter().map(|c| Some(c)).collect())
            }
            FieldBehavior::Option => parse_quote!(#field_name: Some(other.#field_name)),
        }
    }

    fn get_inner(ty: &Type) -> (Type, FieldBehavior) {
        let match_path = |segments: &Punctuated<PathSegment, _>| {
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
                &segments[0].arguments
            {
                if let GenericArgument::Type(t) = &args[0] {
                    t.clone()
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        };

        match ty {
            Type::Path(TypePath {
                path: Path { segments, .. },
                ..
            }) if FieldBehavior::Option == (&segments[0].ident).into() => {
                (match_path(segments), FieldBehavior::Option)
            }
            Type::Path(TypePath {
                path: Path { segments, .. },
                ..
            }) if FieldBehavior::Vec == (&segments[0].ident).into() => {
                let ty = Self::get_inner(&match_path(segments)).0;
                (parse_quote!(Vec<#ty>), FieldBehavior::Vec)
            }
            ty => (ty.clone(), FieldBehavior::Retain),
        }
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
            FieldData::get_inner(&field.ty)
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
