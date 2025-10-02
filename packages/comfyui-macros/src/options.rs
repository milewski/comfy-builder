use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Result};

pub trait Options: Parse {
    fn generate_token_stream(&self) -> TokenStream;
}

#[derive(Default)]
pub struct AnyOption {
    options: HashMap<String, Expr>,
}

impl Parse for AnyOption {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values = AnyOption::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let _ = input.parse::<syn::Token![=]>()?;
            let value: Expr = input.parse()?;

            values.options.insert(key.to_string(), value);

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(values)
    }
}

impl Options for AnyOption {
    fn generate_token_stream(&self) -> TokenStream {
        let tokens: Vec<proc_macro2::TokenStream> = self
            .options
            .iter()
            .map(|(key, value)| quote! { dict.set_item(#key, #value)?; })
            .collect();

        TokenStream::from(quote! {
            #(#tokens)*
        })
    }
}
