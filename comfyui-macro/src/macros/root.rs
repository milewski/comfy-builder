use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn node(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as DeriveInput);
    let struct_name = &input_struct.ident;

    TokenStream::from(quote! {
        #[pyo3::pyclass]
        #[derive(std::default::Default)]
        #input_struct

        #[pyo3::pymethods]
        impl #struct_name {
            #[new]
            fn initialize() -> Self {
                Self::new()
            }

            #[classattr]
            #[pyo3(name = "DESCRIPTION")]
            fn description() -> &'static str {
                Self::DESCRIPTION
            }

            #[classattr]
            #[pyo3(name = "FUNCTION")]
            fn function() -> &'static str {
                "run"
            }

            #[classmethod]
            #[pyo3(name = "INPUT_TYPES")]
            fn input_types<'a>(cls: &Bound<'a, PyType>) -> PyResult<Bound<'a, PyDict>> {
                <<Self as CustomNode>::In as InputPort<'a>>::get_inputs(cls.py())
            }

            #[classattr]
            #[pyo3(name = "RETURN_TYPES")]
            fn return_types<'a>(py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
                <<Self as CustomNode>::Out as OutputPort<'a>>::values().into_pyobject(py)
            }

            #[classattr]
            #[pyo3(name = "RETURN_NAMES")]
            fn return_names<'a>(py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
                <<Self as CustomNode>::Out as OutputPort<'a>>::keys().into_pyobject(py)
            }

            #[classattr]
            #[pyo3(name = "CATEGORY")]
            fn category() -> &'static str {
                Self::CATEGORY
            }

            #[classmethod]
            #[pyo3(signature = (**kwargs))]
            pub fn run<'a>(py: &'a pyo3::Bound<pyo3::types::PyType>, kwargs: Option<&pyo3::Bound<pyo3::types::PyDict>>) -> impl pyo3::IntoPyObject<'a> {
                dbg!(kwargs);
                let instance = Self::new();
                let output = instance.execute(instance.initialize_input(kwargs));
                output.into_pyobject(py.py()).unwrap()
            }
        }
    })
}
