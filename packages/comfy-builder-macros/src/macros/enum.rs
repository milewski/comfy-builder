use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Expr, ExprLit, Lit, Variant, parse_macro_input};

fn fetch_label(variant: &Variant) -> Option<String> {
    variant
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("label"))
        .and_then(|attribute| attribute.meta.require_name_value().ok())
        .and_then(|meta| match &meta.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(content),
                ..
            }) => Some(content.value()),
            _ => None,
        })
}

pub fn enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("Enum can only be derived for enums."),
    };

    let variant_names: Vec<_> = variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;

            if let Some(label) = fetch_label(variant) {
                quote! { #label }
            } else {
                quote! { stringify!(#ident) }
            }
        })
        .collect();

    let variant_matches: Vec<_> = variants
        .iter()
        .map(|variant| &variant.ident)
        .map(|ident| quote! { #name::#ident })
        .collect();

    TokenStream::from(quote! {

        use pyo3::prelude::*;

        impl comfy_builder_core::node::EnumVariants for #name {
            fn variants() -> Vec<&'static str> {
                vec![#(#variant_names),*]
            }
        }

        impl From<String> for #name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    #(#variant_names => #variant_matches,)*
                    _ => panic!("Invalid variant name: {}", value),
                }
            }
        }

        impl<'py> pyo3::FromPyObject<'py> for #name {
            fn extract_bound(object: &pyo3::Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
                Ok(Self::from(object.extract::<String>()?))
            }
        }

    })
}
