use crate::types::IntoDict;
use crate::types::comfy_type::{ComfyType, ToComfyType};

impl<'py> ToComfyType<'py> for String {
    fn comfy_type() -> ComfyType {
        ComfyType::String
    }
}

impl<'py> IntoDict<'py> for String {}
