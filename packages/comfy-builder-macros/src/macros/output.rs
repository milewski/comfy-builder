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
    let mut inserts: Vec<proc_macro2::TokenStream> = Vec::new();
    // let mut tooltips_inserts: Vec<proc_macro2::TokenStream> = Vec::new();
    // let mut output_list_inserts: Vec<proc_macro2::TokenStream> = Vec::new();
    // let mut into_pyobject_fields = Vec::new();

    if let Fields::Named(fields_named) = fields {
        for field in fields_named.named {
            let field = FieldExtractor::from(&field);
            let value_ident = field.value_ident();
            let property_ident = field.property_ident();
            let named_attributes = field.named_attributes();
            let is_list = field.is_wrapped_by_vector();

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

            field_inserts.push(quote! {
                self.#property_ident.into_py_any(python)?
            });

            inserts.push(quote! {
                {
                    let kind = comfy_builder_core::ComfyDataTypes::from(stringify!(#value_ident));
                    let dict = pyo3::types::PyDict::new(python);
                    dict.set_item("is_output_list", #is_list)?;
                    dict.set_item("display_name", stringify!(#property_ident))?;

                    #(#attributes)*

                    io.getattr(kind.to_comfy())?.getattr("Output")?.call((), Some(&dict))?
                }
            });
        }
    }

    // let body = if into_pyobject_fields.is_empty() {
    //     quote! { Ok(pyo3::types::PyTuple::empty(py).into_pyobject(py)?) }
    // } else {
    //     quote! { (#(#into_pyobject_fields)*).into_pyobject(py) }
    // };

    TokenStream::from(quote! {
        use pyo3::prelude::*;

        // impl<'a> comfy_builder_core::node::OutputPort<'a> for #name {
        //     fn get_outputs() -> indexmap::IndexMap<&'static str, (&'static str, comfy_builder_core::node::DataType)> {
        //         let mut map = indexmap::IndexMap::new();
        //         #(#field_inserts)*
        //         map
        //     }
        //
        //     fn get_tooltips() -> Vec<&'static str> {
        //         let mut map = Vec::new();
        //         #(#tooltips_inserts)*
        //         map
        //     }
        //
        //     fn get_output_list() -> Vec<bool> {
        //         let mut map = Vec::new();
        //         #(#output_list_inserts)*
        //         map
        //     }
        // }
        //
        // impl<'py> pyo3::IntoPyObject<'py> for #name {
        //     type Target = pyo3::types::PyTuple;
        //     type Output = pyo3::Bound<'py, Self::Target>;
        //     type Error = pyo3::PyErr;
        //
        //     fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        //         #body
        //     }
        // }

        // impl<'a> From<comfy_builder_core::prelude::Kwargs<'a>> for #name {
        //     fn from(kwargs: comfy_builder_core::prelude::Kwargs) -> Self {
        //         #name {
        //             number: kwargs
        //                 .as_ref()
        //                 .and_then(|kwargs| kwargs.get_item("number").ok())
        //                 .flatten()
        //                 .and_then(|value| value.extract::<usize>().ok())
        //                 .unwrap(),
        //         }
        //     }
        // }

        // impl comfy_builder_core::prelude::Out for #name {
        //
        //     fn to_schema<'py>(&self, python: pyo3::Python<'py>) -> PyResult<pyo3::Bound<'py, pyo3::types::PyTuple>> {
        //         // (self.number,).into_pyobject(python)
        //
        //         // pyo3::types::PyTuple::new(python, #(#field_inserts)*)
        //         pyo3::types::PyTuple::new(python, [#(#field_inserts)*])
        //
        //     }
        //
        // }

        impl<'py> comfy_builder_core::prelude::Out<'py> for #name {
            fn blueprints(python: pyo3::Python<'py>, io: &pyo3::Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyList>> {
                pyo3::types::PyList::new(python,[#(#inserts),*])
            }

            fn to_schema(self, python: pyo3::Python) -> pyo3::PyResult<pyo3::Bound<pyo3::types::PyTuple>> {
                pyo3::types::PyTuple::new(python,[#(#field_inserts),*])
            }
        }

    })
}
