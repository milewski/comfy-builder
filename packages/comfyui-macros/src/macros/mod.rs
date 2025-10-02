pub mod r#enum;
pub mod input;
pub mod output;
pub mod node;
pub mod register;

macro_rules! numeric_types {
    (signed) => {
        "i8" | "i16" | "i32" | "i128" | "i64" | "isize"
    };
    (unsigned) => {
        "u8" | "u16" | "u32" | "u128" | "u64" | "usize"
    };
    () => {
        numeric_types!(unsigned) | numeric_types!(signed)
    };
}

use numeric_types;
use proc_macro2::Ident;
use syn::{GenericArgument, PathArguments, Type};

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