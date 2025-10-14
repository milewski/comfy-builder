use crate::types::IntoDict;
use crate::{ComfyDataTypes, ToComfyType};

impl<'py> ToComfyType<'py> for String {
    fn comfy_type() -> ComfyDataTypes {
        ComfyDataTypes::String
    }
}

impl<'py> IntoDict<'py> for String {}
