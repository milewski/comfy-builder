use crate::types::comfy_type::{ComfyType, AsInput};

impl<'py> AsInput<'py> for String {
    fn comfy_type() -> ComfyType {
        ComfyType::String
    }
}
