use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

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

    let into_pyobject_body = if into_pyobject_fields.is_empty() {
        quote! { Ok(PyTuple::empty(py).into_pyobject(py)?) }
    } else {
        quote! { (#(#into_pyobject_fields)*).into_pyobject(py) }
    };

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
                #into_pyobject_body
            }
        }
    })
}
