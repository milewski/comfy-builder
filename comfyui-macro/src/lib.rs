use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

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

#[proc_macro_derive(InputDerive, attributes(attribute))]
pub fn input_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("InputDerive only works on structs with named fields"),
        },
        _ => panic!("InputDerive only works on structs"),
    };

    let mut attributes: Vec<proc_macro2::TokenStream> = vec![];
    let mut decoders: Vec<proc_macro2::TokenStream> = vec![];

    for field in fields {

        if let (Some(kind), Some(attribute)) = (extract_field_info(&field.ty), &field.ident) {

            let (bucket, ident) = match kind {
                IdentKind::Required(ident) => (quote! { required }, ident),
                IdentKind::Optional(ident) => (quote! { optional }, ident),
            };

            attributes.push(quote! {

                let dict = pyo3::types::PyDict::new(py);

                if matches!(stringify!(#ident), "u8" | "u16" | "u32" | "u128" | "u64" | "usize" | "i8" | "i16" | "i32" | "i128" | "i64" | "isize") {

                    dict.set_item("default", 456)?;
                    dict.set_item("min", 123)?;
                    dict.set_item("max", 456)?;
                    dict.set_item("step", 1)?;

                    #bucket.set_item(stringify!(#attribute), (crate::node::DataType::from(stringify!(#ident)).to_string(), dict))?;

                } else if matches!(stringify!(#ident), "bool") {

                    dict.set_item("default", true)?;
                    dict.set_item("label_on", "&self.label_on")?;
                    dict.set_item("label_off", "&self.label_off")?;

                    #bucket.set_item(stringify!(#attribute), (crate::node::DataType::from(stringify!(#ident)).to_string(), dict))?;

                } else if std::any::TypeId::of::<#ident>() == std::any::TypeId::of::<String>() {

                    dict.set_item("placeholder", "placeholder")?;
                    dict.set_item("multiline", true)?;
                    dict.set_item("default", "hello world")?;

                    #bucket.set_item(stringify!(#attribute), (crate::node::DataType::from(stringify!(#ident)).to_string(), dict))?;

                } else if std::any::TypeId::of::<#ident>() == std::any::TypeId::of::<crate::tensor::TensorWrapper>() {

                    #bucket.set_item(stringify!(#attribute), (crate::node::DataType::from(stringify!(#ident)).to_string(), dict))?;

                }

                if matches!(stringify!(#ident), "HiddenUniqueId") {
                    hidden.set_item(stringify!(#attribute), "UNIQUE_ID")?;
                }

            });

            let extract_logic = quote! {
                kwargs
                    .get_item(stringify!(#attribute))
                    .ok()
                    .flatten()
                    .and_then(|value| value.extract::<#ident>().ok())
            };

            decoders.push(match kind {
                IdentKind::Required(_) => quote! { #attribute: #extract_logic.unwrap(), },
                IdentKind::Optional(_) => quote! { #attribute: #extract_logic, },
            });

        }

    }

    TokenStream::from(quote! {

        impl<'a> crate::node::InputPort<'a> for #name {
            fn get_inputs(py: pyo3::Python<'a>) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::types::PyDict>> {
                let output = pyo3::types::PyDict::new(py);
                let required = pyo3::types::PyDict::new(py);
                let optional = pyo3::types::PyDict::new(py);
                let hidden = pyo3::types::PyDict::new(py);

                #(#attributes)*

                output.set_item("required", required)?;
                output.set_item("optional", optional)?;
                output.set_item("hidden", hidden)?;

                pyo3::PyResult::Ok(output)
            }
        }

        impl<'a> std::convert::From<&'a pyo3::Bound<'a, pyo3::types::PyDict>> for #name {
            fn from(kwargs: &'a pyo3::Bound<'a, pyo3::types::PyDict>) -> Self {
                Self {
                    #(#decoders)*
                }
            }
        }

    })
}

#[derive(Debug)]
enum IdentKind<T> {
    Required(T),
    Optional(T),
}

fn extract_field_info(ty: &Type) -> Option<IdentKind<&Ident>> {
    if let Type::Path(path) = ty {
        if let Some(ident) = path.path.get_ident() {
            return Some(IdentKind::Required(ident));
        }

        if path.path.segments.len() == 1 && path.path.segments[0].ident == "Option" {
            if let PathArguments::AngleBracketed(angle) = &path.path.segments[0].arguments {
                if let Some(GenericArgument::Type(inner_ty)) = angle.args.first() {
                    if let Type::Path(inner_path) = inner_ty {
                        if let Some(ident) = inner_path.path.get_ident() {
                            return Some(IdentKind::Optional(ident));
                        }
                    }
                }
            }
        }
    }
    None
}
