use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{ToTokens, quote};
use std::collections::HashMap;
use std::ops::Deref;
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Expr, Token, parse, parse_macro_input};

#[derive(Debug)]
struct Arguments {
    map: HashMap<String, String>,
}

impl Deref for Arguments {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut map = HashMap::new();
        let allows_attributes = ["id", "display_name", "category", "description"];

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: Expr = input.parse()?;

            if allows_attributes.contains(&key.to_string().as_str()) {
                map.insert(
                    key.to_string(),
                    value
                        .into_token_stream()
                        .to_string()
                        .trim_matches('"')
                        .to_string(),
                );
            } else {
                Err(syn::Error::new(key.span(), "unrecognized attribute"))?;
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Arguments { map })
    }
}

pub fn node(attr: TokenStream, input: TokenStream) -> TokenStream {
    let arguments = parse_macro_input!(attr as Arguments);
    let input_struct = parse_macro_input!(input as DeriveInput);
    let ident = &input_struct.ident;

    let node_id = arguments
        .get("id")
        .cloned()
        .unwrap_or_else(|| ident.to_string());

    let display_name = arguments
        .get("display_name")
        .map(|value| quote! { Some(#value) })
        .unwrap_or_else(|| quote! { None::<std::string::String> });

    let category = arguments
        .get("category")
        .map(|value| quote! { Some(#value) })
        .unwrap_or_else(|| quote! { None::<std::string::String> });

    let description = arguments
        .get("description")
        .map(|value| quote! { Some(#value) })
        .unwrap_or_else(|| quote! { None::<std::string::String> });

    TokenStream::from(quote! {
        // use pyo3::prelude::*;
        use pyo3::IntoPyObjectExt;

        #[derive(std::default::Default)]
        #input_struct

        inventory::submit! {
            comfy_builder_core::registry::NodeRegistration::new::<#ident>()
        }

        #[pyo3::pyfunction]
        #[pyo3(signature = (class, **kwargs))]
        fn __execute<'py>(
            class: pyo3::Bound<'py, pyo3::types::PyType>,
            kwargs: Option<pyo3::Bound<'py, pyo3::types::PyDict>>,
        ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
            use comfy_builder_core::node::Out;

            let instance = #ident::new();
            let input = instance.initialize_inputs(kwargs.into())?;
            let output = instance.execute(input).map_err(|error| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                    "Failed to execute node.\n\n{}", error
                ))
            })?;

            let python = class.py();
            let node_output = python
                .import(format!("comfy_api.{}", crate::__injected::API_VERSION))?
                .getattr("io")?
                .getattr("NodeOutput")?;

            node_output.call1(output.to_schema(python)?)
        }

        #[pyfunction]
        fn __define_schema<'py>(class: pyo3::Bound<'py, pyo3::types::PyType>) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
            use comfy_builder_core::node::In;
            use comfy_builder_core::node::Out;

            let python = class.py();
            let io = python
                .import(format!("comfy_api.{}", crate::__injected::API_VERSION))?
                .getattr("io")?;

            let inputs = <#ident as comfy_builder_core::node::Node>::In::blueprints(python, &io)?;
            let outputs = <#ident as comfy_builder_core::node::Node>::Out::blueprints(python, &io)?;
            let is_list = <#ident as comfy_builder_core::node::Node>::In::is_list();

            let kwargs = pyo3::types::PyDict::new(python);

            kwargs.set_item("node_id", #node_id)?;
            kwargs.set_item("display_name", #display_name)?;

            if let Some(category) = #category {
                kwargs.set_item("category", category)?;
            }

            if let Some(description) = #description {
                kwargs.set_item("description", description)?;
            }

            kwargs.set_item("is_input_list", is_list)?;

            kwargs.set_item("inputs", inputs)?;
            kwargs.set_item("outputs", outputs)?;

            io.getattr("Schema")?.call((), Some(&kwargs))
        }

        impl comfy_builder_core::node::NodeFunctionProvider for #ident {

            fn define_fn(python: pyo3::Python) -> pyo3::PyResult<pyo3::Bound<pyo3::types::PyCFunction>> {
                wrap_pyfunction!(__define_schema, python)
            }

            fn execute_fn(python: pyo3::Python) -> pyo3::PyResult<pyo3::Bound<pyo3::types::PyCFunction>> {
                wrap_pyfunction!(__execute, python)
            }

        }

    })
}
