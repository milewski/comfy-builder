use crate::macros::{extract_field_info, IdentKind};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn node_output_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Get the fields from the struct
    let fields = match input.data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("OutputPort can only be derived for structs"),
    };

    let mut field_inserts = Vec::new();
    let mut into_pyobject_fields = Vec::new();

    if let Fields::Named(fields_named) = fields {
        for field in fields_named.named {
            if let (Some(kind), Some(attribute)) = (extract_field_info(&field.ty), &field.ident) {
                let ident = match kind {
                    IdentKind::Required(ident) => ident,
                    IdentKind::Optional(ident) => ident,
                };

                let token = quote! { comfy_builder_core::node::DataType::from(stringify!(#ident)) };

                field_inserts.push(quote! {
                    map.insert(stringify!(#attribute), #token);
                });

                into_pyobject_fields.push(quote! {
                    self.#attribute.into_pyobject(py)?,
                });
            }
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
            fn get_outputs() -> indexmap::IndexMap<&'static str, comfy_builder_core::node::DataType> {
                let mut map = indexmap::IndexMap::new();
                #(#field_inserts)*
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
