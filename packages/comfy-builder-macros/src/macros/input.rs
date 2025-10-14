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
    let is_list = fields.iter().any(|field| field.is_wrapped_by_vector());

    for field in fields {
        let property_ident = field.property_ident();
        let value_ident = field.value_ident();

        let value_type_call = {
            if field.is_wrapped_by_vector() || field.is_optional() {
                field.inner_value_type_call().unwrap()
            } else {
                field.value_type_call()
            }
        };

        let mut named_attributes = field.named_attributes();
        let is_optional = field.is_optional();

        let display_name = named_attributes
            .remove("display_name")
            .map(|label| quote! { #label })
            .unwrap_or_else(|| quote! { stringify!(#property_ident) });

        let attributes: Vec<proc_macro2::TokenStream> = named_attributes
            .into_iter()
            .map(|(key, value)| {
                if key == "doc" {
                    ("tooltip".to_string(), value)
                } else {
                    (key, value)
                }
            })
            .map(|(key, value)| quote! { dict.set_item(#key, #value)?; })
            .collect();

        {
            elements.push(quote! {
                {
                    use comfy_builder_core::prelude::AsInput;

                    let mut dict = pyo3::types::PyDict::new(python);
                    let comfy_type = #value_type_call::comfy_type();

                    #(#attributes)*

                    #value_type_call::set_options(&mut dict, &io)?;

                    dict.set_item("optional", #is_optional)?;

                    io.getattr(comfy_type.to_string())?.getattr("Input")?.call((#display_name,), Some(&dict))?
                }
            });
        }

        {
            let extract_type = field.output_ident(is_list);
            let mut extract_logic = quote! {
                kwargs
                    .as_ref()
                    .and_then(|kwargs| kwargs.get_item(#display_name).ok())
                    .flatten()
                    .and_then(|value| value.extract::<#extract_type>().ok())
            };

            // If the user has defined **any** input as a Vec, ComfyUI will treat all inputs as lists.
            // So on the Rust side, when an item is not defined as a list but others are,
            // the first input is always retrieved from that list instead.
            if is_list && !field.is_wrapped_by_vector() {

                extract_logic = if is_optional {
                    quote! { #extract_logic.and_then(|list| list.map(|list| list.into_iter().next())) }
                } else {
                    quote! { #extract_logic.and_then(|list| list.into_iter().next()) }
                };

            }

            // If the field is a `String`, strip out empty values so that the
            // field’s default is used instead of an empty string.
            // Returning `None` tells the deserializer to fall back to the default.
            let extract_logic = if field.is_string() && is_optional {
                quote! { #extract_logic.and_then(|string| string.map(|string| if string.is_empty() { None } else { Some(string) })) }
            } else {
                quote! { #extract_logic }
            };

            decoders.push(if field.is_required() {
                quote! { #property_ident: #extract_logic.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("unable to retrieve attribute"))? }
            } else {
                quote! { #property_ident: #extract_logic.flatten() }
            });
        }
    }

    TokenStream::from(quote! {
        impl<'py> comfy_builder_core::node::In<'py> for #name {
            fn blueprints(python: pyo3::Python<'py>, io: &pyo3::Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyList>> {
                use comfy_builder_core::prelude::AsInput;

                pyo3::types::PyList::new(python,[#(#elements),*])
            }

            fn is_list() -> bool {
                #is_list
            }
        }

        impl<'py> TryFrom<comfy_builder_core::prelude::Kwargs<'py>> for #name {
            type Error = pyo3::PyErr;

            fn try_from(kwargs: comfy_builder_core::prelude::Kwargs) -> Result<Self, Self::Error> {
                Ok(#name {
                    #(#decoders),*
                })
            }
        }
    })
}
