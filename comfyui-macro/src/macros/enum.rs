use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

pub fn enumerates_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("Enumerates can only be derived for enums"),
    };

    let variant_names: Vec<_> = variants
        .iter()
        .map(|variant| &variant.ident)
        .map(|ident| quote! { stringify!(#ident) })
        .collect();

    let variant_matches: Vec<_> = variants
        .iter()
        .map(|variant| &variant.ident)
        .map(|ident| quote! { #name::#ident })
        .collect();

    TokenStream::from(quote! {

        impl crate::node::EnumVariants for #name {
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
