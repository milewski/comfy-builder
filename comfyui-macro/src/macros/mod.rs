pub mod r#enum;
pub mod input;
pub mod output;
pub mod root;

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
