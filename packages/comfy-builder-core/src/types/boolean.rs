use crate::types::IntoDict;
use crate::types::comfy_type::{ComfyType, ToComfyType};

impl<'py> IntoDict<'py> for bool {}

impl<'py> ToComfyType<'py> for bool {
    fn comfy_type() -> ComfyType {
        ComfyType::Boolean
    }
}
