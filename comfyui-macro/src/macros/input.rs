use crate::options::{BoolOptions, IntOptions, Options, StringOption};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, GenericArgument, Meta, PathArguments, Type,
};

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

fn get_options(ident: &Ident, field: &Field) -> proc_macro2::TokenStream {
    field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("attribute"))
        .map(|attr| match &attr.meta {
            Meta::List(meta_list) => Some(meta_list),
            _ => None,
        })
        .flatten()
        .map(|meta| meta.tokens.clone())
        .and_then(|tokens| match ident.to_string().as_str() {
            "u8" | "u16" | "u32" | "u128" | "u64" | "usize" | "i8" | "i16" | "i32" | "i128"
            | "i64" | "isize" => syn::parse2::<IntOptions>(tokens)
                .ok()
                .map(|option| option.generate_token_stream()),
            "bool" => syn::parse2::<BoolOptions>(tokens)
                .ok()
                .map(|option| option.generate_token_stream()),
            "String" => syn::parse2::<StringOption>(tokens)
                .ok()
                .map(|option| option.generate_token_stream()),
            _ => unreachable!("could not handle ident type {}", ident),
        })
        .unwrap_or_default()
        .into()
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

    let mut attributes: Vec<proc_macro2::TokenStream> = vec![];
    let mut decoders: Vec<proc_macro2::TokenStream> = vec![];

    for field in fields {
        if let (Some(kind), Some(attribute)) = (extract_field_info(&field.ty), &field.ident) {
            let (bucket, ident) = match kind {
                IdentKind::Required(ident) => (quote! { required }, ident),
                IdentKind::Optional(ident) => (quote! { optional }, ident),
            };

            let options = get_options(&ident, &field);

            attributes.push(quote! {

                let dict = pyo3::types::PyDict::new(py);

                if matches!(stringify!(#ident), "u8" | "u16" | "u32" | "u128" | "u64" | "usize" | "i8" | "i16" | "i32" | "i128" | "i64" | "isize") {

                    #options

                    #bucket.set_item(stringify!(#attribute), (crate::node::DataType::from(stringify!(#ident)).to_string(), dict))?;

                } else if matches!(stringify!(#ident), "bool") {

                    #options

                    #bucket.set_item(stringify!(#attribute), (crate::node::DataType::from(stringify!(#ident)).to_string(), dict))?;

                } else if std::any::TypeId::of::<#ident>() == std::any::TypeId::of::<String>() {

                    #options

                    #bucket.set_item(stringify!(#attribute), (crate::node::DataType::from(stringify!(#ident)).to_string(), dict))?;

                } else if std::any::TypeId::of::<#ident>() == std::any::TypeId::of::<crate::tensor::TensorWrapper>() {

                    #bucket.set_item(stringify!(#attribute), (crate::node::DataType::from(stringify!(#ident)).to_string(), dict))?;

                }

                if std::any::TypeId::of::<#ident>() == std::any::TypeId::of::<crate::attributes::UniqueId>() {
                    hidden.set_item(stringify!(#attribute), "UNIQUE_ID")?;
                }

                if std::any::TypeId::of::<#ident>() == std::any::TypeId::of::<crate::attributes::Prompt>() {
                    hidden.set_item(stringify!(#attribute), "PROMPT")?;
                }

                if std::any::TypeId::of::<#ident>() == std::any::TypeId::of::<crate::attributes::ExtraPngInfo>() {
                    hidden.set_item(stringify!(#attribute), "EXTRA_PNGINFO")?;
                }

                if std::any::TypeId::of::<#ident>() == std::any::TypeId::of::<crate::attributes::DynPrompt>() {
                    hidden.set_item(stringify!(#attribute), "DYNPROMPT")?;
                }

            });

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
