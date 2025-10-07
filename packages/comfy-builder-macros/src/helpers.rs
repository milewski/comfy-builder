use crate::options::{AnyOption, Options};
use proc_macro2::{Ident, TokenTree};
use quote::quote;
use std::collections::HashMap;
use syn::{Expr, ExprLit, Field, GenericArgument, Lit, PathArguments, Type, TypePath};

macro_rules! numeric_types {
    (signed) => {
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
    };
    (unsigned) => {
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
    };
    (float) => {
        "f32" | "f64"
    };
    () => {
        numeric_types!(unsigned) | numeric_types!(signed) | numeric_types!(float)
    };
}

use numeric_types;

#[derive(Debug)]
pub struct FieldExtractor<'a> {
    field: &'a Field,
}

impl<'a> From<&'a Field> for FieldExtractor<'a> {
    fn from(field: &'a Field) -> Self {
        FieldExtractor { field }
    }
}

impl<'a> FieldExtractor<'a> {
    pub fn attributes(&self) -> Option<AnyOption> {
        self.field
            .attrs
            .iter()
            .filter_map(|attr| attr.meta.require_list().ok())
            .find_map(|meta| meta.parse_args::<AnyOption>().ok())
    }

    pub fn named_attributes(&self) -> HashMap<String, &Lit> {
        self.field
            .attrs
            .iter()
            .filter_map(|attr| attr.meta.require_name_value().ok())
            .flat_map(|meta| {
                meta.path.require_ident().map(|ident| {
                    (
                        ident.to_string(),
                        match &meta.value {
                            Expr::Lit(ExprLit { lit, .. }) => lit,
                            _ => unreachable!(),
                        },
                    )
                })
            })
            .collect()
    }

    pub fn options(&self) -> proc_macro2::TokenStream {
        self.attributes()
            .map(|option| option.generate_token_stream())
            .unwrap_or_default()
            .into()
    }

    pub fn property_ident(&self) -> &Ident {
        self.field.ident.as_ref().unwrap()
    }

    pub fn is_string(&self) -> bool {
        let kind_str = self.value_ident().to_string();

        matches!(kind_str.as_str(), "String")
    }

    pub fn is_tensor_type(&self) -> bool {
        let kind_str = self.value_ident().to_string();

        matches!(
            kind_str.as_str(),
            numeric_types!() | "Tensor" | "Mask" | "Latent"
        )
    }

    pub fn is_primitive(&self) -> bool {
        let kind_str = self.value_ident().to_string();

        matches!(kind_str.as_str(), numeric_types!() | "String" | "bool")
    }

    pub fn is_enum(&self) -> bool {
        self.field
            .attrs
            .iter()
            .find(|attribute| attribute.meta.path().is_ident("attribute"))
            .and_then(|attribute| attribute.meta.require_list().ok())
            .and_then(|meta| {
                meta.tokens
                    .clone()
                    .into_iter()
                    .find(|token| matches!(token, TokenTree::Ident(ident) if ident == "enum"))
            })
            .is_some()
    }

    pub fn is_hidden(&self) -> bool {
        matches!(
            self.value_ident().to_string().as_str(),
            "UniqueId" | "Prompt" | "ExtraPngInfo" | "DynPrompt"
        )
    }

    pub fn get_hidden_tokens(&self) -> Option<&'static str> {
        self.is_hidden()
            .then(|| match self.value_ident().to_string().as_str() {
                "UniqueId" => "UNIQUE_ID",
                "Prompt" => "PROMPT",
                "ExtraPngInfo" => "EXTRA_PNGINFO",
                "DynPrompt" => "DYNPROMPT",
                _ => unreachable!(),
            })
    }

    pub fn options_default(&self) -> proc_macro2::TokenStream {
        match self.value_ident().to_string().as_str() {
            numeric_types!() => {
                let ident = self.value_ident();

                quote! {
                    dict.set_item("min", #ident::MIN)?;
                    dict.set_item("max", #ident::MAX)?;
                }
            }
            _ => quote! {},
        }
    }

    pub fn value_ident(&self) -> &Ident {
        match &self.field.ty {
            Type::Path(type_path) => match type_path.path.get_ident() {
                Some(ident) => ident,
                None => match extract_ident_from_option(type_path) {
                    Some(ident) => ident,
                    None => unreachable!(),
                },
            },
            _ => unreachable!(),
        }
    }

    pub fn is_required(&self) -> bool {
        match &self.field.ty {
            Type::Path(type_path) => type_path.path.get_ident().is_some(),
            _ => false,
        }
    }
}

pub fn extract_ident_from_option(path: &TypePath) -> Option<&Ident> {
    if path.path.segments.len() == 1
        && path.path.segments[0].ident == "Option"
        && let PathArguments::AngleBracketed(angle) = &path.path.segments[0].arguments
        && let Some(GenericArgument::Type(inner_ty)) = angle.args.first()
        && let Type::Path(inner_path) = inner_ty
        && let Some(ident) = inner_path.path.get_ident()
    {
        return Some(ident);
    }

    None
}
