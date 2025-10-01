use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ExprLit, Lit, Result};

pub trait Options: Parse {
    fn generate_token_stream(&self) -> TokenStream;
}

#[derive(Debug, Default)]
pub struct IntOptions {
    min: Option<usize>,
    max: Option<usize>,
    step: Option<usize>,
    default: Option<usize>,
}

#[derive(Debug, Default)]
pub struct BoolOptions {
    label_on: Option<String>,
    label_off: Option<String>,
    default: Option<bool>,
}

#[derive(Debug, Default)]
pub struct StringOption {
    multiline: Option<bool>,
    default: Option<String>,
    placeholder: Option<String>,
}

#[derive(Debug)]
pub struct EnumOption(bool);

impl Parse for EnumOption {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Some((ident, _)) = input.cursor().ident() {
            if ident.to_string().as_str() == "enum" {
                return Ok(EnumOption(true));
            }
        }

        Ok(EnumOption(false))
    }
}

impl Options for EnumOption {
    fn generate_token_stream(&self) -> TokenStream {
        TokenStream::from(quote! {})
    }
}

impl Parse for IntOptions {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values = IntOptions::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            let value: Expr = input.parse()?;

            match key.to_string().as_str() {
                "min" => values.min = parse_lit_int(&value)?,
                "max" => values.max = parse_lit_int(&value)?,
                "step" => values.step = parse_lit_int(&value)?,
                "default" => values.default = parse_lit_int(&value)?,
                _ => return Err(syn::Error::new_spanned(key, "unknown attribute key")),
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(values)
    }
}

impl Parse for BoolOptions {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values = BoolOptions::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            let value: Expr = input.parse()?;

            match key.to_string().as_str() {
                "label_on" => values.label_on = parse_lit_str(&value)?,
                "label_off" => values.label_off = parse_lit_str(&value)?,
                "default" => values.default = parse_lit_bool(&value)?,
                _ => return Err(syn::Error::new_spanned(key, "unknown attribute key")),
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(values)
    }
}

impl Parse for StringOption {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values = StringOption::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            let value: Expr = input.parse()?;

            match key.to_string().as_str() {
                "placeholder" => values.placeholder = parse_lit_str(&value)?,
                "multiline" => values.multiline = parse_lit_bool(&value)?,
                "default" => values.default = parse_lit_str(&value)?,
                _ => return Err(syn::Error::new_spanned(key, "unknown attribute key")),
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(values)
    }
}

impl Options for IntOptions {
    fn generate_token_stream(&self) -> TokenStream {
        let min = self
            .min
            .and_then(|min| quote! { dict.set_item("min", #min)?; }.into())
            .unwrap_or_else(|| quote! {});
        let max = self
            .max
            .and_then(|max| quote! { dict.set_item("max", #max)?; }.into())
            .unwrap_or_else(|| quote! {});
        let step = self
            .step
            .and_then(|step| quote! { dict.set_item("step", #step)?; }.into())
            .unwrap_or_else(|| quote! {});
        let default = self
            .default
            .and_then(|default| quote! { dict.set_item("default", #default)?; }.into())
            .unwrap_or_else(|| quote! {});

        TokenStream::from(quote! {
            #min
            #max
            #step
            #default
        })
    }
}

impl Options for BoolOptions {
    fn generate_token_stream(&self) -> TokenStream {
        let label_on = self
            .label_on
            .as_ref()
            .map(|label_on| quote! { dict.set_item("label_on", #label_on)?; })
            .unwrap_or_default();

        let label_off = self
            .label_off
            .as_ref()
            .map(|label_off| quote! { dict.set_item("label_off", #label_off)?; })
            .unwrap_or_default();

        let default = self
            .default
            .map(|default| quote! { dict.set_item("default", #default)?; })
            .unwrap_or_default();

        TokenStream::from(quote! {
            #label_on
            #label_off
            #default
        })
    }
}

impl Options for StringOption {
    fn generate_token_stream(&self) -> TokenStream {
        let multiline = self
            .multiline
            .map(|multiline| quote! { dict.set_item("multiline", #multiline)?; })
            .unwrap_or_default();

        let default = self
            .default
            .as_ref()
            .map(|default| quote! { dict.set_item("default", #default)?; })
            .unwrap_or_default();

        let placeholder = self
            .placeholder
            .as_ref()
            .map(|placeholder| quote! { dict.set_item("placeholder", #placeholder)?; })
            .unwrap_or_default();

        TokenStream::from(quote! {
            #multiline
            #placeholder
            #default
        })
    }
}

fn parse_lit_str(expr: &Expr) -> Result<Option<String>> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(lit_str),
        ..
    }) = expr
    {
        Ok(Some(lit_str.value()))
    } else {
        Ok(None)
    }
}

fn parse_lit_bool(expr: &Expr) -> Result<Option<bool>> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Bool(lit_bool),
        ..
    }) = expr
    {
        Ok(Some(lit_bool.value))
    } else {
        Ok(None)
    }
}

fn parse_lit_int(expr: &Expr) -> Result<Option<usize>> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Int(lit_int),
        ..
    }) = expr
    {
        lit_int
            .base10_parse::<usize>()
            .map(Some)
            .map_err(|e| syn::Error::new_spanned(lit_int, e))
    } else {
        Ok(None)
    }
}
