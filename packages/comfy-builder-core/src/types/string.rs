use crate::types::comfy_type::{AsInput, ComfyType};

impl<'py> AsInput<'py> for String {
    fn comfy_type() -> ComfyType {
        ComfyType::String
    }
}
