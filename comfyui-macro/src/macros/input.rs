use crate::macros::{extract_field_info, numeric_types, IdentKind};
use crate::options::{BoolOptions, IntOptions, Options, StringOption};
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields};

fn get_options(ident: &Ident, field: &Field) -> proc_macro2::TokenStream {
    field
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("attribute"))
        .map(|attribute| attribute.meta.require_list().ok())
        .flatten()
        .and_then(|attribute| match ident.to_string().as_str() {
            numeric_types!() => attribute
                .parse_args::<IntOptions>()
                .ok()
                .map(|option| option.generate_token_stream()),
            "bool" => attribute
                .parse_args::<BoolOptions>()
                .ok()
                .map(|option| option.generate_token_stream()),
            "String" => attribute
                .parse_args::<StringOption>()
                .ok()
                .map(|option| option.generate_token_stream()),
            _ => None,
        })
        .unwrap_or_default()
        .into()
}

fn is_enum(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .find(|attribute| attribute.meta.path().is_ident("attribute"))
        .and_then(|attribute| {
            attribute.meta.require_list().ok().and_then(|meta| {
                meta.tokens
                    .clone()
                    .into_iter()
                    .find(|token| matches!(token, TokenTree::Ident(ident) if ident == "enum"))
            })
        })
        .is_some()
}

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

    let mut decoders: Vec<proc_macro2::TokenStream> = vec![];
    let mut attributes: Vec<proc_macro2::TokenStream> = vec![];

    for field in fields {
        if let (Some(kind), Some(attribute)) = (extract_field_info(&field.ty), &field.ident) {
            let (bucket, ident) = match kind {
                IdentKind::Required(ident) => (quote! { required }, ident),
                IdentKind::Optional(ident) => (quote! { optional }, ident),
            };

            let options = get_options(&ident, &field);
            let ident_str = ident.to_string();

            if matches!(ident_str.as_str(), numeric_types!(signed))
                || matches!(ident_str.as_str(), numeric_types!(unsigned))
                || matches!(ident_str.as_str(), "bool")
                || matches!(ident_str.as_str(), "String")
                || matches!(ident_str.as_str(), "Tensor")
                || matches!(ident_str.as_str(), "Mask")
            {
                attributes.push(quote! {
                    let dict = pyo3::types::PyDict::new(py);
                    #options
                    #bucket.set_item(stringify!(#attribute), (crate::node::DataType::from(stringify!(#ident)).to_string(), dict))?;
                })
            }

            if matches!(
                ident_str.as_str(),
                "UniqueId" | "Prompt" | "ExtraPngInfo" | "DynPrompt"
            ) {
                let token = match ident_str.as_str() {
                    "UniqueId" => "UNIQUE_ID",
                    "Prompt" => "PROMPT",
                    "ExtraPngInfo" => "EXTRA_PNGINFO",
                    "DynPrompt" => "DYNPROMPT",
                    _ => unreachable!(),
                };

                attributes.push(quote! {
                    hidden.set_item(stringify!(#attribute), #token)?;
                })
            }

            if is_enum(&field) {
                attributes.push(quote! {
                    #bucket.set_item(stringify!(#attribute), (#ident::variants(),))?;
                })
            }

            let extract_logic = quote! {
                kwargs
                    .and_then(|kwargs| kwargs.get_item(stringify!(#attribute)).ok())
                    .flatten()
                    .and_then(|value| value.extract::<#ident>().ok())
            };

            decoders.push(match kind {
                IdentKind::Required(_) => {
                    quote! { #attribute: #extract_logic.expect("Unable to retrieve attribute."), }
                }
                IdentKind::Optional(_) => quote! { #attribute: #extract_logic, },
            });
        }
    }

    TokenStream::from(quote! {

        impl<'a> crate::node::InputPort<'a> for #name {

            fn get_inputs(py: pyo3::Python<'a>) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::types::PyDict>> {

                use crate::node::EnumVariants;

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

        impl<'a> std::convert::From<Option<&'a pyo3::Bound<'a, pyo3::types::PyDict>>> for #name {
            fn from(kwargs: Option<&'a pyo3::Bound<'a, pyo3::types::PyDict>>) -> Self {
                #name {
                    #(#decoders)*
                }
            }
        }

    })
}
