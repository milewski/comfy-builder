use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn node(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as DeriveInput);
    let ident = &input_struct.ident;

    TokenStream::from(quote! {
        use pyo3::prelude::*;

        inventory::submit! {
            comfy_builder_core::registry::NodeRegistration::new::<#ident>()
        }

        #[pyo3::pyclass]
        #[derive(std::default::Default)]
        #input_struct

        #[pyo3::pymethods]
        impl #ident {
            #[new]
            fn __initialize() -> Self {
                Self::new()
            }

            #[classattr]
            #[pyo3(name = "DESCRIPTION")]
            fn __description() -> &'static str {
                Self::DESCRIPTION
            }

            #[classattr]
            #[pyo3(name = "FUNCTION")]
            fn __function() -> &'static str {
                "__run"
            }

            #[classmethod]
            #[pyo3(name = "INPUT_TYPES")]
            fn __input_types<'a>(cls: &pyo3::Bound<'a, pyo3::types::PyType>) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::types::PyDict>> {
                <<Self as comfy_builder_core::node::Node>::In as comfy_builder_core::node::InputPort<'a>>::get_inputs(cls.py())
            }

            #[classattr]
            #[pyo3(name = "RETURN_TYPES")]
            fn __return_types<'a>(py: pyo3::Python<'a>) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::PyAny>> {
                <<Self as comfy_builder_core::node::Node>::Out as comfy_builder_core::node::OutputPort<'a>>::values().into_pyobject(py)
            }

            #[classattr]
            #[pyo3(name = "RETURN_NAMES")]
            fn __return_names<'a>(py: pyo3::Python<'a>) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::PyAny>> {
                <<Self as comfy_builder_core::node::Node>::Out as comfy_builder_core::node::OutputPort<'a>>::keys().into_pyobject(py)
            }

            #[classattr]
            #[pyo3(name = "CATEGORY")]
            fn __category() -> &'static str {
                Self::CATEGORY
            }

            #[classmethod]
            #[pyo3(signature = (**kwargs))]
            pub fn __run<'a>(py: &'a pyo3::Bound<pyo3::types::PyType>, kwargs: std::option::Option<&pyo3::Bound<pyo3::types::PyDict>>) -> pyo3::PyResult<impl pyo3::IntoPyObject<'a>> {
                let instance = Self::new();
                let output = instance.execute(instance.initialize_input(kwargs)).map_err(|error| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!("Execution failed: {}", error))
                })?;

                Ok(output.into_pyobject(py.py())?)
            }
        }
    })
}
