use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_attribute]
pub fn node(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as DeriveInput);
    let struct_name = &input_struct.ident;

    TokenStream::from(quote! {
        #[pyo3::pyclass]
        #[derive(Default)]
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
                let instance = Self::new();
                let output = instance.execute(instance.initialize_input(kwargs));
                output.into_pyobject(py.py()).unwrap()
            }
        }
    })
}

#[proc_macro_derive(OutputPort)]
pub fn output_port_derive(input: TokenStream) -> TokenStream {
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
            let field_name = field.ident.unwrap();

            // Determine the data type based on field type
            let data_type = match field.ty {
                Type::Path(type_path) => {
                    let type_str = format!("{}", quote!(#type_path));
                    quote!(DataType::from(#type_str))
                }
                _ => quote!(DataType::Unknown),
            };

            field_inserts.push(quote! {
                map.insert(stringify!(#field_name), #data_type);
            });

            into_pyobject_fields.push(quote! {
                self.#field_name.into_pyobject(py)?,
            });
        }
    }

    TokenStream::from(quote! {
        impl<'a> OutputPort<'a> for #name {
            fn get_outputs() -> indexmap::IndexMap<&'static str, DataType> {
                let mut map = indexmap::IndexMap::new();
                #(#field_inserts)*
                map
            }
        }

        impl<'py> IntoPyObject<'py> for #name {
            type Target = PyTuple;
            type Output = Bound<'py, Self::Target>;
            type Error = PyErr;

            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                (#(#into_pyobject_fields)*).into_pyobject(py)
            }
        }
    })
}
