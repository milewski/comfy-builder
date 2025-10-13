use std::any::type_name;
use num_traits::Num;
use crate::{ComfyDataTypes, ToComfyType};
use crate::types::{ComfyNativeType, IntoDict};

impl<'py> ComfyNativeType<'py> for std::string::String {}

impl<'py> IntoDict<'py> for std::string::String {
    fn to_native_type() -> ComfyDataTypes {
        ComfyDataTypes::String
    }
}

impl ToComfyType for String {
    fn comfy_type() -> ComfyDataTypes {
        ComfyDataTypes::String
    }
}
