use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, Data, Fields, Type, Ident,
    Attribute, Expr, Lit
};

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

            // Default INPUT_TYPES implementation
            #[classmethod]
            #[pyo3(name = "INPUT_TYPES")]
            fn py_input_types<'a>(cls: &pyo3::Bound<'a, pyo3::types::PyType>) -> pyo3::Bound<'a, pyo3::types::PyDict> {
                let py = cls.py();
                let out = pyo3::types::PyDict::new(py);
                let required = pyo3::types::PyDict::new(py);
                
                required.set_item("image", ("IMAGE",)).unwrap();
                out.set_item("required", required).unwrap();
                out
            }

            #[classattr]
            #[pyo3(name = "RETURN_TYPES")]
            fn return_types(py: pyo3::Python) -> pyo3::PyResult<pyo3::Bound<pyo3::types::PyTuple>> {
                ("IMAGE",).into_pyobject(py)
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
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(InputConfig, attributes(number))]
pub fn input_derive(input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as DeriveInput);
    let struct_name = &input_struct.ident;

    // Extract input fields with their attributes
    let input_fields_info = extract_input_fields_info(&input_struct);

    // Generate the INPUT_TYPES method based on field attributes
    let input_types_method = generate_input_types_method(&input_fields_info);

    // Generate the to_dict method implementation
    let to_dict_method = generate_to_dict_method(&input_struct);

    let expanded = quote! {
        impl #struct_name {
            #input_types_method
        }

        impl<'a> InputPort<'a> for #struct_name {
            fn to_dict(&self) -> pyo3::PyResult<pyo3::types::PyDict> {
                #to_dict_method
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug, Clone)]
struct NumberConfig {
    min: Option<Expr>,
    max: Option<Expr>,
    step: Option<Expr>,
    default: Option<Expr>,
}

#[derive(Debug, Clone)]
struct FieldInfo {
    name: Ident,
    field_type: Type,
    input_type: String, // "INT", "FLOAT", "IMAGE", etc.
    number_config: Option<NumberConfig>,
}

fn extract_input_fields_info(input: &DeriveInput) -> Vec<FieldInfo> {
    let mut fields_info = Vec::new();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                let field_name = field.ident.as_ref().unwrap().clone();
                let field_type = field.ty.clone();

                // Determine input type based on field type
                let input_type = determine_input_type(&field.ty);

                // Extract number attribute
                let mut number_config = None;

                for attr in &field.attrs {
                    if attr.path().is_ident("number") {
                        let mut config = NumberConfig {
                            min: None,
                            max: None,
                            step: None,
                            default: None,
                        };

                        if let Err(_) = attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("min") {
                                config.min = Some(meta.value()?.parse()?);
                            } else if meta.path.is_ident("max") {
                                config.max = Some(meta.value()?.parse()?);
                            } else if meta.path.is_ident("step") {
                                config.step = Some(meta.value()?.parse()?);
                            } else if meta.path.is_ident("default") {
                                config.default = Some(meta.value()?.parse()?);
                            }
                            Ok(())
                        }) {
                            // If parsing fails, continue with empty config
                        }

                        number_config = Some(config);
                    }
                }

                fields_info.push(FieldInfo {
                    name: field_name,
                    field_type,
                    input_type,
                    number_config,
                });
            }
        }
    }

    fields_info
}

fn generate_to_dict_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            let field_assignments: Vec<_> = fields_named.named.iter()
                .filter_map(|field| {
                    field.ident.as_ref().map(|field_name| {
                        let field_name_str = field_name.to_string();
                        quote! {
                            dict.set_item(#field_name_str, self.#field_name.clone())?;
                        }
                    })
                })
                .collect();

            quote! {
                let py = pyo3::Python::assume_gil_acquired();
                let dict = pyo3::types::PyDict::new(py);
                
                #(#field_assignments)*
                
                Ok(dict)
            }
        } else {
            quote! {
                let py = pyo3::Python::assume_gil_acquired();
                Ok(pyo3::types::PyDict::new(py))
            }
        }
    } else {
        quote! {
            let py = pyo3::Python::assume_gil_acquired();
            Ok(pyo3::types::PyDict::new(py))
        }
    }
}

fn determine_input_type(field_type: &Type) -> String {
    // This is a simplified type detection
    let type_str = quote! { #field_type }.to_string();

    if type_str.contains("TensorWrapper") || type_str.contains("IMAGE") {
        "IMAGE".to_string()
    } else if type_str.contains("usize") || type_str.contains("i32") || type_str.contains("i64") {
        "INT".to_string()
    } else if type_str.contains("f32") || type_str.contains("f64") {
        "FLOAT".to_string()
    } else {
        "STRING".to_string()
    }
}

fn generate_input_types_method(fields_info: &[FieldInfo]) -> proc_macro2::TokenStream {
    let field_configs: Vec<_> = fields_info.iter().map(|field_info| {
        let field_name = &field_info.name;
        let field_name_str = field_info.name.to_string();
        let input_type = &field_info.input_type;

        if let Some(config) = &field_info.number_config {
            let default_item = config.default.as_ref().map(|default| {
                quote! { config_dict.set_item("default", #default).unwrap(); }
            });

            let min_item = config.min.as_ref().map(|min| {
                quote! { config_dict.set_item("min", #min).unwrap(); }
            });

            let max_item = config.max.as_ref().map(|max| {
                quote! { config_dict.set_item("max", #max).unwrap(); }
            });

            let step_item = config.step.as_ref().map(|step| {
                quote! { config_dict.set_item("step", #step).unwrap(); }
            });

            quote! {
                {
                    let config_dict = pyo3::types::PyDict::new(py);
                    
                    #default_item
                    #min_item
                    #max_item
                    #step_item
                    
                    required.set_item(#field_name_str, (#input_type, config_dict)).unwrap();
                }
            }
        } else {
            quote! {
                required.set_item(#field_name_str, (#input_type,)).unwrap();
            }
        }
    }).collect();

    quote! {
        pub fn input_types_for_comfyui(py: pyo3::Python) -> pyo3::Bound<pyo3::types::PyDict> {
            let out = pyo3::types::PyDict::new(py);
            let required = pyo3::types::PyDict::new(py);

            #(#field_configs)*

            out.set_item("required", required).unwrap();
            out
        }
    }
}