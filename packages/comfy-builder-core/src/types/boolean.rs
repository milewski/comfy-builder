use crate::types::{IntoDict};
use crate::{ComfyDataTypes, ToComfyType};

impl<'py> IntoDict<'py> for bool {}

impl<'py> ToComfyType<'py> for bool {
    fn comfy_type() -> ComfyDataTypes {
        ComfyDataTypes::Boolean
    }
}
