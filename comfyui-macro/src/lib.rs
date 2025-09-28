use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn node(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as DeriveInput);
    let struct_name = &input_struct.ident;

    let expanded = quote! {
            #[pyo3::pyclass]
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
                fn input_types<'a>(cls: &Bound<'a, PyType>) -> Bound<'a, PyDict> {
                    let py = cls.py();
                    let out = PyDict::new(py);
                    let required = PyDict::new(py);
                    // image
                    required.set_item("image", ("IMAGE",)).unwrap();
                    // width
                    let width = PyDict::new(py);
                    width.set_item("default", 1024).unwrap();
                    width.set_item("min", 0).unwrap();
                    // width.set_item("max", 1024).unwrap();
                    width.set_item("step", 1).unwrap();
                    required.set_item("width", ("INT", width)).unwrap();
                    // width
                    let height = PyDict::new(py);
                    height.set_item("default", 1024).unwrap();
                    height.set_item("min", 0).unwrap();
                    // height.set_item("max", 1024).unwrap();
                    height.set_item("step", 1).unwrap();
                    required.set_item("height", ("INT", height)).unwrap();
                    out.set_item("required", required).unwrap();
                    out
                }

                #[classattr]
                #[pyo3(name = "RETURN_TYPES")]
                fn return_types(py: Python) -> PyResult<Bound<PyTuple>> {
                    ("INT", "IMAGE",).into_pyobject(py)
                }

                #[classattr]
                #[pyo3(name = "CATEGORY")]
                fn category() -> &'static str {
                    Self::CATEGORY
                }

                #[classmethod]
                #[pyo3(signature = (**kwargs))]
                pub fn run<'a>(py: &'a Bound<PyType>, kwargs: Option<&Bound<PyDict>>) -> impl IntoPyObject<'a> {
                    let instance = Self::new();
                    let output = instance.execute(instance.initialize_input(kwargs));
                    output.into_pyobject(py.py()).unwrap()
                }
            }
        };

    TokenStream::from(expanded)
}
