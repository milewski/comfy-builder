use crate::helpers::FieldExtractor;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

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

    let mut elements: Vec<proc_macro2::TokenStream> = vec![];
    let mut decoders: Vec<proc_macro2::TokenStream> = vec![];

    let fields: Vec<_> = fields.iter().map(FieldExtractor::from).collect();
    // let is_list = fields.iter().any(|field| field.is_wrapped_by_vector());

    for field in fields {
        let property_ident = field.property_ident();
        let value_ident = field.value_ident();
        let options_default = field.options_default();
        let options = field.options();
        let mut named_attributes = field.named_attributes();

        let display_name = named_attributes
            .remove("display_name")
            .map(|label| quote! { #label })
            .unwrap_or_else(|| quote! { stringify!(#property_ident) });

        let extra: Vec<proc_macro2::TokenStream> = named_attributes
            .into_iter()
            .map(|(key, value)| quote! { extra.set_item(#key, #value)?; })
            .collect();

        let is_optional = field.is_optional();

        if field.is_primitive() {
            elements.push(quote! {
                {
                    let extra = pyo3::types::PyDict::new(python);
                    
                    #(#extra)*
                    
                    let kind = comfy_builder_core::ComfyDataTypes::from(stringify!(#value_ident));
                    let dict = kind.to_type().into_dict(python, &io, extra)?;

                    dict.set_item("optional", #is_optional)?;

                    io.getattr(kind.to_comfy())?.getattr("Input")?.call((#display_name,), Some(&dict))?
                }
            });
        }

        decoders.push(quote! {
            #property_ident: kwargs
                .as_ref()
                .and_then(|kwargs| kwargs.get_item(stringify!(#property_ident)).ok())
                .flatten()
                .and_then(|value| value.extract::<#value_ident>().ok())
                .unwrap()
        })

        // let label = named_attributes
        //     .get("label")
        //     .map(|label| quote! { #label })
        //     .unwrap_or_else(|| quote! { stringify!(#property_ident) });

        // let bucket = if field.is_required() {
        //     quote! { required }
        // } else {
        //     quote! { optional }
        // };

        // if field.is_primitive() || field.is_tensor_type() {
        //     elements.push(quote! {
        //         let dict = pyo3::types::PyDict::new(py);
        //         let data_type = comfy_builder_core::node::DataType::from(stringify!(#value_ident)).to_string();
        //
        //         #options_default
        //         #options
        //
        //         #bucket.set_item(#label, (data_type, dict))?;
        //     });
        // }
        //
        // if field.is_enum() {
        //     elements.push(quote! {
        //         #bucket.set_item(#label, (#value_ident::variants(),))?;
        //     });
        // }
        //
        // if let Some(token) = field.get_hidden_tokens() {
        //     elements.push(quote! {
        //         hidden.set_item(#label, #token)?;
        //     })
        // }
        //
        // let extract_type = field.output_ident(is_list);
        // let mut extract_logic = quote! {
        //     kwargs
        //         .and_then(|kwargs| kwargs.get_item(#label).ok())
        //         .flatten()
        //         .and_then(|value| value.extract::<#extract_type>().ok())
        // };
        //
        // // If the user has defined **any** input as a Vec, ComfyUI will treat all inputs as lists.
        // // So on the Rust side, when an item is not defined as a list but others are,
        // // the first input is always retrieved from that list instead.
        // if is_list && !field.is_wrapped_by_vector() {
        //     extract_logic = quote! {
        //         #extract_logic.and_then(|list| list.into_iter().next())
        //     }
        // }
        //
        // // If the field is a `String`, strip out empty values so that the
        // // field’s default is used instead of an empty string.
        // // Returning `None` tells the deserializer to fall back to the default.
        // let extract_logic = if field.is_string() {
        //     quote! { #extract_logic.and_then(|string| if string.is_empty() { None } else { Some(string) }) }
        // } else {
        //     quote! { #extract_logic }
        // };
        //
        // decoders.push(if field.is_required() {
        //     quote! { #property_ident: #extract_logic.expect("unable to retrieve attribute."), }
        // } else {
        //     quote! { #property_ident: #extract_logic, }
        // });
    }

    TokenStream::from(quote! {

        // use comfy_builder_core::node::EnumVariants;
        // use comfy_builder_core::node::InputPort;
        // use pyo3::prelude::*;
        // use comfy_builder_core::prelude::*;

        // impl<'a> comfy_builder_core::node::InputPort<'a> for #name {
        //
        //     fn get_inputs(py: pyo3::Python<'a>) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::types::PyDict>> {
        //
        //         let output = pyo3::types::PyDict::new(py);
        //         let required = pyo3::types::PyDict::new(py);
        //         let optional = pyo3::types::PyDict::new(py);
        //         let hidden = pyo3::types::PyDict::new(py);
        //
        //         #(#elements)*
        //
        //         output.set_item("required", required)?;
        //         output.set_item("optional", optional)?;
        //         output.set_item("hidden", hidden)?;
        //
        //         pyo3::PyResult::Ok(output)
        //
        //     }
        //
        //     fn is_input_list() -> bool {
        //         #is_list
        //     }
        //
        // }
        //
        // impl<'a> std::convert::From<Option<&'a pyo3::Bound<'a, pyo3::types::PyDict>>> for #name {
        //     fn from(kwargs: Option<&'a pyo3::Bound<'a, pyo3::types::PyDict>>) -> Self {
        //         #name {
        //             #(#decoders)*
        //         }
        //     }
        // }

        impl<'py> comfy_builder_core::prelude::In<'py> for #name {
            fn blueprints(python: pyo3::Python<'py>, io: &pyo3::Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyList>> {
                pyo3::types::PyList::new(python,[#(#elements),*])
            }
        }

        impl<'py> From<comfy_builder_core::prelude::Kwargs<'py>> for #name {
            fn from(kwargs: comfy_builder_core::prelude::Kwargs) -> Self {
                #name {
                    #(#decoders),*
                }
            }
        }
    })
}
