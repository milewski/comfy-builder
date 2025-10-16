use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Expr, ExprLit, Lit, Variant, parse_macro_input};

fn fetch_display_name(variant: &Variant) -> Option<String> {
    variant
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("display_name"))
        .and_then(|attribute| attribute.meta.require_name_value().ok())
        .and_then(|meta| match &meta.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(content), ..
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

            if let Some(label) = fetch_display_name(variant) {
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
        use pyo3::exceptions::*;

        inventory::submit! {
            comfy_builder_core::registry::EnumRegistration { name: stringify!(#name) }
        }

        impl TryFrom<&str> for #name {
            type Error = pyo3::PyErr;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    #(#variant_names => Ok(#variant_matches),)*
                    _ => Err(pyo3::exceptions::PyValueError::new_err(format!("invalid variant name: {}", value))),
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{}", match self {
                    #(#variant_matches => #variant_names,)*
                })
            }
        }

        impl<'py> pyo3::IntoPyObject<'py> for #name {
            type Target = pyo3::PyAny;
            type Output = pyo3::Bound<'py, Self::Target>;
            type Error = pyo3::PyErr;

            fn into_pyobject(self, python: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
                self.to_string().into_bound_py_any(python)
            }
        }

        impl<'py> pyo3::FromPyObject<'py> for #name {
            fn extract_bound(object: &pyo3::Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
                Self::try_from(object.extract::<&str>()?)
            }
        }

        impl<'py> comfy_builder_core::prelude::AsInput<'py> for #name {
            fn comfy_type() -> comfy_builder_core::prelude::ComfyType {
                comfy_builder_core::prelude::ComfyType::Enum
            }

            fn set_options(dict: &mut pyo3::Bound<'py, pyo3::types::PyDict>, io: &pyo3::Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<()> {
                dict.set_item("options", vec![#(#variant_names),*])?;
                Ok(())
            }
        }
    })
}
