use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{
    Expr, ExprLit, Field, GenericArgument, Lit, Path, PathArguments, PathSegment, Type, TypePath,
};

pub struct FieldHelper<'a> {
    field: &'a Field,
}

impl<'a> From<&'a Field> for FieldHelper<'a> {
    fn from(field: &'a Field) -> Self {
        FieldHelper { field }
    }
}

impl<'a> FieldHelper<'a> {
    pub fn named_attributes(&self) -> HashMap<String, &Lit> {
        self.field
            .attrs
            .iter()
            .filter_map(|attr| attr.meta.require_name_value().ok())
            .flat_map(|meta| {
                meta.path.require_ident().map(|ident| {
                    (
                        ident.to_string(),
                        match &meta.value {
                            Expr::Lit(ExprLit { lit, .. }) => lit,
                            _ => unreachable!(),
                        },
                    )
                })
            })
            .map(|(key, value)| (if key == "doc" { "tooltip".into() } else { key }, value))
            .collect()
    }

    pub fn property_ident(&self) -> &Ident {
        self.field.ident.as_ref().unwrap()
    }

    pub fn inner_value_type_call(&self) -> Option<proc_macro2::TokenStream> {
        if let Type::Path(type_path) = &self.field.ty
            && let Some(segment) = type_path.path.segments.first()
                // && segment.ident == "Vec"
                && let PathArguments::AngleBracketed(args) = &segment.arguments
                && let Some(GenericArgument::Type(ty)) = args.args.first()
        {
            return Some(extract_full_type_as_static_call(ty));
        }
        None
    }

    pub fn value_type_call(&self) -> TokenStream {
        extract_full_type_as_static_call(&self.field.ty)
    }

    /// Return the complete ident as defined on the struct side
    pub fn output_ident(&self, force_vector: bool) -> TokenStream {
        let ident = &self.field.ty;
        let is_optional = self.is_optional();
        let mut content = quote! { #ident };

        if is_optional {
            content = match self.inner_value_type_call() {
                None => content,
                Some(inner) => quote! { #inner },
            };
        }

        let content = match self.is_wrapped_by_vector() {
            true => quote! { #content },
            false => {
                if force_vector {
                    quote! { Vec<#content> }
                } else {
                    quote! { #content }
                }
            }
        };

        if self.is_optional() {
            return quote! { Option<#content> };
        }

        content
    }

    pub fn value_ident(&self) -> &Ident {
        match &self.field.ty {
            Type::Path(type_path) => match type_path.path.get_ident() {
                Some(ident) => ident,
                None => match split_inner_ident(type_path) {
                    Some((_, right)) => right,
                    None => unreachable!(),
                },
            },
            _ => unreachable!(),
        }
    }

    pub fn value_ident_wrapper(&self) -> Option<&Ident> {
        match &self.field.ty {
            Type::Path(type_path) => match type_path.path.get_ident() {
                Some(_) => None,
                None => match split_inner_ident(type_path) {
                    Some((left, _)) => Some(left),
                    None => unreachable!(),
                },
            },
            _ => unreachable!(),
        }
    }

    pub fn is_wrapped_by_vector(&self) -> bool {
        self.value_ident_wrapper()
            .map(|ident| ident.to_string().as_str() == "Vec")
            .unwrap_or_default()
    }
    
    pub fn is_string(&self) -> bool {
        let kind_str = self.value_ident().to_string();

        matches!(kind_str.as_str(), "String")
    }
    
    pub fn is_required(&self) -> bool {
        !self.is_optional()
    }

    pub fn is_optional(&self) -> bool {
        self.value_ident_wrapper()
            .map(|ident| ident.to_string().as_str() == "Option")
            .unwrap_or_default()
    }
}

fn split_inner_ident(path: &TypePath) -> Option<(&Ident, &Ident)> {
    if path.path.segments.len() == 1
        && let PathArguments::AngleBracketed(angle) = &path.path.segments[0].arguments
        && let Some(GenericArgument::Type(inner_ty)) = angle.args.first()
        && let Type::Path(inner_path) = inner_ty
        && let Some(ident) = inner_path.path.get_ident()
    {
        return Some((&path.path.segments[0].ident, ident));
    }

    None
}

fn extract_full_type_as_static_call(value: &Type) -> TokenStream {
    match value {
        Type::Path(type_path) => {
            let path_without_args: Path = Path {
                leading_colon: type_path.path.leading_colon,
                segments: type_path
                    .path
                    .segments
                    .iter()
                    .map(|segment| PathSegment {
                        ident: segment.ident.clone(),
                        arguments: PathArguments::None,
                    })
                    .collect(),
            };

            let generic_args: Vec<_> = type_path
                .path
                .segments
                .iter()
                .filter_map(|segment| match &segment.arguments {
                    PathArguments::AngleBracketed(args) => Some(args),
                    _ => None,
                })
                .flat_map(|args| args.args.iter())
                .collect();

            if generic_args.is_empty() {
                quote! { #path_without_args }
            } else {
                quote! { #path_without_args::<#(#generic_args),*> }
            }
        }
        _ => panic!("unsupported type..."),
    }
}
