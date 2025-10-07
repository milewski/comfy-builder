use crate::helpers::FieldExtractor;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn node_output_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("NodeOutput can only be derived for structs"),
    };

    let mut field_inserts: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut tooltips_inserts: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut into_pyobject_fields = Vec::new();

    if let Fields::Named(fields_named) = fields {
        for field in fields_named.named {
            let field = FieldExtractor::from(&field);
            let ident = field.value_ident();
            let property_ident = field.property_ident();
            let named_attributes = field.named_attributes();

            let label = named_attributes
                .get("label")
                .map(|label| quote! { #label })
                .unwrap_or_else(|| quote! { stringify!(#property_ident) });

            let tooltip = named_attributes
                .get("tooltip")
                .or_else(|| named_attributes.get("doc"))
                .map(|tooltip| quote! { #tooltip })
                .unwrap_or_else(|| quote! { "" });

            let token = quote! {
                comfy_builder_core::node::DataType::from(stringify!(#ident))
            };

            tooltips_inserts.push(quote! {
                map.push(#tooltip);
            });

            field_inserts.push(quote! {
                map.insert(stringify!(#property_ident), (#label, #token));
            });

            into_pyobject_fields.push(quote! {
                self.#property_ident.into_pyobject(py)?,
            });
        }
    }

    let body = if into_pyobject_fields.is_empty() {
        quote! { Ok(pyo3::types::PyTuple::empty(py).into_pyobject(py)?) }
    } else {
        quote! { (#(#into_pyobject_fields)*).into_pyobject(py) }
    };

    TokenStream::from(quote! {
        use pyo3::prelude::*;

        impl<'a> comfy_builder_core::node::OutputPort<'a> for #name {
            fn get_outputs() -> indexmap::IndexMap<&'static str, (&'static str, comfy_builder_core::node::DataType)> {
                let mut map = indexmap::IndexMap::new();
                #(#field_inserts)*
                map
            }

            fn get_tooltips() -> Vec<&'static str> {
                let mut map = Vec::new();
                #(#tooltips_inserts)*
                map
            }
        }

        impl<'py> pyo3::IntoPyObject<'py> for #name {
            type Target = pyo3::types::PyTuple;
            type Output = pyo3::Bound<'py, Self::Target>;
            type Error = pyo3::PyErr;

            fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
                #body
            }
        }
    })
}
