use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields
    ,
};
use crate::helpers::FieldExtractor;

pub fn node_input_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("NodeInput only works on structs with named fields"),
        },
        _ => panic!("NodeInput only works on structs"),
    };

    let mut decoders: Vec<proc_macro2::TokenStream> = vec![];
    let mut elements: Vec<proc_macro2::TokenStream> = vec![];

    for field in fields.iter().map(FieldExtractor::from) {
        let property_ident = field.property_ident();
        let value_ident = field.value_ident();
        let options_default = field.options_default();
        let options = field.options();
        let bucket = if field.is_required() {
            quote! { required }
        } else {
            quote! { optional }
        };

        if field.is_primitive() || field.is_tensor_type() {
            elements.push(quote! {
                let dict = pyo3::types::PyDict::new(py);
                let data_type = comfy_builder_core::node::DataType::from(stringify!(#value_ident)).to_string();

                #options_default
                #options

                #bucket.set_item(stringify!(#property_ident), (data_type, dict))?;
            });
        }

        if field.is_enum() {
            elements.push(quote! {
                #bucket.set_item(stringify!(#property_ident), (#value_ident::variants(),))?;
            });
        }

        if let Some(token) = field.get_hidden_tokens() {
            elements.push(quote! {
                hidden.set_item(stringify!(#property_ident), #token)?;
            })
        }

        let extract_logic = quote! {
            kwargs
                .and_then(|kwargs| kwargs.get_item(stringify!(#property_ident)).ok())
                .flatten()
                .and_then(|value| value.extract::<#value_ident>().ok())
        };

        // If the field is a `String`, strip out empty values so that the
        // field’s default is used instead of an empty string.
        // Returning `None` tells the deserializer to fall back to the default.
        let extract_logic = if field.is_string() {
            quote! { #extract_logic.and_then(|string| if string.is_empty() { None } else { Some(string) }) }
        } else {
            quote! { #extract_logic }
        };

        decoders.push(if field.is_required() {
            quote! { #property_ident: #extract_logic.expect("unable to retrieve attribute."), }
        } else {
            quote! { #property_ident: #extract_logic, }
        });
    }

    TokenStream::from(quote! {

        use comfy_builder_core::node::EnumVariants;
        use comfy_builder_core::node::InputPort;
        use pyo3::prelude::*;

        impl<'a> comfy_builder_core::node::InputPort<'a> for #name {

            fn get_inputs(py: pyo3::Python<'a>) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::types::PyDict>> {

                let output = pyo3::types::PyDict::new(py);
                let required = pyo3::types::PyDict::new(py);
                let optional = pyo3::types::PyDict::new(py);
                let hidden = pyo3::types::PyDict::new(py);

                #(#elements)*

                output.set_item("required", required)?;
                output.set_item("optional", optional)?;
                output.set_item("hidden", hidden)?;

                pyo3::PyResult::Ok(output)

            }

        }

        impl<'a> std::convert::From<Option<&'a pyo3::Bound<'a, pyo3::types::PyDict>>> for #name {
            fn from(kwargs: Option<&'a pyo3::Bound<'a, pyo3::types::PyDict>>) -> Self {
                #name {
                    #(#decoders)*
                }
            }
        }

    })
}
