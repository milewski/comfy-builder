use crate::types::IntoDict;
use crate::{ComfyDataTypes, ComfyInput, ToComfyType};

impl ToComfyType for String {
    fn comfy_type() -> ComfyDataTypes {
        ComfyDataTypes::String
    }
}

impl<'py> IntoDict<'py> for String {}